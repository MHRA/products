use crate::{pagination, pagination::PageInfo};
use async_graphql::SimpleObject;
use search_client::{
    models::{ReportResult, ReportResults},
    Search,
};

#[SimpleObject(desc = "A report related to medicine levels in pregnancy")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Report {
    #[field(desc = "Products associated with report")]
    pub products: Option<Vec<String>>,
    #[field(desc = "Active substances associated with report")]
    pub active_substances: Option<Vec<String>>,
    #[field(desc = "Report name")]
    pub title: Option<String>,
    #[field(desc = "Highlights")]
    pub highlights: Option<Vec<String>>,
    #[field(desc = "File size")]
    pub file_size_in_bytes: Option<i32>,
    #[field(desc = "PDF file name")]
    pub file_name: Option<String>,
    #[field(desc = "PDF file url")]
    pub file_url: Option<String>,
    #[field(desc = "Summary")]
    pub summary: Option<String>,
    #[field(desc = "Matrices")]
    pub matrices: Option<Vec<String>>,
    #[field(desc = "PBPK models")]
    pub pbpk_models: Option<Vec<String>>,
}

impl From<ReportResult> for Report {
    fn from(r: ReportResult) -> Self {
        Self {
            products: r.products,
            active_substances: Some(r.active_substances),
            title: Some(r.report_name),
            file_size_in_bytes: Some(r.metadata_storage_size),
            file_name: Some(r.file_name),
            file_url: Some(r.metadata_storage_path),
            summary: Some(r.summary),
            matrices: r.matrices,
            pbpk_models: r.pbpk_models,
            highlights: match r.highlights {
                Some(a) => Some(a.content),
                _ => None,
            },
        }
    }
}

pagination! {Reports, ReportEdge, Report}

fn get_report_edges(reports: Vec<Report>, offset: i32) -> Vec<ReportEdge> {
    reports
        .into_iter()
        .enumerate()
        .map(|(i, report)| ReportEdge {
            node: report,
            cursor: base64::encode((i as i32 + offset).to_string()),
        })
        .collect()
}

fn get_reports_from_edges(edges: Vec<ReportEdge>, offset: i32, total_count: i32) -> Reports {
    let result_count = edges.len() as i32;

    Reports {
        edges,
        total_count,
        page_info: PageInfo::build(offset, result_count, total_count),
    }
}

pub fn get_reports_graph_from_reports_vector(
    docs: Vec<Report>,
    offset: i32,
    total_count: i32,
) -> Reports {
    let edges = get_report_edges(docs, offset);
    get_reports_from_edges(edges, offset, total_count)
}

pub struct AzureReportResult {
    docs: Vec<Report>,
    offset: i32,
    total_count: i32,
}

impl Into<Reports> for AzureReportResult {
    fn into(self) -> Reports {
        get_reports_graph_from_reports_vector(self.docs, self.offset, self.total_count)
    }
}

pub async fn get_reports(
    client: &impl Search,
    search: &str,
    first: Option<i32>,
    offset: i32,
    substance_name: Option<&str>,
) -> Result<AzureReportResult, anyhow::Error> {
    let result_count = first.unwrap_or(10);

    let azure_result = client
        .search_with_pagination_and_filter::<ReportResults>(
            &search,
            search_client::AzurePagination {
                result_count,
                offset,
            },
            true,
            build_filter(substance_name).as_deref(),
        )
        .await?;

    let docs = azure_result
        .search_results
        .into_iter()
        .map(Report::from)
        .collect();

    let total_count = azure_result.count.unwrap_or(0);

    Ok(AzureReportResult {
        docs,
        total_count,
        offset,
    })
}

fn build_filter(substance_name: Option<&str>) -> Option<String> {
    match substance_name {
        Some(substance) => Some(build_substance_name_filter(substance)),
        None => None,
    }
}

