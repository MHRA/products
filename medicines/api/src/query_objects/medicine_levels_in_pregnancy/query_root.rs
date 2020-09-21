use crate::{
    azure_context::AzureContext,
    query_objects::medicine_levels_in_pregnancy::{
        report::{get_reports, Reports},
        substance::{get_substance, SubstanceReports},
    },
    query_objects::shared::substances_index::{get_substances_index, SubstanceIndex},
};
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

#[Object(desc = "Entrypoint for medicine levels in pregnancy")]
impl MedicineLevelsInPregnancy {
    #[field(desc = "substance")]
    async fn substance(
        &self,
        context: &Context<'_>,
        name: Option<String>,
    ) -> FieldResult<SubstanceReports> {
        match name {
            Some(name) => get_substance(name).await.map_err(|e| {
                tracing::error!("Error fetching results from Azure search service: {:?}", e);
                e.into()
            }),
            None => Err(anyhow::anyhow!(
                "Getting a substance without providing a substance name is not supported."
            )
            .into()),
        }
    }
    #[field(desc = "substances_index")]
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
                e.into()
            })
    }

    #[field(desc = "reports")]
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
            e.into()
        })
    }
}

fn get_offset_or_default(skip: Option<i32>, after: Option<String>, default: i32) -> i32 {
    match (after, skip) {
        (Some(encoded), _) => match convert_after_to_offset(encoded) {
            Ok(a) => a,
            _ => default,
        },
        (None, Some(offset)) => offset,
        _ => default,
    }
}

fn convert_after_to_offset(encoded: String) -> Result<i32, anyhow::Error> {
    let bytes = base64::decode(encoded)?;
    let string = std::str::from_utf8(&bytes)?;
    Ok(string.parse::<i32>()? + 1)
}
