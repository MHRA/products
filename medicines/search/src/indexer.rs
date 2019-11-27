use awc::Client;
use actix_rt::System;
use mime;
use std::fs;

pub fn create_indexer() {
    let indexer_definition = get_indexer_definition();

    System::new("create_indexer").block_on(async {
        let mut client = Client::default();

        client
            .post("https://rb-mhra-mip.search.windows.net/indexers?api-version=2019-05-06")
            .set(awc::http::header::ContentType(mime::APPLICATION_JSON))
            .header("api-key", "323AC9DC56CB64CE71C33C4A28C832A4")
            .send_body(indexer_definition)
            .await
            .and_then(|response| {
                println!("Response: {:?}", response);
                Ok(())
            })
    });
}

fn get_indexer_definition() -> String {
    let indexer_definition_path = "../definitions/indexers/azureblob-indexer.json";

    fs::read_to_string(indexer_definition_path)
        .expect("Could not read indexer definition.")
}