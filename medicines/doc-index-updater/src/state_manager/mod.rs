use redis::{self, Commands, FromRedisValue, RedisError, Value};
use serde_derive::Serialize;
use std::str::FromStr;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

#[derive(Serialize)]
enum JobStatus {
    Accepted,
    Done,
    NotFound,
    Error { message: String, code: String },
}

impl FromStr for JobStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<JobStatus, ()> {
        match s {
            "Accepted" => Ok(JobStatus::Accepted),
            "Done" => Ok(JobStatus::Done),
            t => Ok(JobStatus::Error {
                message: t.to_owned(),
                code: "0x0".to_owned(),
            }),
        }
    }
}

impl FromRedisValue for JobStatus {
    fn from_redis_value(t: &Value) -> Result<JobStatus, RedisError> {
        match t {
            Value::Status(s) => Ok(JobStatus::from_str(s).unwrap()),
            _ => Ok(JobStatus::NotFound),
        }
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

fn handler(id: Uuid) -> impl Reply {
    let status = get_from_redis(id).unwrap();

    warp::reply::json(&JobStatusResponse { id, status })
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("jobs" / Uuid).map(handler)
}
