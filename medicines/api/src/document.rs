use crate::{pagination, pagination::PageInfo};
use juniper::GraphQLObject;
use search_client::{models::IndexResult, Search};

#[derive(GraphQLObject, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
#[graphql(description = "A document")]
pub struct Document {
    product_name: Option<String>,
    active_substances: Option<Vec<String>>,
    title: Option<String>,
    highlights: Option<Vec<String>>,
    created: Option<String>,
    doc_type: Option<String>,
    file_size_in_bytes: Option<i32>,
    name: Option<String>,
    url: Option<String>,
}

impl From<IndexResult> for Document {
    fn from(r: IndexResult) -> Self {
        Self {
            product_name: r.product_name,
            active_substances: Some(r.substance_name),
            title: Some(r.title),
            created: r.created,
            doc_type: Some(r.doc_type),
            file_size_in_bytes: Some(r.metadata_storage_size),
            name: Some(r.file_name),
            url: Some(r.metadata_storage_path),
            highlights: match r.highlights {
                Some(a) => Some(a.content),
                _ => None,
            },
        }
    }
}

pagination! {Documents, DocumentEdge, Document}

pub async fn get_documents(
    client: &impl Search,
    search: String,
    first: Option<i32>,
    last: Option<i32>,
    before: Option<String>,
    after: Option<String>,
) -> Result<Documents, anyhow::Error> {
    // TODO: Do something with last and after.

    let offset = after.unwrap_or("-1".to_string()).parse::<i32>().unwrap() + 1;
    let result_count = first.unwrap_or(10);

    let azure_result = client
        .search_with_pagination(
            &search,
            search_client::AzurePagination {
                result_count: result_count,
                offset: offset,
            },
            true,
        )
        .await?;

    let mut cursor = offset.clone();
    let edges = azure_result
        .search_results
        .iter()
        .map(|search_result| Document::from(search_result.clone()))
        .map(|document| {
            let edge = DocumentEdge {
                node: document,
                cursor: cursor.to_string(),
            };
            cursor += 1;
            return edge;
        })
        .collect();

    let total_count = azure_result.count.unwrap_or(0);
    let has_previous_page = offset != 0;
    let has_next_page = offset + result_count <= total_count;
    let start_cursor = offset.to_string();
    let end_cursor = std::cmp::min(total_count, offset + result_count - 1).to_string();

    Ok(Documents {
        edges,
        total_count: total_count,
        page_info: PageInfo {
            has_previous_page: has_previous_page,
            has_next_page: has_next_page,
            start_cursor: start_cursor,
            end_cursor: end_cursor,
        },
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use async_trait::async_trait;
    use search_client::models::IndexResults;
    use tokio_test::block_on;

    struct TestAzureSearchClient {
        pub search_results: Vec<IndexResult>,
    }

    #[async_trait]
    impl Search for TestAzureSearchClient {
        async fn search(&self, _search_term: &str) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
        async fn search_with_pagination(
            &self,
            _search_term: &str,
            _pagination: search_client::AzurePagination,
            _include_count: bool,
        ) -> Result<IndexResults, reqwest::Error> {
            Ok(IndexResults {
                search_results: self.search_results.clone(),
                context: String::from(""),
                count: Some(1234),
            })
        }
        async fn filter_by_collection_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
        async fn filter_by_non_collection_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<IndexResults, reqwest::Error> {
            unimplemented!()
        }
    }

    fn given_some_search_results() -> Vec<IndexResult> {
        vec![
            IndexResult {
                doc_type: "Spc".to_string(),
                file_name: "our_id".to_string(),
                metadata_storage_name: "storage_name".to_string(),
                metadata_storage_path: "test/path".to_string(),
                product_name: Some("product".to_string()),
                substance_name: vec!["substance".to_string()],
                title: "title".to_string(),
                created: None,
                facets: vec!["facet".to_string()],
                keywords: None,
                metadata_storage_size: 300,
                release_state: None,
                rev_label: None,
                suggestions: vec!["suggestion".to_string()],
                score: 1.0,
                highlights: None,
            },
            IndexResult {
                doc_type: "Par".to_string(),
                file_name: "other_id".to_string(),
                metadata_storage_name: "other_storage_name".to_string(),
                metadata_storage_path: "test/other_path".to_string(),
                product_name: Some("product two".to_string()),
                substance_name: vec!["ecnatsbus".to_string()],
                title: "Jaws".to_string(),
                created: None,
                facets: vec!["otherfacet".to_string()],
                keywords: None,
                metadata_storage_size: 300,
                release_state: None,
                rev_label: None,
                suggestions: vec!["the other document".to_string()],
                score: 0.9,
                highlights: None,
            },
        ]
    }

    fn given_a_search_client(search_results: &Vec<IndexResult>) -> impl Search {
        TestAzureSearchClient {
            search_results: search_results.clone(),
        }
    }

    fn when_we_get_the_first_page_of_documents(search_client: impl Search) -> Documents {
        block_on(get_documents(
            &search_client,
            "Search string".to_string(),
            None,
            None,
            None,
            None,
        ))
        .unwrap()
    }

    fn then_we_have_expected_documents(
        documents_response: &Documents,
        search_results: &Vec<IndexResult>,
    ) {
        assert_eq!(documents_response.edges.len(), search_results.len());
        let expected_names = search_results
            .into_iter()
            .map(|result| result.product_name.to_owned().unwrap());
        let edges = &documents_response.edges;
        let actual_names = edges
            .into_iter()
            .map(|edge| edge.node.product_name.as_ref().unwrap().as_ref());
        assert!(expected_names.eq(actual_names));
    }

    fn then_we_have_first_page(documents_response: &Documents) {
        assert_eq!(1234, documents_response.total_count);
        let expected_page_info = PageInfo {
            has_previous_page: false,
            has_next_page: true,
            start_cursor: "0".to_string(),
            end_cursor: "9".to_string(),
        };
        assert_eq!(expected_page_info, documents_response.page_info);
    }

    #[test]
    fn test_get_documents_first_page() {
        let search_results = given_some_search_results();
        let search_client = given_a_search_client(&search_results);
        let response = when_we_get_the_first_page_of_documents(search_client);
        then_we_have_expected_documents(&response, &search_results);
        then_we_have_first_page(&response);
    }
}
