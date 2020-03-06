extern crate doc_index_updater;

mod support;
use doc_index_updater::{models::CreateMessage, service_bus_client::create_factory};
use support::{get_ok, get_test_create_message};
use uuid::Uuid;

#[test]
#[ignore]
fn create_queue_works() {
    let id = Uuid::new_v4();
    let sent_message = get_test_create_message(id);
    let mut queue = get_ok(create_factory());
    get_ok(queue.send(sent_message.clone(), time::Duration::seconds(1)));

    let mut received_message = get_ok(queue.receive::<CreateMessage>());
    while received_message != sent_message {
        received_message = get_ok(queue.receive::<CreateMessage>());
    }

    assert_eq!(received_message, sent_message);
}
