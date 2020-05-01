use juniper::{FieldResult, RootNode};

use crate::{
    azure_context::AzureContext,
    document::{get_documents, DocumentType, Documents},
    product::{get_product, get_substance_with_products, Product},
    substance::{get_substances_starting_with_letter, Substance},
};

pub struct QueryRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl QueryRoot {
    async fn substance(context: &AzureContext, name: Option<String>) -> FieldResult<Substance> {
        match name {
            Some(name) => get_substance_with_products(&name, &context.client)
                .await
                .map_err(|e| {
                    tracing::error!("Error fetching results from Azure search service: {:?}", e);
                    juniper::FieldError::new(
                        "Error fetching search results",
                        juniper::Value::null(),
                    )
                }),
            None => Err(juniper::FieldError::new(
                "Getting a substance without providing a substance name is not supported.",
                juniper::Value::null(),
            )),
        }
    }

    async fn product(_context: &AzureContext, name: String) -> FieldResult<Product> {
        get_product(name).await.map_err(|e| {
            tracing::error!("Error fetching results from Azure search service: {:?}", e);
            juniper::FieldError::new("Error fetching search results", juniper::Value::null())
        })
    }

    async fn substances_by_first_letter(
        context: &AzureContext,
        letter: String,
    ) -> FieldResult<Vec<Substance>> {
        get_substances_starting_with_letter(&context.client, letter.chars().next().unwrap())
            .await
            .map_err(|e| {
                tracing::error!("Error fetching results from Azure search service: {:?}", e);
                juniper::FieldError::new("Error fetching search results", juniper::Value::null())
            })
    }

    async fn documents(
        context: &AzureContext,
        search: Option<String>,
        first: Option<i32>,
        skip: Option<i32>,
        after: Option<String>,
        document_types: Option<Vec<DocumentType>>,
    ) -> FieldResult<Documents> {
        let offset = get_offset_or_default(skip, after, 0);

        let docs = get_documents(
            &context.client,
            search.as_deref().unwrap_or(" "),
            first,
            offset,
            document_types,
            None,
        )
        .await
        .map_err(|e| {
            tracing::error!("Error fetching results from Azure search service: {:?}", e);
            juniper::FieldError::new("Error fetching search results", juniper::Value::null())
        })?
        .into();

        Ok(docs)
    }
}

fn get_offset_or_default(skip: Option<i32>, after: Option<String>, default: i32) -> i32 {
    match (after, skip) {
        (Some(encoded), _) => match base_64_decode(encoded) {
            Ok(a) => a,
            _ => default,
        },
        (None, Some(offset)) => offset,
        _ => default,
    }
}

fn base_64_decode(encoded: String) -> Result<i32, anyhow::Error> {
    let bytes = base64::decode(encoded)?;
    let string = std::str::from_utf8(&bytes)?;
    Ok(string.parse::<i32>()? + 1)
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct SubscriptionRoot;

#[juniper::graphql_subscription(Context = AzureContext)]
impl SubscriptionRoot {}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, SubscriptionRoot {})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base_64_decode() {
        let encoded = base64::encode("1229".to_string());
        assert_eq!(base_64_decode(encoded).unwrap(), 1230);
    }
}
