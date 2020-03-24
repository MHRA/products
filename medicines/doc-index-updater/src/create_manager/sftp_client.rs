use crate::models::FileSource;
use ssh2::{File, Session, Sftp};
use std::{fmt::Display, io::Read, net::TcpStream};

#[derive(Debug)]
enum SentinelSftpError {
    TcpError(std::io::Error),
    Ssh2Error(ssh2::Error),
}

impl Display for SentinelSftpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SentinelSftpError::TcpError(e) => {
                write!(f, "A TCP error connecting to server. ({:?})", e)
            }
            SentinelSftpError::Ssh2Error(e) => {
                write!(f, "An SSH error connecting to server. ({:?})", e)
            }
        }
    }
}

impl std::error::Error for SentinelSftpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<ssh2::Error> for SentinelSftpError {
    fn from(e: ssh2::Error) -> Self {
        SentinelSftpError::Ssh2Error(e)
    }
}

impl From<std::io::Error> for SentinelSftpError {
    fn from(e: std::io::Error) -> Self {
        SentinelSftpError::TcpError(e)
    }
}

async fn sentinel_sftp_factory() -> Result<Sftp, SentinelSftpError> {
    let server = get_env_fail_fast("SENTINEL_SFTP_SERVER").await;
    let user = get_env_fail_fast("SENTINEL_SFTP_USERNAME").await;
    let password = get_env_fail_fast("SENTINEL_SFTP_PASSWORD").await;

    let tcp = TcpStream::connect(format!("{}:22", server))?;
    let mut ssh_session = Session::new()?;
    ssh_session.set_tcp_stream(tcp);
    ssh_session.handshake()?;

    ssh_session.userauth_password(&user, &password)?;
    assert!(ssh_session.authenticated());

    let sftp = ssh_session.sftp()?;

    Ok(sftp)
}

pub async fn get_env_fail_fast(name: &str) -> String {
    std::env::var(name).expect(&format!("Set env variable {} first!", name))
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
    Ok(sftp.open(path).map_err(|e| {
        tracing::error!("{:?}", e);
        e
    })?)
}

pub async fn retrieve(source: FileSource, filepath: String) -> Result<Vec<u8>, anyhow::Error> {
    let mut sentinel_sftp_client = match source {
        FileSource::Sentinel => sentinel_sftp_factory().await?,
    };
    let mut file = retrieve_file_from_sftp(&mut sentinel_sftp_client, filepath.clone()).await?;
    let mut bytes = Vec::<u8>::new();
    let size = file.read_to_end(&mut bytes)?;
    tracing::info!("File retrieved from SFTP at {} ({} bytes) ", filepath, size);
    Ok(bytes)
}
