use ssh2::{Error, File, Session, Sftp};
use std::{io::Read, net::TcpStream};

pub async fn sftp_factory() -> Result<Sftp, Error> {
    let server = get_env_fail_fast("SFTP_SERVER").await;
    let user = get_env_fail_fast("SFTP_USERNAME").await;
    let password = get_env_fail_fast("SFTP_PASSWORD").await;

    let tcp = TcpStream::connect(format!("{}:22", server)).unwrap();
    let mut ssh_session = Session::new()?;
    ssh_session.set_tcp_stream(tcp);
    ssh_session.handshake()?;

    let _ = ssh_session.userauth_password(&user, &password)?;
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

pub async fn retrieve(filepath: String) -> Result<String, anyhow::Error> {
    let mut sftp_client = sftp_factory().await?;
    let mut file = retrieve_file_from_sftp(&mut sftp_client, filepath).await?;
    let mut some_string = "".to_owned();
    let _ = file.read_to_string(&mut some_string);
    tracing::info!("{:?}", some_string);
    Ok(some_string)
}
