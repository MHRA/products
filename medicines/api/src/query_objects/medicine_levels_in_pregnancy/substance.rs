use crate::query_objects::medicine_levels_in_pregnancy::report::{
    get_reports, get_reports_graph_from_reports_vector, Report, Reports,
};
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

#[Object(desc = "A medical product containing active ingredients")]
impl SubstanceReports {
    #[field(desc = "name")]
    async fn name(&self) -> &str {
        &self.name
    }

    #[field(desc = "reports")]
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
                e.into()
            })
        }
    }
}

pub async fn get_substance(substance_name: String) -> Result<SubstanceReports, reqwest::Error> {
    Ok(SubstanceReports::new(substance_name, None))
}