fn build_substance_name_filter(substance_name: &str) -> String {
    format!(
        "active_substances/any(substance: substance eq '{}')",
        substance_name
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use async_trait::async_trait;
    use search_client::models::{FacetResults, ReportResults};
    use serde::de::DeserializeOwned;
    use test_case::test_case;
    use tokio_test::block_on;

    struct TestAzureSearchClient {
        pub search_results: Vec<ReportResult>,
    }

    impl TestAzureSearchClient {
        fn new(search_results: Vec<ReportResult>) -> Self {
            Self { search_results }
        }
    }

    #[async_trait]
    impl Search for TestAzureSearchClient {
        async fn search<T>(&self, _search_term: &str) -> Result<T, reqwest::Error>
        where
            T: DeserializeOwned,
        {
            unimplemented!()
        }
        async fn search_with_pagination<T>(
            &self,
            _search_term: &str,
            _pagination: search_client::AzurePagination,
            _include_count: bool,
        ) -> Result<T, reqwest::Error>
        where
            T: DeserializeOwned,
        {
            unimplemented!();
        }
        async fn search_with_pagination_and_filter<T>(
            &self,
            _search_term: &str,
            _pagination: search_client::AzurePagination,
            _include_count: bool,
            _filter: Option<&str>,
        ) -> Result<T, reqwest::Error>
        where
            T: DeserializeOwned,
        {
            Ok(ReportResults {
                search_results: self.search_results.clone(),
                context: String::from(""),
                count: Some(1234),
            })
        }
        async fn search_by_facet_field(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<FacetResults, reqwest::Error> {
            unimplemented!()
        }
        async fn filter_by_collection_field<T>(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<T, reqwest::Error>
        where
            T: DeserializeOwned,
        {
            unimplemented!()
        }
        async fn filter_by_non_collection_field<T>(
            &self,
            _field_name: &str,
            _field_value: &str,
        ) -> Result<T, reqwest::Error>
        where
            T: DeserializeOwned,
        {
            unimplemented!()
        }
    }

    fn given_a_search_result(product_name: &str) -> ReportResult {
        ReportResult {
            products: Some(vec!["product".to_string()]),
            metadata_storage_name: "storage_name".to_string(),
            metadata_storage_path: "test/path".to_string(),
            active_substances: vec!["substance".to_string()],
            report_name: "title".to_string(),
            file_name: "file name".to_string(),
            matrices: Some(vec!["matrix".to_string()]),
            pbpk_models: Some(vec!["pbpk model".to_string()]),
            summary: "summary".to_string(),
            metadata_storage_size: 300,
            score: 1.0,
            highlights: None,
        }
    }

    fn given_first_page_of_search_results() -> Vec<ReportResult> {
        vec![
            given_a_search_result("first"),
            given_a_search_result("second"),
            given_a_search_result("third"),
            given_a_search_result("fourth"),
            given_a_search_result("fifth"),
            given_a_search_result("sixth"),
            given_a_search_result("seventh"),
            given_a_search_result("eighth"),
            given_a_search_result("ninth"),
            given_a_search_result("tenth"),
        ]
    }

    fn given_last_page_of_search_results() -> Vec<ReportResult> {
        vec![
            given_a_search_result("fourth last"),
            given_a_search_result("third last"),
            given_a_search_result("second last"),
            given_a_search_result("last"),
        ]
    }

    fn given_a_search_client(search_results: &[ReportResult]) -> TestAzureSearchClient {
        TestAzureSearchClient::new(search_results.to_owned())
    }

    fn when_we_get_the_first_page_of_reports(search_client: impl Search) -> Reports {
        block_on(get_reports(&search_client, "Search string", None, 0))
            .unwrap()
            .into()
    }

    fn when_we_get_the_last_page_of_reports(search_client: impl Search) -> Reports {
        block_on(get_reports(&search_client, "Search string", None, 1230))
            .unwrap()
            .into()
    }

    fn then_we_have_the_first_page(reports_response: &Reports) {
        let expected_names = vec![
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth",
        ];
        let edges = &reports_response.edges;
        let actual_names = edges
            .iter()
            .map(|edge| edge.node.product_name.as_ref().unwrap());
        assert!(actual_names.eq(expected_names));

        assert_eq!(1234, reports_response.total_count);

        let expected_page_info = PageInfo {
            has_previous_page: false,
            has_next_page: true,
            start_cursor: base64::encode("0"),
            end_cursor: base64::encode("9"),
        };
        assert_eq!(expected_page_info, reports_response.page_info);
    }

    fn then_we_have_the_last_page(reports_response: &Reports) {
        let expected_names = vec!["fourth last", "third last", "second last", "last"];
        let edges = &reports_response.edges;
        let actual_names = edges
            .iter()
            .map(|edge| edge.node.product_name.as_ref().unwrap());

        assert!(actual_names.eq(expected_names));

        assert_eq!(1234, reports_response.total_count);
        let expected_page_info = PageInfo {
            has_previous_page: true,
            has_next_page: false,
            start_cursor: base64::encode("1230"),
            end_cursor: base64::encode("1233"),
        };
        assert_eq!(expected_page_info, reports_response.page_info);
    }

    #[test]
    fn test_get_reports_first_page() {
        let search_results = given_first_page_of_search_results();
        let search_client = given_a_search_client(&search_results);
        let response = when_we_get_the_first_page_of_reports(search_client);
        then_we_have_the_first_page(&response);
    }

    #[test]
    fn test_get_reports_last_page() {
        let search_results = given_last_page_of_search_results();
        let search_client = given_a_search_client(&search_results);
        let response = when_we_get_the_last_page_of_reports(search_client);
        then_we_have_the_last_page(&response);
    }
}
