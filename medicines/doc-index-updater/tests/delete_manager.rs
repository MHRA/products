extern crate doc_index_updater;

use core::{fmt::Debug, future::Future};
use doc_index_updater::{delete_manager, service_bus_client::delete_factory};
use tokio_test::block_on;

#[test]
#[ignore]
fn delete_manager_works() {
    let sent_message = "This is the test message";
    let mut delete_client = block_on(delete_factory()).unwrap();
    get_ok(delete_client.send_event(sent_message, time::Duration::seconds(1)));

    let mut received_message = get_ok(delete_manager::get_message(&mut delete_client));
    while received_message != sent_message {
        received_message = get_ok(delete_manager::get_message(&mut delete_client));
    }

    assert_eq!(received_message, sent_message);
}

fn get_ok<T, U>(spawn: impl Future<Output = Result<T, U>>) -> T
where
    U: Debug,
{
    block_on(spawn).unwrap()
}
