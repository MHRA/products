use redis::{self, Commands, FromRedisValue, RedisError, RedisWrite, ToRedisArgs, Value};
use serde_derive::Serialize;
use std::str::FromStr;
use uuid::Uuid;
use warp::{reject, Filter, Rejection, Reply};

#[derive(Serialize, Debug, PartialEq)]
enum JobStatus {
    Accepted,
    Done,
    NotFound,
    Error { message: String, code: String },
}

impl FromStr for JobStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<JobStatus, Self::Err> {
        match s {
            "Accepted" => Ok(JobStatus::Accepted),
            "Done" => Ok(JobStatus::Done),
            "Error" => Ok(JobStatus::Error {
                message: "Error status".to_owned(),
                code: "0x0".to_owned(),
            }),
            e => Err(format!("Status unknown: {}", e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("Accepted", Ok(JobStatus::Accepted))]
    #[test_case("Done", Ok(JobStatus::Done))]
    #[test_case("Error", Ok(JobStatus::Error {message:"Error status".to_owned(), code:"0x0".to_owned()}))]
    #[test_case("Bedro", Err("Status unknown: Bedro".to_owned()))]
    fn test_parse(input: &str, output: Result<JobStatus, String>) {
        assert_eq!(input.parse::<JobStatus>(), output);
    }
}

impl FromRedisValue for JobStatus {
    fn from_redis_value(t: &Value) -> Result<JobStatus, RedisError> {
        let p = String::from_redis_value(t)?;
        let status = JobStatus::from_str(&p);
        match status {
            Ok(s) => Ok(s),
            Err(_) => Ok(JobStatus::Error {
                message: "Redis Error".to_owned(),
                code: "0x0".to_owned(),
            }),
        }
    }
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
        println!("{:#}", s);
        out.write_arg(s.as_bytes());
    }
}

#[derive(Serialize)]
struct JobStatusResponse {
    id: Uuid,
    status: JobStatus,
}

fn get_from_redis(id: Uuid) -> redis::RedisResult<JobStatus> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    Ok(con.get(id.to_string()).unwrap_or(JobStatus::NotFound))
}

fn set_in_redis(id: Uuid, status: JobStatus) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    Ok(con.set(id.to_string(), status).unwrap())
}

fn handler(id: Uuid) -> impl Reply {
    let status = get_from_redis(id).unwrap();

    warp::reply::json(&JobStatusResponse { id, status })
}

#[derive(Debug)]
struct MyRedisError(RedisError);

impl reject::Reject for MyRedisError {}

impl From<RedisError> for MyRedisError {
    fn from(t: RedisError) -> Self {
        Self(t)
    }
}

fn complete_handler(id: Uuid) -> Result<impl Reply, MyRedisError> {
    set_in_redis(id, JobStatus::Done)?;
    Ok(warp::reply())
}

pub fn jobs() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid).map(handler)
}

pub fn complete() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("complete" / Uuid).map(|id| complete_handler(id).unwrap())
}
