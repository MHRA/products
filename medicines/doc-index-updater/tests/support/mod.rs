#![allow(dead_code)]
use core::{fmt::Debug, future::Future};
use doc_index_updater::{
    models::{CreateMessage, DeleteMessage, Document, FileSource, Message},
    service_bus_client::{DocIndexUpdaterQueue, RetrieveFromQueueError, RetrievedMessage},
};
use redis::{self, Value};
use search_client::models::DocumentType;
use std::{fs, io, process, thread::sleep, time::Duration};
use tokio_test::block_on;
use uuid::Uuid;

pub mod document_api;

#[derive(PartialEq)]
enum ServerType {
    Tcp,
    Unix,
}

pub struct RedisServer {
    pub process: process::Child,
    addr: redis::ConnectionAddr,
}

impl Default for RedisServer {
    fn default() -> Self {
        let listener = net2::TcpBuilder::new_v4()
            .unwrap()
            .reuse_address(true)
            .unwrap()
            .bind("127.0.0.1:0")
            .unwrap()
            .listen(1)
            .unwrap();

        let server_port = dbg!(listener.local_addr().unwrap().port());

        let addr = redis::ConnectionAddr::Tcp("127.0.0.1".to_string(), server_port);

        RedisServer::new_with_addr(addr, |cmd| {
            cmd.spawn()
                .expect("Could not find redis, try installing redis server.")
        })
    }
}

impl RedisServer {
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
            _ => unimplemented!(),
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

impl Default for TestContext {
    fn default() -> Self {
        let server = RedisServer::default();

        let client = redis::Client::open(redis::ConnectionInfo {
            addr: Box::new(server.get_client_addr().clone()),
            db: 0,
            passwd: None,
        })
        .unwrap();
        let mut con;

        let try_connection_after = Duration::from_secs(1);
        sleep(try_connection_after);
        loop {
            match client.get_connection() {
                Err(err) => {
                    if !err.is_connection_refusal() {
                        panic!("Could not connect: {}", err);
                    }
                    sleep(try_connection_after);
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
}

impl TestContext {
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

pub fn get_ok<T, U>(spawn: impl Future<Output = Result<T, U>>) -> T
where
    U: Debug,
{
    block_on(spawn).unwrap()
}

pub fn get_test_document() -> Document {
    Document {
        id: "id".to_string(),
        name: "name".to_string(),
        document_type: DocumentType::Pil,
        author: "author".to_string(),
        products: vec!["products".to_string()],
        keywords: Some(vec!["keywords".to_string()]),
        pl_number: "pl_number".to_string(),
        active_substances: vec!["active_substances".to_string()],
        file_source: FileSource::Sentinel,
        file_path: "file_path".to_string(),
    }
}

pub fn get_test_create_message(id: Uuid) -> CreateMessage {
    CreateMessage {
        job_id: id,
        document: get_test_document(),
        initiator_email: None,
    }
}

pub fn get_test_delete_message(job_id: Uuid, document_content_id: String) -> DeleteMessage {
    DeleteMessage {
        job_id,
        document_id: document_content_id.into(),
        initiator_email: None,
    }
}

pub async fn get_message_safely<T: Message>(
    queue: &mut DocIndexUpdaterQueue,
) -> RetrievedMessage<T> {
    // This ensures test messages
    // which aren't deserializable
    // don't panic the entire test
    loop {
        match queue.receive::<T>().await {
            Ok(a) => return a,
            Err(RetrieveFromQueueError::ParseError(_)) => continue,
            Err(RetrieveFromQueueError::NotFoundError) => continue,
            Err(e) => {
                panic!("bad error: {:?}", e);
            }
        }
    }
}

pub fn repeatedly_check_until_result_is<T>(
    expected: T,
    mut perform_check: impl FnMut() -> T,
    max_attempts: u8,
) where
    T: PartialEq + std::fmt::Debug,
{
    let mut i = 0;
    loop {
        let result = perform_check();
        if result == expected {
            break;
        } else if i > max_attempts {
            panic!(
                "[{:?}] wasn't [{:?}] after {} seconds.",
                result, expected, max_attempts
            );
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        i += 1;
    }
}
