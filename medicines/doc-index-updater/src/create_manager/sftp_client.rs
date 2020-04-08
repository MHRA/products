use crate::models::FileSource;
use anyhow::anyhow;
use ssh2::{File, Session, Sftp};
use std::{io::Read, net::TcpStream};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SftpError {
    #[error("A TCP error connecting to server. ({0:?})")]
    TcpError(#[from] std::io::Error),
    #[error("An SSH error connecting to server. ({0:?})")]
    Ssh2Error(#[from] ssh2::Error),
    #[error("File could not be retrieved on server")]
    CouldNotRetrieveFile,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

async fn sentinel_sftp_factory() -> Result<Sftp, SftpError> {
    let server = get_env_fail_fast("SENTINEL_SFTP_SERVER").await;
    let user = get_env_fail_fast("SENTINEL_SFTP_USERNAME").await;
    let password = get_env_fail_fast("SENTINEL_SFTP_PASSWORD").await;

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
    ssh_session.set_tcp_stream(tcp);
    ssh_session.handshake()?;

    tracing::debug!(message = "SFTP server handshake complete");

    ssh_session.userauth_password(&user, &password)?;

    if ssh_session.authenticated() {
        tracing::debug!(message = "SFTP session authenticated");
        ssh_session.sftp().map_err(Into::into)
    } else {
        let message = "SFTP session authentication failed";
        tracing::debug!(message);
        Err(SftpError::Other(anyhow!(message)))
    }
}

pub async fn get_env_fail_fast(name: &str) -> String {
    let failure_message = format!("Set env variable {} first!", name);
    std::env::var(name).expect(&failure_message)
}

async fn retrieve_file_from_sftp(
    sftp: &mut ssh2::Sftp,
    filepath: String,
) -> Result<File, anyhow::Error> {
    let path = std::path::Path::new(&filepath);
    let path = match path.strip_prefix("/") {
        Ok(p) => p,
        Err(_) => path,
    };
    tracing::info!("{:?}", path);

    // Additional logging to debug observed sftp issue
    // in nonprod environment
    let parent_dir = path.parent();
    if let Some(parent_dir_path) = parent_dir {
        tracing::debug!("Finding contents of {:?}", &parent_dir_path);
        if let Ok(file_stats) = sftp.readdir(parent_dir_path) {
            for (path_buf, file_stat) in file_stats {
                tracing::debug!("{:?}", path_buf.to_str());
                tracing::debug!("File stats: {:#?}", file_stat);
            }
        } else {
            tracing::debug!("Couldn't find dir contents");
        }
    }

    Ok(sftp.open(path).map_err(|e| {
        tracing::error!("{:?}", e);
        match e.code() {
            -31 => SftpError::CouldNotRetrieveFile,
            _ => SftpError::Ssh2Error(e),
        }
    })?)
}

pub async fn retrieve(source: FileSource, filepath: String) -> Result<Vec<u8>, SftpError> {
    let mut sentinel_sftp_client = match source {
        FileSource::Sentinel => sentinel_sftp_factory().await?,
    };
    let mut file = retrieve_file_from_sftp(&mut sentinel_sftp_client, filepath.clone()).await?;
    let mut bytes = Vec::<u8>::new();
    let size = file.read_to_end(&mut bytes)?;
    tracing::info!("File retrieved from SFTP at {} ({} bytes) ", filepath, size);
    Ok(bytes)
}
