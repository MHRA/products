#![allow(dead_code)]
use redis::{self, Value};
use std::{fs, io, process, thread::sleep, time::Duration};

#[derive(PartialEq)]
enum ServerType {
    Tcp,
    Unix,
}

pub struct RedisServer {
    pub process: process::Child,
    addr: redis::ConnectionAddr,
}

impl RedisServer {
    pub fn new() -> RedisServer {
        let addr = redis::ConnectionAddr::Tcp("127.0.0.1".to_string(), 6379);
        RedisServer::new_with_addr(addr, |cmd| cmd.spawn().unwrap())
    }

    pub fn new_with_addr<F: FnOnce(&mut process::Command) -> process::Child>(
        addr: redis::ConnectionAddr,
        spawner: F,
    ) -> RedisServer {
        let mut cmd = process::Command::new("redis-server");
        cmd.stdout(process::Stdio::null())
            .stderr(process::Stdio::null());

        match addr {
            redis::ConnectionAddr::Tcp(ref bind, server_port) => {
                cmd.arg("--port")
                    .arg(server_port.to_string())
                    .arg("--bind")
                    .arg(bind);
            }
            redis::ConnectionAddr::Unix(ref path) => {
                cmd.arg("--port").arg("0").arg("--unixsocket").arg(&path);
            }
        };

        RedisServer {
            process: spawner(&mut cmd),
            addr,
        }
    }

    pub fn wait(&mut self) {
        self.process.wait().unwrap();
    }

    pub fn get_client_addr(&self) -> &redis::ConnectionAddr {
        &self.addr
    }

    pub fn stop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
        if let redis::ConnectionAddr::Unix(ref path) = *self.get_client_addr() {
            fs::remove_file(&path).ok();
        }
    }
}

impl Drop for RedisServer {
    fn drop(&mut self) {
        self.stop()
    }
}

pub struct TestContext {
    pub server: RedisServer,
    pub client: redis::Client,
}

impl TestContext {
    pub fn new() -> TestContext {
        let server = RedisServer::new();

        let client = redis::Client::open(redis::ConnectionInfo {
            addr: Box::new(server.get_client_addr().clone()),
            db: 0,
            passwd: None,
        })
        .unwrap();
        let mut con;

        let millisecond = Duration::from_millis(1);
        loop {
            match client.get_connection() {
                Err(err) => {
                    if err.is_connection_refusal() {
                        sleep(millisecond);
                    } else {
                        panic!("Could not connect: {}", err);
                    }
                }
                Ok(x) => {
                    con = x;
                    break;
                }
            }
        }
        redis::cmd("FLUSHDB").execute(&mut con);

        TestContext { server, client }
    }

    pub fn connection(&self) -> redis::Connection {
        self.client.get_connection().unwrap()
    }

    pub async fn async_connection(&self) -> redis::RedisResult<redis::aio::Connection> {
        self.client.get_async_connection().await
    }

    pub fn stop_server(&mut self) {
        self.server.stop();
    }
}

pub fn encode_value<W>(value: &Value, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    #![allow(clippy::write_with_newline)]
    match *value {
        Value::Nil => write!(writer, "$-1\r\n"),
        Value::Int(val) => write!(writer, ":{}\r\n", val),
        Value::Data(ref val) => {
            write!(writer, "${}\r\n", val.len())?;
            writer.write_all(val)?;
            writer.write_all(b"\r\n")
        }
        Value::Bulk(ref values) => {
            write!(writer, "*{}\r\n", values.len())?;
            for val in values.iter() {
                encode_value(val, writer)?;
            }
            Ok(())
        }
        Value::Okay => write!(writer, "+OK\r\n"),
        Value::Status(ref s) => write!(writer, "+{}\r\n", s),
    }
}
