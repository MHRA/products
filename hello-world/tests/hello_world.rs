use async_std::prelude::*;
use async_std::task;
use std::time::Duration;

#[test]
fn hello_world() -> anyhow::Result<()> {
    task::block_on(async {
        let server =
            task::spawn(async { hello_world::create_app().listen("localhost:8080").await });

        let client = task::spawn(async {
            task::sleep(Duration::from_millis(100)).await;
            let string: String = surf::get("http://localhost:8080/hello/Me")
                .recv_string()
                .await
                .unwrap();
            assert_eq!(string, "Hello, Me\n".to_string());
            Ok(())
        });

        server.race(client).await
    })?;
    Ok(())
}
