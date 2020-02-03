use juniper::{FieldResult, RootNode};

use crate::{
    product::{get_product, Products},
    substance::{get_substances, Substances},
};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn products(search_term: String) -> FieldResult<Option<Products>> {
        Ok(get_product(search_term).await)
    }

    async fn substances(first: i32) -> FieldResult<Substances> {
        Ok(get_substances(first).await)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
