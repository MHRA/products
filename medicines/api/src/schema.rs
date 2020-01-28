use juniper::{FieldResult, RootNode};

use juniper::GraphQLObject;

use crate::substance::{Substances, get_substances};

#[derive(GraphQLObject)]
#[graphql(description = "A medical product containing active ingredients")]
struct Product {
    id: String,
    name: String,
    pdf_url: Option<String>,
    substances: Option<Vec<String>>,
    file_name: Option<String>,
    release_state: Option<String>,
    doc_type: Option<String>,
    title: Option<String>,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn product(id: String) -> FieldResult<Product> {
        Ok(Product {
            id: "1234".to_owned(),
            name: "Nurofen".to_owned(),
            pdf_url: None,
            substances: None,
            file_name: None,
            release_state: None,
            doc_type: None,
            title: None,
        })
    }

    fn substances(first: i32) -> FieldResult<Substances> {
        Ok(get_substances(first))
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
