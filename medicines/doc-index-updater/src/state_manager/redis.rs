use log::info;
use redis::{self, Commands, FromRedisValue, RedisError, RedisWrite, ToRedisArgs, Value};
use uuid::Uuid;
use warp::reject;

use super::models::JobStatus;

#[derive(Debug)]
pub struct MyRedisError(RedisError);

impl reject::Reject for MyRedisError {}

impl From<RedisError> for MyRedisError {
    fn from(t: RedisError) -> Self {
        Self(t)
    }
}

impl FromRedisValue for JobStatus {
    fn from_redis_value(t: &Value) -> Result<JobStatus, RedisError> {
        let p = String::from_redis_value(t)?;
        p.parse().map_err(to_redis_error)
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
        let s = match self {
            JobStatus::Accepted => "Accepted",
            JobStatus::Done => "Done",
            _ => "No idea, buddy",
        };
        info!("{:#}", s);
        out.write_arg(s.as_bytes());
    }
}

pub fn get_from_redis(id: Uuid) -> redis::RedisResult<JobStatus> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    con.get(id.to_string()).or(Ok(JobStatus::NotFound))
}

pub fn set_in_redis(id: Uuid, status: JobStatus) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    con.set(id.to_string(), status)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(&Value::Status("Accepted".to_string()), Ok(JobStatus::Accepted))]
    #[test_case(&Value::Status("Done".to_string()), Ok(JobStatus::Done))]
    #[test_case(&Value::Status("Error".to_string()), Ok(JobStatus::Error {message:"Error status".to_owned(), code:"0x0".to_owned()}))]
    #[test_case(&Value::Status("Bedro".to_string()), Err(to_redis_error( "Status unknown: Bedro".to_string())))]
    fn from_redis_value(input: &Value, output: Result<JobStatus, RedisError>) {
        assert_eq!(JobStatus::from_redis_value(input), output);
    }
}
