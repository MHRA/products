use juniper::{FieldResult, RootNode};

use crate::{
    azure_context::AzureContext,
    product::{get_products_by_substance_name, Product},
    substance::{get_substances, Substances},
};

pub struct QueryRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl QueryRoot {
    async fn products(
        context: &AzureContext,
        substance_name: Option<String>,
    ) -> FieldResult<Vec<Product>> {
        match substance_name {
            Some(name) =>
                get_products_by_substance_name(&name, &context.client).await.map_err(|e| {
                    tracing::error!("Error fetching results from Azure search service: {:?}", e);

                    juniper::FieldError::new(
                        "Error fetching search results",
                        juniper::Value::null()
                    )
                }),
            None =>
                Err(juniper::FieldError::new(
                    "Getting a list of products without providing a substance name is not currently supported.",
                    juniper::Value::null()
                ))
        }
    }

    async fn substances(first: i32) -> FieldResult<Substances> {
        Ok(get_substances(first).await)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = AzureContext)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
