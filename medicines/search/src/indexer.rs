use actix_rt::System;
use actix_web::{client, client::Client};
use mime;

pub fn create_indexer() -> Result<(), client::SendRequestError> {
    let indexer_definition = get_indexer_definition();

    System::new("create_indexer").block_on(async {
        let client = Client::default();

        let request = client
            .post("https://rb-mhra-mip.search.windows.net/indexers?api-version=2019-05-06")
            .set(actix_web::http::header::ContentType(mime::APPLICATION_JSON))
            .header("api-key", "323AC9DC56CB64CE71C33C4A28C832A4");
        println!("{:?}\n{:?}", request, indexer_definition);
        request
            .send_body(indexer_definition)
            .await
            .and_then(|response| {
                println!("Response: {:?}", response);
                Ok(())
            })
    })
}

fn get_indexer_definition() -> String {
    include_str!("../definitions/indexers/azureblob-indexer.json").to_string()
}
