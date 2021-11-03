use reqwest::Url;

pub async fn make_post_request_with_body(
    definition: String,
    url: &str,
    api_key: &str,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let _response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("api-key", api_key)
        .body(definition)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub async fn make_put_request_with_body(
    definition: String,
    url: Url,
    api_key: &str,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let _response = client
        .put(url)
        .header("Content-Type", "application/json")
        .header("api-key", api_key)
        .body(definition)
        .send()
        .await?;

    println!("{:#?}", _response.text().await?);

    Ok(())
}

pub async fn make_post_request(url: &str, api_key: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let _response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("api-key", api_key)
        .header("Content-Length", 0)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub async fn make_delete_request(url: &str, api_key: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let _response = client
        .delete(url)
        .header("Content-Type", "application/json")
        .header("api-key", api_key)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
