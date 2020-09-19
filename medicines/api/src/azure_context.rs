use search_client::AzureSearchClient;

pub struct AzureContext {
    pub products_client: AzureSearchClient,
    pub bmgf_client: AzureSearchClient,
}

pub fn create_context() -> AzureContext {
    let products_client = AzureSearchClient::new();
    let bmgf_client = AzureSearchClient::new_with_index("bmgf-index".to_string());
    AzureContext {
        products_client,
        bmgf_client,
    }
}
