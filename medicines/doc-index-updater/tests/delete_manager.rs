extern crate doc_index_updater;

use doc_index_updater::{delete_manager, service_bus_client::delete_factory};
use tokio_test::block_on;

#[test]
#[ignore]
fn delete_manager_works() {
    let mut delete_client = block_on(delete_factory()).unwrap();
    let a = block_on(delete_manager::get_message(&mut delete_client)).unwrap();

    assert_eq!(a, "free");
}
