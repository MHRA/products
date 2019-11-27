use awc::Client;
use actix_rt::System;
use mime;
use std::fs;

pub fn create_index() {
    let index_definition = get_index_definition();

    System::new("create_index").block_on(async {
        let mut client = Client::default();

        client
            .post("https://rb-mhra-mip.search.windows.net/indexes?api-version=2019-05-06")
            .set(awc::http::header::ContentType(mime::APPLICATION_JSON))
            .header("api-key", "323AC9DC56CB64CE71C33C4A28C832A4")
            .send_body(index_definition)
            .await
            .and_then(|response| {
                println!("Response: {:?}", response);
                Ok(())
            })
    });
}

fn get_index_definition() -> String {
    let index_definition_path = "../definitions/indexes/azureblob-index.json";

    fs::read_to_string(index_definition_path)
        .expect("Could not read index definition.")
}