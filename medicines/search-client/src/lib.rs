pub mod models;

use crate::models::{AzureIndexChangedResults, IndexEntry, IndexResults};
use async_trait::async_trait;
use core::fmt::Debug;
use serde::ser::Serialize;
use std::collections::HashMap;

#[derive(Clone)]
struct AzureConfig {
    search_service: String,
    search_index: String,
    api_key: String,
    api_version: String,
}

pub struct AzureSearchClient {
    client: reqwest::Client,
    config: AzureConfig,
}

pub struct AzurePagination {
    pub result_count: i32,
    pub offset: i32,
}

pub struct AzureFilterSet {
    pub boolean_operator: String,
    pub field_filters: Vec<AzureFieldFilter>,
}

pub struct AzureFieldFilter {
    pub field_name: String,
    pub operator: String,
    pub field_value: String,
}

impl Default for AzureSearchClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AzureSearchClient {
    // Unfortunately this method is required by the API project.
    // We can't rely on the `factory` below which returns `impl Search + ...` because we need a concrete type for the [juniper context](../../api/src/schema.rs:11).
    //
    // There might be a way around this but I think it would probably take a fair bit of effort so in the interests of time we're just having `AzureContext` depend on `AzureSearchClient` directly in api/src/azure_search.rs.
    pub fn new() -> Self {
        let api_key = get_env("AZURE_API_ADMIN_KEY");
        let search_index = get_env("AZURE_SEARCH_INDEX");
        let search_service = get_env("SEARCH_SERVICE");
        let api_version = get_env("AZURE_SEARCH_API_VERSION");

        AzureSearchClient {
            client: reqwest::Client::new(),
            config: AzureConfig {
                api_key,
                search_index,
                search_service,
                api_version,
            },
        }
    }
}

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Set env variable {} first!", key))
}

pub fn factory() -> impl Search + DeleteIndexEntry + CreateIndexEntry {
    AzureSearchClient::new()
}

#[async_trait]
pub trait Search {
    async fn search(&self, search_term: &str) -> Result<IndexResults, reqwest::Error>;

    async fn search_with_pagination(
        &self,
        search_term: &str,
        pagination: AzurePagination,
        include_count: bool,
    ) -> Result<IndexResults, reqwest::Error>;

    async fn search_with_pagination_and_filter(
        &self,
        search_term: &str,
        pagination: AzurePagination,
        include_count: bool,
        filter: AzureFilterSet,
    ) -> Result<IndexResults, reqwest::Error>;

    async fn filter_by_collection_field(
        &self,
        field_name: &str,
        field_value: &str,
    ) -> Result<IndexResults, reqwest::Error>;

    async fn filter_by_non_collection_field(
        &self,
        field_name: &str,
        field_value: &str,
    ) -> Result<IndexResults, reqwest::Error>;
}

#[async_trait]
impl Search for AzureSearchClient {
    async fn search(&self, search_term: &str) -> Result<IndexResults, reqwest::Error> {
        search(search_term, None, None, None, &self.client, &self.config).await
    }

    async fn search_with_pagination(
        &self,
        search_term: &str,
        pagination: AzurePagination,
        include_count: bool,
    ) -> Result<IndexResults, reqwest::Error> {
        search(
            search_term,
            Some(pagination),
            Some(include_count),
            None,
            &self.client,
            &self.config,
        )
        .await
    }

    async fn search_with_pagination_and_filter(
        &self,
        search_term: &str,
        pagination: AzurePagination,
        include_count: bool,
        filter: AzureFilterSet,
    ) -> Result<IndexResults, reqwest::Error> {
        search(
            search_term,
            Some(pagination),
            Some(include_count),
            Some(filter),
            &self.client,
            &self.config,
        )
        .await
    }

    async fn filter_by_collection_field(
        &self,
        field_name: &str,
        field_value: &str,
    ) -> Result<IndexResults, reqwest::Error> {
        let request = build_filter_by_collection_request(
            field_name,
            field_value,
            "eq",
            &self.client,
            &self.config,
        )?;

        self.client
            .execute(request)
            .await?
            .error_for_status()?
            .json::<IndexResults>()
            .await
    }

