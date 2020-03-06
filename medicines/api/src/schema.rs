use juniper::{FieldResult, RootNode};

use crate::{
    azure_search::AzureContext,
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
        if substance_name.is_some() {
            return Ok(
                get_products_by_substance_name(substance_name.unwrap(), &context.client).await,
            );
        }

        Err(juniper::FieldError::new(
            "Getting a list of products without providing a substance name is not currently supported.",
            juniper::Value::null()
        ))
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
