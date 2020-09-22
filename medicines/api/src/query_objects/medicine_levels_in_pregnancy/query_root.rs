use crate::{
    azure_context::AzureContext,
    pagination::get_offset_or_default,
    query_objects::medicine_levels_in_pregnancy::{
        report::{get_reports, Reports},
        substance::{get_substance, SubstanceReports},
    },
    query_objects::shared::substances_index::{get_substances_index, SubstanceIndex},
};
use anyhow::anyhow;
use async_graphql::{Context, FieldResult, Object};

pub struct MedicineLevelsInPregnancy {
    substance: Option<SubstanceReports>,
    substances_index: Option<Vec<SubstanceIndex>>,
    reports: Option<Reports>,
}

impl MedicineLevelsInPregnancy {
    pub fn new() -> Self {
        Self {
            substance: None,
            substances_index: None,
            reports: None,
        }
    }
}

#[Object(desc = "Entrypoint for reports related to medicine levels in pregnancy")]
impl MedicineLevelsInPregnancy {
    #[field(desc = "Retrieves all reports associated with the queried active substance")]
    async fn substance(
        &self,
        _context: &Context<'_>,
        name: Option<String>,
    ) -> FieldResult<SubstanceReports> {
        match name {
            Some(name) => get_substance(name).await.map_err(|e| {
                tracing::error!("Error fetching results from Azure search service: {:?}", e);
                anyhow!("Error retrieving results").into()
            }),
            None => Err(anyhow::anyhow!(
                "Getting a substance without providing a substance name is not supported."
            )
            .into()),
        }
    }
    #[field(
        desc = "List of active substances beginning with the provided letter that have reports associated with them, along with the count of reports for each"
    )]
    async fn substances_index(
        &self,
        context: &Context<'_>,
        letter: String,
    ) -> FieldResult<Vec<SubstanceIndex>> {
        let context = context.data::<AzureContext>()?;
        get_substances_index(&context.bmgf_client, letter.chars().next().unwrap())
            .await
            .map_err(|e| {
                tracing::error!("Error fetching results from Azure search service: {:?}", e);
                anyhow!("Error retrieving results").into()
            })
    }

    #[field(desc = "Reports related to medicine levels in pregnancy")]
    async fn reports(
        &self,
        context: &Context<'_>,
        search: Option<String>,
        first: Option<i32>,
        skip: Option<i32>,
        after: Option<String>,
    ) -> FieldResult<Reports> {
        let context = context.data::<AzureContext>()?;
        let offset = get_offset_or_default(skip, after, 0);

        get_reports(
            &context.bmgf_client,
            search.as_deref().unwrap_or(" "),
            first,
            offset,
            None,
        )
        .await
        .map(Into::into)
        .map_err(|e| {
            tracing::error!("Error fetching results from Azure search service: {:?}", e);
            anyhow!("Error retrieving results").into()
        })
    }
}
