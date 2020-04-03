use crate::models::FileSource;
use ssh2::{Channel, Session};
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

async fn sentinel_session_factory() -> Result<Session, SentinelSftpError> {
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

    assert!(ssh_session.authenticated());

    Ok(ssh_session)
}

pub async fn get_env_fail_fast(name: &str) -> String {
    let failure_message = format!("Set env variable {} first!", name);
    std::env::var(name).expect(&failure_message)
}

async fn retrieve_scp_channel(
    session: Session,
    filepath: String,
) -> Result<Channel, anyhow::Error> {
    let path = std::path::Path::new(&filepath);
    let path = match path.strip_prefix("/") {
        Ok(p) => p,
        Err(_) => path,
    };
    tracing::info!("{:?}", path);
    let channel_and_stats_result = session.scp_recv(path).map_err(|e| {
        tracing::error!("{:?}", e);
        e
    })?;

    Ok(channel_and_stats_result.0)
}

pub async fn retrieve(source: FileSource, filepath: String) -> Result<Vec<u8>, anyhow::Error> {
    let sentinel_session = match source {
        FileSource::Sentinel => sentinel_session_factory().await?,
    };
    let mut channel = retrieve_scp_channel(sentinel_session, filepath.clone()).await?;
    let mut bytes = Vec::<u8>::new();
    let size = channel.read_to_end(&mut bytes)?;
    tracing::info!("File retrieved using SCP at {} ({} bytes) ", filepath, size);
    Ok(bytes)
}
