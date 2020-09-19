use async_graphql::{Context, EmptyMutation, EmptySubscription, FieldResult, Object, Schema};

use crate::{azure_context::AzureContext, products_query_root::Products};
use search_client::models::DocumentType;

pub struct QueryRoot;

#[Object(desc = "Query root")]
impl QueryRoot {
    // async fn substance(
    //     &self,
    //     context: &Context<'_>,
    //     name: Option<String>,
    // ) -> FieldResult<Substance> {
    //     let context = context.data::<AzureContext>()?;
    //     match name {
    //         Some(name) => get_substance_with_products(&name, &context.client)
    //             .await
    //             .map_err(|e| {
    //                 tracing::error!("Error fetching results from Azure search service: {:?}", e);
    //                 e.into()
    //             }),
    //         None => Err(anyhow::anyhow!(
    //             "Getting a substance without providing a substance name is not supported."
    //         )
    //         .into()),
    //     }
    // }

    // async fn product(&self, _context: &Context<'_>, name: String) -> FieldResult<Product> {
    //     get_product(name).await.map_err(|e| {
    //         tracing::error!("Error fetching results from Azure search service: {:?}", e);
    //         e.into()
    //     })
    // }

    // async fn substances_index(
    //     &self,
    //     context: &Context<'_>,
    //     letter: String,
    // ) -> FieldResult<Vec<SubstanceIndex>> {
    //     let context = context.data::<AzureContext>()?;
    //     get_substances_index(&context.client, letter.chars().next().unwrap())
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Error fetching results from Azure search service: {:?}", e);
    //             e.into()
    //         })
    // }

    // async fn products_index(
    //     &self,
    //     context: &Context<'_>,
    //     substance: String,
    // ) -> FieldResult<Vec<ProductIndex>> {
    //     let context = context.data::<AzureContext>()?;
    //     get_products_index(&context.client, &substance)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Error fetching results from Azure search service: {:?}", e);
    //             e.into()
    //         })
    // }

    // async fn documents(
    //     &self,
    //     context: &Context<'_>,
    //     search: Option<String>,
    //     first: Option<i32>,
    //     skip: Option<i32>,
    //     after: Option<String>,
    //     document_types: Option<Vec<DocumentType>>,
    // ) -> FieldResult<Documents> {
    //     let context = context.data::<AzureContext>()?;
    //     let offset = get_offset_or_default(skip, after, 0);

    //     get_documents(
    //         &context.client,
    //         search.as_deref().unwrap_or(" "),
    //         first,
    //         offset,
    //         document_types,
    //         None,
    //     )
    //     .await
    //     .map(Into::into)
    //     .map_err(|e| {
    //         tracing::error!("Error fetching results from Azure search service: {:?}", e);
    //         e.into()
    //     })
    // }

    async fn products(&self, context: &Context<'_>) -> FieldResult<Products> {
        Ok(Products::new())
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

type QuerySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct ApiSchema(pub QuerySchema);

impl ApiSchema {
    pub fn new(context: AzureContext) -> ApiSchema {
        ApiSchema(
            Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
                .data(context)
                .finish(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case("LTE=".to_string(), 0; "for the first page of results")]
    #[test_case("MA==".to_string(), 1; "for grabbing the second result onwards")]
    #[test_case("OQ==".to_string(), 10; "for the second page of results when pagesize is 10")]
    #[test_case(base64::encode("1229".to_string()), 1230; "for as late as page 124")]
    fn test_convert_after_to_offset(encoded: String, expected: i32) {
        assert_eq!(convert_after_to_offset(encoded).unwrap(), expected);
    }

    #[test_case(Some(10), Some("LTE=".to_string()), 15, 0; "matches after when only after is provided")]
    #[test_case(Some(10), None, 15, 10; "matches skip when only skip is provided")]
    #[test_case(None, Some("LTE=".to_string()), 15, 0; "matches after when both are provided")]
    #[test_case(None, None, 10, 10; "matches default when neither are provided")]
    fn test_get_offset_or_default(
        skip: Option<i32>,
        after: Option<String>,
        default: i32,
        expected: i32,
    ) {
        let offset = get_offset_or_default(skip, after, default);
        assert_eq!(offset, expected);
    }
}
