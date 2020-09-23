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
    reports: Vec<Report>,
    offset: i32,
    total_count: i32,
) -> Reports {
    let edges = get_report_edges(reports, offset);
    get_reports_from_edges(edges, offset, total_count)
}

pub struct AzureReportResult {
    reports: Vec<Report>,
    offset: i32,
    total_count: i32,
}

impl Into<Reports> for AzureReportResult {
    fn into(self) -> Reports {
        get_reports_graph_from_reports_vector(self.reports, self.offset, self.total_count)
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

    Ok(map_azure_result(azure_result, offset))
}

fn map_azure_result(result: ReportResults, offset: i32) -> AzureReportResult {
    let reports = result
        .search_results
        .into_iter()
        .map(Report::from)
        .collect();

    let total_count = result.count.unwrap_or(0);

    AzureReportResult {
        reports,
        total_count,
        offset,
    }
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
    use search_client::models::{AzureHighlight, ReportResults};

    fn given_a_search_result(report_name: &str) -> ReportResult {
        ReportResult {
            products: Some(vec!["product".to_string()]),
            metadata_storage_name: "storage_name".to_string(),
            metadata_storage_path: "test/path".to_string(),
            active_substances: vec!["substance".to_string()],
            report_name: report_name.to_string(),
            file_name: "file name".to_string(),
            matrices: Some(vec!["matrix".to_string()]),
            pbpk_models: Some(vec!["pbpk model".to_string()]),
            summary: "summary".to_string(),
            metadata_storage_size: 300,
            score: 1.0,
            highlights: Some(AzureHighlight {
                content: vec![String::from("highlight")],
            }),
        }
    }

    fn given_azure_search_results(reports: Vec<ReportResult>, count: i32) -> ReportResults {
        ReportResults {
            search_results: reports,
            context: String::default(),
            count: Some(count),
        }
    }

    fn given_a_single_search_result() -> ReportResults {
        let results = vec![given_a_search_result("first")];
        given_azure_search_results(results, 1234)
    }

    fn given_search_results() -> ReportResults {
        let results = vec![
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
        ];
        given_azure_search_results(results, 1234)
    }

    fn when_we_map_the_results(results: ReportResults) -> AzureReportResult {
        map_azure_result(results, 0)
    }

    fn then_all_fields_map_correctly(reports_response: AzureReportResult) {
        let first_result = reports_response.reports[0].clone();
        assert_eq!(first_result.products.unwrap().first().unwrap(), "product");
        assert_eq!(
            first_result.active_substances.unwrap().first().unwrap(),
            "substance"
        );
        assert_eq!(first_result.title.unwrap(), "first");
        assert_eq!(first_result.file_size_in_bytes.unwrap(), 300);
        assert_eq!(first_result.file_name.unwrap(), "file name");
        assert_eq!(first_result.file_url.unwrap(), "test/path");
        assert_eq!(first_result.summary.unwrap(), "summary");
        assert_eq!(first_result.matrices.unwrap().first().unwrap(), "matrix");
        assert_eq!(
            first_result.pbpk_models.unwrap().first().unwrap(),
            "pbpk model"
        );
        assert_eq!(
            first_result.highlights.unwrap().first().unwrap(),
            "highlight"
        );
    }

    fn then_we_have_the_expected_output(reports_response: AzureReportResult) {
        let expected_names = vec![
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth",
        ];
        let actual_names = reports_response
            .reports
            .iter()
            .filter_map(|report| report.title.clone())
            .collect::<Vec<String>>();
        assert!(actual_names.eq(&expected_names));

        assert_eq!(1234, reports_response.total_count);
    }

    #[test]
    fn test_map_result() {
        let search_results = given_a_single_search_result();
        let response = when_we_map_the_results(search_results);
        then_all_fields_map_correctly(response);
    }

    #[test]
    fn test_map_results() {
        let search_results = given_search_results();
        let response = when_we_map_the_results(search_results);
        then_we_have_the_expected_output(response);
    }
}
