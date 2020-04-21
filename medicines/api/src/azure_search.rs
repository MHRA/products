use search_client::AzureSearchClient;

pub struct AzureContext {
    pub client: AzureSearchClient,
}

impl juniper::Context for AzureContext {}

pub fn create_context() -> AzureContext {
    let client = AzureSearchClient::new();

    AzureContext { client }
}
