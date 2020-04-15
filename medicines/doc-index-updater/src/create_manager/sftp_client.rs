use crate::models::FileSource;
use anyhow::anyhow;
use async_ssh2::{Session, Sftp};
use fehler::{throw, throws};
use std::net::TcpStream;
use thiserror::Error;
use tokio::io::AsyncReadExt;

#[derive(Error, Debug)]
pub enum SftpError {
    #[error("A TCP error connecting to server. ({0:?})")]
    TcpError(#[from] std::io::Error),
    #[error("An SSH error connecting to server. ({0:?})")]
    Ssh2Error(#[from] async_ssh2::Error),
    #[error("File could not be retrieved on server")]
    CouldNotRetrieveFile,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[throws(SftpError)]
async fn sentinel_sftp_factory() -> Sftp {
    let server = get_env_fail_fast("SENTINEL_SFTP_SERVER").await;
    let user = get_env_fail_fast("SENTINEL_SFTP_USERNAME").await;
    let public_key_path = get_env_fail_fast("SENTINEL_PUBLIC_KEY_PATH").await;
    let private_key_path = get_env_fail_fast("SENTINEL_PRIVATE_KEY_PATH").await;
    let private_key_password = get_env_fail_fast("SENTINEL_PRIVATE_KEY_PASSWORD").await;

    tracing::debug!(
        message = format!(
            "Initiating Sentinel sftp connection with server: {} with user: {}",
            server, user
        )
        .as_str()
    );
    let tcp = TcpStream::connect(format!("{}:22", server))?;

    tracing::debug!(message = "SFTP server connection established");

    let mut ssh_session = Session::new()?;
    ssh_session.set_tcp_stream(tcp)?;
    ssh_session.handshake().await?;

    tracing::debug!(message = "SFTP server handshake complete");

    tracing::debug!(message = "Trying authentication");

    let public_key_path = std::path::Path::new(&public_key_path);
    let private_key_path = std::path::Path::new(&private_key_path);
    ssh_session
        .userauth_pubkey_file(
            &user,
            Some(&public_key_path),
            &private_key_path,
            Some(&private_key_password),
        )
        .await?;
    tracing::debug!(message = "Finished trying authentication");

    if ssh_session.authenticated() {
        tracing::debug!(message = "SFTP session authenticated");
        ssh_session.sftp().await?
    } else {
        let message = "SFTP session authentication failed";
        tracing::debug!(message);
        throw!(SftpError::Other(anyhow!(message)))
    }
}

pub async fn get_env_fail_fast(name: &str) -> String {
    let failure_message = format!("Set env variable {} first!", name);
    std::env::var(name).expect(&failure_message)
}

#[throws(anyhow::Error)]
async fn retrieve_file_from_sftp(sftp: &mut Sftp, filepath: String) -> async_ssh2::File {
    let path = std::path::Path::new(&filepath);

    tracing::info!("{:?}", path);

    // Additional logging to debug observed sftp issue
    // in nonprod environment
    let parent_dir = path.parent();
    if let Some(parent_dir_path) = parent_dir {
        tracing::debug!("Finding contents of {:?}", &parent_dir_path);
        if let Ok(file_stats) = sftp.readdir(parent_dir_path).await {
            for (path_buf, file_stat) in file_stats {
                tracing::debug!("{:?}", path_buf.to_str());
                tracing::debug!("File stats: {:#?}", file_stat);
            }
        } else {
            tracing::debug!("Couldn't find dir contents");
        }
    }

    sftp.open(path).await.map_err(|e| {
        tracing::error!("{:?}", e);
        match e {
            async_ssh2::Error::SSH2(e) => match e.code() {
                -31 => SftpError::CouldNotRetrieveFile,
                _ => SftpError::Ssh2Error(e.into()),
            },
            _ => SftpError::Ssh2Error(e),
        }
    })?
}

#[throws(SftpError)]
pub async fn retrieve(source: FileSource, filepath: String) -> Vec<u8> {
    let mut sentinel_sftp_client = match source {
        FileSource::Sentinel => sentinel_sftp_factory().await?,
    };
    let mut file = retrieve_file_from_sftp(&mut sentinel_sftp_client, filepath.clone()).await?;
    let mut bytes = Vec::<u8>::new();
    let size = file.read_to_end(&mut bytes).await?;
    tracing::info!("File retrieved from SFTP at {} ({} bytes) ", filepath, size);
    bytes
}