    async fn filter_by_non_collection_field(
        &self,
        field_name: &str,
        field_value: &str,
    ) -> Result<IndexResults, reqwest::Error> {
        let request = build_filter_by_field_request(
            field_name,
            field_value,
            "eq",
            &self.client,
            &self.config,
        )?;

        self.client
            .execute(request)
            .await?
            .error_for_status()?
            .json::<IndexResults>()
            .await
    }
}

trait AzureSearchRequestBuilder {
    fn filter(self, filter_set: AzureFilterSet) -> Self;
}

impl AzureSearchRequestBuilder for reqwest::RequestBuilder {
    fn filter(self, filter_set: AzureFilterSet) -> Self {
        if filter_set.field_filters.is_empty() {
            return self;
        }
        let filter_strings = filter_set.field_filters.into_iter().map(|filter| {
            format!(
                "{} {} '{}'",
                filter.field_name, filter.operator, filter.field_value
            )
        });
        let filter_string = format!(
            "({})",
            filter_strings
                .collect::<Vec<_>>()
                .join(format!(" {} ", filter_set.boolean_operator).as_str())
        );
        self.query(&[("$filter", filter_string)])
    }
}

fn build_search(
    search_term: &str,
    pagination: Option<AzurePagination>,
    include_count: Option<bool>,
    filter_set: Option<AzureFilterSet>,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<reqwest::Request, reqwest::Error> {
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let include_count = include_count.unwrap_or(false).to_string();

    let mut request_builder = client
        .get(&base_url)
        .query(&[
            ("api-version", config.api_version.as_str()),
            ("highlight", "content"),
            ("queryType", "full"),
            ("search", search_term),
            ("scoringProfile", "preferKeywords"),
            ("$count", &include_count),
        ])
        .header("api-key", &config.api_key);

    if let Some(filter_set) = filter_set {
        request_builder = request_builder.filter(filter_set);
    }

    match pagination {
        Some(pagination) => Ok(request_builder
            .query(&[
                ("$top", pagination.result_count.to_string()),
                ("$skip", pagination.offset.to_string()),
            ])
            .build()?),
        None => Ok(request_builder.build()?),
    }
}

fn build_filter_by_collection_request(
    field_name: &str,
    value: &str,
    operator: &str,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<reqwest::Request, reqwest::Error> {
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let filter = format!(
        "{field_name}/any(value: value {operator} '{value}')",
        field_name = field_name,
        value = value,
        operator = operator,
    );

    client
        .get(&base_url)
        .query(&[("api-version", &config.api_version), ("$filter", &filter)])
        .header("api-key", &config.api_key)
        .build()
}

fn build_filter_by_field_request(
    field_name: &str,
    value: &str,
    operator: &str,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<reqwest::Request, reqwest::Error> {
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let filter = format!(
        "{field_name} {operator} '{value}'",
        field_name = field_name,
        value = value,
        operator = operator,
    );

    client
        .get(&base_url)
        .query(&[("api-version", &config.api_version), ("$filter", &filter)])
        .header("api-key", &config.api_key)
        .build()
}

#[async_trait]
pub trait DeleteIndexEntry {
    async fn delete_index_entry(
        &self,
        key_name: &str,
        value: &str,
    ) -> Result<AzureIndexChangedResults, anyhow::Error>;
}

#[async_trait]
impl DeleteIndexEntry for AzureSearchClient {
    async fn delete_index_entry(
        &self,
        key_name: &str,
        value: &str,
    ) -> Result<AzureIndexChangedResults, anyhow::Error> {
        let mut key_values = HashMap::new();
        key_values.insert(key_name, value);
        key_values.insert("@search.action", "delete");

        update_index(key_values, &self.client, &self.config).await
    }
}

#[async_trait]
pub trait CreateIndexEntry {
    async fn create_index_entry(
        &self,
        key_values: IndexEntry,
    ) -> Result<AzureIndexChangedResults, anyhow::Error>;
}

#[async_trait]
impl CreateIndexEntry for AzureSearchClient {
    async fn create_index_entry(
        &self,
        key_values: IndexEntry,
    ) -> Result<AzureIndexChangedResults, anyhow::Error> {
        update_index(key_values, &self.client, &self.config).await
    }
}

async fn search(
    search_term: &str,
    pagination: Option<AzurePagination>,
    include_count: Option<bool>,
    filter_set: Option<AzureFilterSet>,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<IndexResults, reqwest::Error> {
    let req = build_search(
        search_term,
        pagination,
        include_count,
        filter_set,
        &client,
        &config,
    )?;

    tracing::debug!("Requesting from URL: {}", &req.url());
    client
        .execute(req)
        .await?
        .error_for_status()?
        .json::<IndexResults>()
        .await
}

async fn update_index<T>(
    key_values: T,
    client: &reqwest::Client,
    config: &AzureConfig,
) -> Result<AzureIndexChangedResults, anyhow::Error>
where
    T: Serialize + Sized + Debug,
{
    let base_url = format!(
        "https://{search_service}.search.windows.net/indexes/{search_index}/docs/index",
        search_service = config.search_service,
        search_index = config.search_index
    );

    let mut body = HashMap::new();
    body.insert("value", [key_values]);

    let req = client
        .post(&base_url)
        .query(&[("api-version", &config.api_version)])
        .header("api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;

    tracing::debug!("\nBody: {:?}", &body);
    tracing::debug!("\nRequest: {:?}", &req);
    tracing::debug!("\nRequesting from URL: {}", &req.url());

    let h = client.execute(req).await?;

    if h.status() == reqwest::StatusCode::OK {
        h.json::<AzureIndexChangedResults>()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    } else {
        let error_message = h.text().await?;
        Err(anyhow::anyhow!(error_message))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn given_we_have_a_search_client() -> reqwest::Client {
        reqwest::Client::new()
    }

    fn given_we_have_a_search_term() -> String {
        "cool beans".to_string()
    }

    fn given_we_have_a_config() -> AzureConfig {
        AzureConfig {
            api_key: "api_key".to_string(),
            search_index: "search_index".to_string(),
            search_service: "search_service".to_string(),
            api_version: "api_version".to_string(),
        }
    }

    fn when_we_build_a_search_request_without_pagination(
        client: reqwest::Client,
        search_term: String,
        config: AzureConfig,
    ) -> Result<reqwest::Request, reqwest::Error> {
        build_search(&search_term, None, None, None, &client, &config)
    }

    fn then_search_url_without_pagination_is_as_expected(
        actual_result: Result<reqwest::Request, reqwest::Error>,
    ) {
        if let Ok(actual) = actual_result {
            let actual = actual.url().to_string();
            let expected = "https://search_service.search.windows.net/indexes/search_index/docs?api-version=api_version&highlight=content&queryType=full&search=cool+beans&scoringProfile=preferKeywords&%24count=false"
                .to_string();

            assert_eq!(actual, expected);
        } else {
            assert!(false, "Provided search request is an error");
        }
    }

    #[test]
    fn test_build_search_without_pagination() {
        let client = given_we_have_a_search_client();
        let search_term = given_we_have_a_search_term();
        let config = given_we_have_a_config();
        let actual = when_we_build_a_search_request_without_pagination(client, search_term, config);
        then_search_url_without_pagination_is_as_expected(actual);
    }

    fn when_we_build_a_search_request_with_pagination(
        client: reqwest::Client,
        search_term: String,
        config: AzureConfig,
    ) -> Result<reqwest::Request, reqwest::Error> {
        build_search(
            &search_term,
            Some(AzurePagination {
                result_count: 10,
                offset: 50,
            }),
            Some(true),
            None,
            &client,
            &config,
        )
    }

    fn when_we_build_a_search_request_with_pagination_and_filter(
        client: reqwest::Client,
        search_term: String,
        config: AzureConfig,
    ) -> Result<reqwest::Request, reqwest::Error> {
        build_search(
            &search_term,
            Some(AzurePagination {
                result_count: 10,
                offset: 50,
            }),
            Some(true),
            Some(AzureFilterSet {
                boolean_operator: "xor".to_string(),
                field_filters: vec![
                    AzureFieldFilter {
                        field_name: "my_cool_field".to_string(),
                        operator: "eq".to_string(),
                        field_value: "my cool value".to_string(),
                    },
                    AzureFieldFilter {
                        field_name: "my_cool_field".to_string(),
                        operator: "ne".to_string(),
                        field_value: "my uncool value".to_string(),
                    },
                ],
            }),
            &client,
            &config,
        )
    }

    fn then_search_url_with_pagination_is_as_expected(
        actual_result: Result<reqwest::Request, reqwest::Error>,
    ) {
        if let Ok(actual) = actual_result {
            let actual = actual.url().to_string();
            let expected = "https://search_service.search.windows.net/indexes/search_index/docs?api-version=api_version&highlight=content&queryType=full&search=cool+beans&scoringProfile=preferKeywords&%24count=true&%24top=10&%24skip=50"
                .to_string();

            assert_eq!(actual, expected);
        } else {
            assert!(false, "Provided search request is an error");
        }
    }

    fn then_search_url_with_pagination_and_filter_is_as_expected(
        actual_result: Result<reqwest::Request, reqwest::Error>,
    ) {
        if let Ok(actual) = actual_result {
            let actual = actual.url().to_string();
            let expected = "https://search_service.search.windows.net/indexes/search_index/docs?api-version=api_version&highlight=content&queryType=full&search=cool+beans&scoringProfile=preferKeywords&%24count=true&%24filter=%28my_cool_field+eq+%27my+cool+value%27+xor+my_cool_field+ne+%27my+uncool+value%27%29&%24top=10&%24skip=50"
                .to_string();

            assert_eq!(actual, expected);
        } else {
            assert!(false, "Provided search request is an error");
        }
    }

    #[test]
    fn test_build_search_with_pagination() {
        let client = given_we_have_a_search_client();
        let search_term = given_we_have_a_search_term();
        let config = given_we_have_a_config();
        let actual = when_we_build_a_search_request_with_pagination(client, search_term, config);
        then_search_url_with_pagination_is_as_expected(actual);
    }

    #[test]
    fn test_build_search_with_pagination_and_filter() {
        let client = given_we_have_a_search_client();
        let search_term = given_we_have_a_search_term();
        let config = given_we_have_a_config();
        let actual =
            when_we_build_a_search_request_with_pagination_and_filter(client, search_term, config);
        then_search_url_with_pagination_and_filter_is_as_expected(actual);
    }

    #[test]
    fn test_build_filter_by_collection_request() {
        let client = reqwest::Client::new();
        let config = AzureConfig {
            search_service: "my_cool_service".to_string(),
            search_index: "my_cool_search_index".to_string(),
            api_key: "my_cool_api_key".to_string(),
            api_version: "2017-11-11".to_string(),
        };

        let req = build_filter_by_collection_request(
            &"my_cool_field".to_string(),
            &"my cool value".to_string(),
            &"cooler_than".to_string(),
            &client,
            &config,
        )
        .unwrap();

        let api_key = req.headers().get("api-key").unwrap().to_str().unwrap();
        assert_eq!(api_key, config.api_key);

        let url = req.url();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("my_cool_service.search.windows.net"));
        assert_eq!(url.path(), "/indexes/my_cool_search_index/docs");

        let mut query = url.query_pairs();
        assert_eq!(
            query
                .find(|query_pair| query_pair.0 == "api-version")
                .unwrap()
                .1,
            "2017-11-11"
        );
        assert_eq!(
            query
                .find(|query_pair| query_pair.0 == "$filter")
                .unwrap()
                .1,
            "my_cool_field/any(value: value cooler_than 'my cool value')"
        );
    }
}
