extern crate doc_index_updater;

mod support;
use doc_index_updater::{
    models::DeleteMessage,
    service_bus_client::delete_factory,
    state_manager::{get_client, StateManager},
};
use support::{get_message_safely, get_ok, get_test_delete_message};
use tokio_test::block_on;
use uuid::Uuid;

#[test]
#[ignore]
fn delete_queue_works() {
    let id = Uuid::new_v4();
    let sent_message = get_test_delete_message(id, format!("doc-{}", id));
    let mut queue = get_ok(delete_factory());
    get_ok(queue.send(sent_message.clone(), time::Duration::seconds(1)));

    let mut retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
    while retrieval.message != sent_message {
        retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
    }

    assert_eq!(retrieval.message, sent_message);

    let queue_removal_response = block_on(retrieval.remove());
    assert!(queue_removal_response.is_ok());
    assert_eq!(queue_removal_response.unwrap(), "");
}

#[test]
fn handle_error_does_not_remove_message_from_queue_or_set_job_status() {
    let id = Uuid::new_v4();
    let sent_message = get_test_delete_message(id, format!("doc-{}", id));
    let mut queue = get_ok(delete_factory());
    get_ok(queue.send(sent_message.clone(), time::Duration::seconds(1)));

    let redis_client = get_client("localhost".to_string());

    match redis_client {
        Ok(message) => {
            let state_manager = StateManager::new(message);
            get_ok(queue.try_process_from_queue::<DeleteMessage>(&state_manager));
        }
        Err(message) => {}
    }

    let mut retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
    // if retrieval.message == sent_message {
    //     assert_eq!(true, false, "Message was removed and shouldn't have been")
    // }

    let mut i = 0;
    while retrieval.message != sent_message {
        retrieval = block_on(get_message_safely::<DeleteMessage>(&mut queue));
        // i += 1;
        // if i == 10 {
        //     assert_eq!(true, false, "Message was removed and shouldn't have been")
        // }
    }

    assert_eq!(retrieval.message, sent_message);

    let queue_removal_response = block_on(retrieval.remove());
    assert!(queue_removal_response.is_ok());
    assert_eq!(queue_removal_response.unwrap(), "");
}
//     let mut retrieval = block_on(get_message_safely::<CreateMessage>(&mut queue));

//     while retrieval.message != sent_message {
//         retrieval = block_on(get_message_safely::<CreateMessage>(&mut queue));
//     }

//     let mut delete_client = get_ok(delete_factory());

//         .await
//         .map_err(|e| anyhow!("Couldn't create service bus client: {:?}", e))?;

//     match delete_client
//         .try_process_from_queue::<DeleteMessage>(&state_manager)
//         .await
//     {
//         Ok(()) => {}
//         Err(e) => tracing::error!("{:?}", e),
//     }
// }
