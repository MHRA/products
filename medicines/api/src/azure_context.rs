use search_client::AzureSearchClient;

pub struct AzureContext {
    pub products_client: AzureSearchClient,
    pub bmgf_client: AzureSearchClient,
}

pub fn create_context(products_index: String, bmgf_index: String) -> AzureContext {
    let products_client = AzureSearchClient::new_with_index(products_index);
    let bmgf_client = AzureSearchClient::new_with_index(bmgf_index);
    AzureContext {
        products_client,
        bmgf_client,
    }
}
