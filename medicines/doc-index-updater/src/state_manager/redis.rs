use crate::models::JobStatus;
use ::redis::Client;
use anyhow::{anyhow, Result};
use redis::{self, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs, Value};
use thiserror::Error;
use uuid::Uuid;
use warp::reject;

#[derive(Debug, Error)]
pub enum MyRedisError {
    #[error("Authentication Error ({0:?})")]
    Auth(RedisError),
    #[error("Incompatible Type (maybe NOT FOUND) ({0:?})")]
    IncompatibleType(RedisError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl reject::Reject for MyRedisError {}

impl From<RedisError> for MyRedisError {
    fn from(e: RedisError) -> Self {
        match e.kind() {
            redis::ErrorKind::AuthenticationFailed => Self::Auth(e),
            redis::ErrorKind::TypeError => Self::IncompatibleType(e),
            _ => Self::Other(e.into()),
        }
    }
}

impl From<MyRedisError> for warp::Rejection {
    fn from(e: MyRedisError) -> Self {
        tracing::error!("{:?}", e);
        warp::reject::custom(MyRedisError::Other(anyhow!("Server Error")))
    }
}

impl FromRedisValue for JobStatus {
    fn from_redis_value(t: &Value) -> Result<JobStatus, RedisError> {
        String::from_redis_value(t)?.parse().map_err(to_redis_error)
    }
}

fn to_redis_error(e: String) -> RedisError {
    RedisError::from((redis::ErrorKind::IoError, "", e))
}

impl ToRedisArgs for JobStatus {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let s = self.to_string();
        tracing::info!("{:#}", s);
        out.write_arg(s.as_bytes());
    }
}

pub fn get_client(address: String) -> Result<Client> {
    Ok(Client::open(address)?)
}

pub async fn get_from_redis(client: Client, id: Uuid) -> RedisResult<JobStatus> {
    let mut con = client.get_async_connection().await?;

    redis::cmd("GET")
        .arg(id.to_string())
        .query_async(&mut con)
        .await
}

pub async fn set_in_redis(client: Client, id: Uuid, status: JobStatus) -> RedisResult<JobStatus> {
    let mut con = client.get_async_connection().await?;

    redis::cmd("SET")
        .arg(id.to_string())
        .arg(status.clone())
        .query_async(&mut con)
        .await?;

    get_from_redis(client, id).await
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(&Value::Status("Accepted".to_string()), Ok(JobStatus::Accepted))]
    #[test_case(&Value::Status("Done".to_string()), Ok(JobStatus::Done))]
    #[test_case(&Value::Status("Error(0x0: Error status)".to_string()), Ok(JobStatus::Error {message:"Error status".to_owned(), code:"0x0".to_owned()}))]
    #[test_case(&Value::Status("Bedro".to_string()), Err(to_redis_error( "Status unknown: Bedro".to_string())))]
    fn from_redis_value(input: &Value, output: Result<JobStatus, RedisError>) {
        assert_eq!(JobStatus::from_redis_value(input), output);
    }
}
