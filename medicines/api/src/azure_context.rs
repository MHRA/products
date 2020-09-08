use search_client::AzureSearchClient;

pub struct AzureContext {
    pub client: AzureSearchClient,
}

pub fn create_context() -> AzureContext {
    let client = AzureSearchClient::new();

    AzureContext { client }
}
