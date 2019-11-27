use awc::Client;
use actix_rt::System;
use mime;
use std::fs;

pub fn create_datasource() {
    let storage_account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let storage_master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let datasource_definition = get_datasource_definition(&storage_account, &storage_master_key);

    System::new("create_datasource").block_on(async {
        let mut client = Client::default();

        client
            .post("https://rb-mhra-mip.search.windows.net/datasources?api-version=2019-05-06")
            .set(awc::http::header::ContentType(mime::APPLICATION_JSON))
            .header("api-key", "323AC9DC56CB64CE71C33C4A28C832A4")
            .send_body(datasource_definition)
            .await
            .and_then(|response| {
                println!("Response: {:?}", response);
                Ok(())
            })
    });
}

fn get_datasource_definition(storage_account: &String, storage_master_key: &String) -> String {
    let datasource_definition_path = "../definitions/datasources/docs.json";

    fs::read_to_string(datasource_definition_path)
        .expect("Could not read datasource definition.")
        .replace("ACCOUNT_NAME_PLACEHOLDER", &storage_account)
        .replace("ACCOUNT_KEY_PLACEHOLDER", &storage_master_key)
}