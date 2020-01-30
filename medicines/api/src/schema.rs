use juniper::{FieldResult, RootNode};

use crate::substance::{Substances, get_substances};
use crate::product::{Products, get_product};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn products(id: String) ->FieldResult<Option<Products>> {
        Ok(get_product(id).await)
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
