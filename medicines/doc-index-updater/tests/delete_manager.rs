extern crate doc_index_updater;

mod support;
use doc_index_updater::{
    models::DeleteMessage,
    service_bus_client::{delete_factory, DocIndexUpdaterQueue, RetrieveFromQueueError},
};
use support::{get_ok, get_test_delete_message};
use tokio_test::block_on;
use uuid::Uuid;

#[test]
#[ignore]
fn delete_queue_works() {
    let id = Uuid::new_v4();
    let sent_message = get_test_delete_message(id, format!("doc-{}", id));
    let mut queue = get_ok(delete_factory());
    get_ok(queue.send(sent_message.clone(), time::Duration::seconds(1)));

    let mut received_message = block_on(get_message_safely(&mut queue));
    while received_message != sent_message {
        received_message = block_on(get_message_safely(&mut queue));
    }

    assert_eq!(received_message, sent_message);
}

async fn get_message_safely(queue: &mut DocIndexUpdaterQueue) -> DeleteMessage {
    // This ensures test messages
    // which aren't deserializable
    // don't panic the entire test
    loop {
        match queue.receive::<DeleteMessage>().await {
            Ok(a) => return a,
            Err(RetrieveFromQueueError::ParseError(_)) => continue,
            Err(RetrieveFromQueueError::NotFoundError) => continue,
            Err(e) => {
                panic!("bad error: {:?}", e);
            }
        }
    }
}
