use juniper::{FieldResult, RootNode};

use crate::{
    azure_context::AzureContext,
    document::{get_documents, Documents},
    product::get_substance_with_products,
    substance::{get_substances, Substance, Substances},
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

    async fn substances(first: i32) -> FieldResult<Substances> {
        Ok(get_substances(first).await)
    }

    async fn documents(
        context: &AzureContext,
        search: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        before: Option<String>,
        after: Option<String>,
    ) -> FieldResult<Documents> {
        Ok(get_documents(&context.client, search, first, last, before, after).await)
    }
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
