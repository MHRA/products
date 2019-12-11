use actix_rt::System;
use actix_web::client::{Client, SendRequestError};
use std::time::Duration;

pub fn make_post_request_with_body(definition: String, url: &str, api_key: &str) -> Result<(), SendRequestError> {
    System::new("azure_rest").block_on(async {
        let client = Client::default();

        let response = client
            .post(url)
            .set(actix_web::http::header::ContentType(mime::APPLICATION_JSON))
            .header("api-key", api_key)
            .timeout(Duration::new(3600, 0))
            .send_body(definition)
            .await?;

        println!("{:#?}", response);
        Ok(())
    })
}

pub fn make_post_request(url: &str, api_key: &str) -> Result<(), SendRequestError> {
    System::new("azure_rest").block_on(async {
        let client = Client::default();

        let response = client
            .post(url)
            .set(actix_web::http::header::ContentType(mime::APPLICATION_JSON))
            .header("api-key", api_key)
            .timeout(Duration::new(3600, 0))
            .send()
            .await?;

        println!("{:#?}", response);
        Ok(())
    })
}

pub fn make_delete_request(url: &str, api_key: &str) -> Result<(), SendRequestError> {
    System::new("azure_rest").block_on(async {
        let client = Client::default();

        let response = client
            .delete(url)
            .set(actix_web::http::header::ContentType(mime::APPLICATION_JSON))
            .header("api-key", api_key)
            .timeout(Duration::new(3600, 0))
            .send()
            .await?;

        println!("{:#?}", response);
        Ok(())
    })
}
