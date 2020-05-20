use futures::prelude::*;
use smol::{Task, Timer};
use std::time::Duration;

#[test]
fn hello_world() -> std::io::Result<()> {
    let server = hello_world::create_app()
        .listen("localhost:8080")
        .map(|_| ());

    let client = surf::get("http://localhost:8080/hello/Me")
        .recv_string()
        .map(|x| assert_eq!(x.unwrap(), "Hello, Me\n".to_string()));

    smol::run(async {
        Task::spawn(server).detach();
        Timer::after(Duration::from_millis(100)).await;
        Task::spawn(client).await;
        Ok(())
    })
}
