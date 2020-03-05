extern crate doc_index_updater;

use doc_index_updater::{create_manager, service_bus_client::create_factory};
use tokio_test::block_on;

#[test]
#[ignore]
fn create_manager_works() {
    let mut create_client = block_on(create_factory()).unwrap();
    let a = block_on(create_manager::get_message(&mut create_client)).unwrap();

    assert_eq!(a, "free");
}
