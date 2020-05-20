use super::{
    models::{BlobResponse, SftpError, StorageClientError},
    GetBlob,
};
use anyhow::anyhow;
use async_ssh2::{Session, Sftp};
use async_trait::async_trait;
use std::net::TcpStream;
use tokio::io::AsyncReadExt;

struct SftpConfig {
    server: String,
    user: String,
    public_key_path: String,
    private_key_path: String,
    private_key_password: String,
}

async fn sentinel_sftp_factory(
    SftpConfig {
        server,
        user,
        public_key_path,
        private_key_path,
        private_key_password,
    }: &SftpConfig,
) -> Result<Sftp, SftpError> {
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
        ssh_session.sftp().await.map_err(Into::into)
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
    sftp: &mut Sftp,
    filepath: &str,
) -> Result<async_ssh2::File, StorageClientError> {
    let path = std::path::Path::new(filepath);

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

    Ok(sftp.open(path).await.map_err(|e| {
        tracing::error!("{:?}", e);
        match e {
            async_ssh2::Error::SSH2(e) => match e.code() {
                -31 => SftpError::CouldNotRetrieveFile,
                _ => SftpError::Ssh2Error(e.into()),
            },
            _ => SftpError::Ssh2Error(e),
        }
    })?)
}

pub struct SftpClient {
    sftp_info: SftpConfig,
}

impl SftpClient {
    pub async fn sentinel() -> Self {
        let server = get_env_fail_fast("SENTINEL_SFTP_SERVER").await;
        let user = get_env_fail_fast("SENTINEL_SFTP_USERNAME").await;
        let public_key_path = get_env_fail_fast("SENTINEL_PUBLIC_KEY_PATH").await;
        let private_key_path = get_env_fail_fast("SENTINEL_PRIVATE_KEY_PATH").await;
        let private_key_password = get_env_fail_fast("SENTINEL_PRIVATE_KEY_PASSWORD").await;

        Self {
            sftp_info: SftpConfig {
                server,
                user,
                public_key_path,
                private_key_path,
                private_key_password,
            },
        }
    }

    async fn get_sftp_connection(&self) -> Result<Sftp, SftpError> {
        Ok(sentinel_sftp_factory(&self.sftp_info).await?)
    }
}

#[async_trait]
impl GetBlob for SftpClient {
    async fn get_blob(&self, blob_name: &str) -> Result<BlobResponse, StorageClientError> {
        let mut file =
            retrieve_file_from_sftp(&mut self.get_sftp_connection().await?, blob_name).await?;
        let mut bytes = Vec::<u8>::new();
        let size = file.read_to_end(&mut bytes).await.map_err(|e| anyhow!(e))?;
        tracing::debug!(
            "File retrieved from SFTP at {} ({} bytes) ",
            blob_name,
            size
        );
        Ok(BlobResponse {
            blob_name: blob_name.to_owned(),
            data: bytes,
        })
    }
}
