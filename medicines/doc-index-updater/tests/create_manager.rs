extern crate doc_index_updater;

mod support;
use doc_index_updater::{models::CreateMessage, service_bus_client::create_factory};
use support::{get_message_safely, get_ok, get_test_create_message};
use tokio_test::block_on;
use uuid::Uuid;

#[test]
#[ignore]
fn create_queue_works() {
    let id = Uuid::new_v4();
    let sent_message = get_test_create_message(id);
    let mut queue = get_ok(create_factory());
    get_ok(queue.send(sent_message.clone(), time::Duration::seconds(30)));

    let mut retrieval = block_on(get_message_safely::<CreateMessage>(&mut queue));
    while retrieval.message != sent_message {
        retrieval = block_on(get_message_safely::<CreateMessage>(&mut queue));
    }

    assert_eq!(retrieval.message, sent_message);

    let queue_removal_response = block_on(retrieval.peek_lock.delete_message());
    assert!(queue_removal_response.is_ok());
    assert_eq!(queue_removal_response.unwrap(), "");
}
