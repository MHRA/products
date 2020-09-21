use crate::query_objects::medicine_levels_in_pregnancy::report::{
    get_reports, get_reports_graph_from_reports_vector, Report, Reports,
};
use anyhow::anyhow;
use async_graphql::{FieldResult, Object};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SubstanceReports {
    name: String,
    reports: Option<Vec<Report>>,
}

impl SubstanceReports {
    pub fn new(name: String, reports: Option<Vec<Report>>) -> Self {
        Self { name, reports }
    }

    pub fn add(&mut self, report: Report) {
        if let Some(ref mut v) = self.reports {
            v.push(report);
        } else {
            self.reports = Some(vec![report])
        }
    }
}

#[Object(desc = "An active ingredient found in medical products")]
impl SubstanceReports {
    #[field(desc = "Name")]
    async fn name(&self) -> &str {
        &self.name
    }

    #[field(desc = "Reports related to active substance")]
    async fn reports(&self, first: Option<i32>, offset: Option<i32>) -> FieldResult<Reports> {
        let offset = match offset {
            Some(a) => a,
            None => 0,
        };

        if let Some(reports) = self.reports.clone() {
            let total_count = reports.len() as i32;

            let reports = match first {
                Some(t) => reports.into_iter().take(t as usize).collect(),
                None => reports,
            };

            Ok(get_reports_graph_from_reports_vector(
                reports,
                offset,
                total_count,
            ))
        } else {
            get_reports(
                &search_client::AzureSearchClient::new_with_index("bmgf-index".to_string()),
                "",
                first,
                offset,
                Some(&self.name),
            )
            .await
            .map(Into::into)
            .map_err(|e| {
                tracing::error!("Error fetching reeports from Azure search service: {:?}", e);
                anyhow!("Error retrieving results").into()
            })
        }
    }
}

pub async fn get_substance(substance_name: String) -> Result<SubstanceReports, reqwest::Error> {
    Ok(SubstanceReports::new(substance_name, None))
}

#[cfg(test)]
mod test {
    use super::*;
    use search_client::models::ReportResult;

    fn azure_result_factory(product_name: Option<String>) -> Report {
        let result = ReportResult {
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
        };

        result.into()
    }

    #[test]
    fn test_sort_substances() {
        let mut substances = Vec::<Substance>::new();
        substances.push(Substance::new("Ibuprofen".to_owned(), None));
        substances.push(Substance::new("Paracetamol".to_owned(), None));
        substances.push(Substance::new("Aspirin".to_owned(), None));
        substances.sort();
        assert_eq!(substances[0].name, "Aspirin");
        assert_eq!(substances[1].name, "Ibuprofen");
        assert_eq!(substances[2].name, "Paracetamol");
    }
}
