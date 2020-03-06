extern crate doc_index_updater;

mod support;
use doc_index_updater::{delete_manager, service_bus_client::delete_factory};
use support::get_ok;

#[test]
#[ignore]
fn delete_manager_works() {
    let sent_message = "This is the delete test message";
    let mut delete_client = get_ok(delete_factory());
    get_ok(delete_client.send_event(sent_message, time::Duration::seconds(1)));

    let mut received_message = get_ok(delete_manager::get_message(&mut delete_client));
    while received_message != sent_message {
        received_message = get_ok(delete_manager::get_message(&mut delete_client));
    }

    assert_eq!(received_message, sent_message);
}
