use juniper::{FieldResult, RootNode};

use juniper::GraphQLObject;

use crate::{pagination};

use crate::pagination::{PageInfo};

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

struct Substance {
    name: String,
}

#[juniper::object]
#[graphql(description = "An active ingredient found in medical products")]
impl Substance {
    fn name(&self) -> &str {
        &self.name
    }
}

pagination!{Substances, SubstanceEdge, Substance}
pub struct QueryRoot;

fn get_substances(first: i32) -> Substances {
  let substances: [&str; 1000] = ["Ibuprofen"; 1000];
  let e = substances.iter().take(first as usize).map(|x| Substance {
      name: x.to_owned().to_owned()
  })
  .map(|y| SubstanceEdge {
      node: y,
      cursor: "cursor".to_owned()
  }).collect();
  
  Substances {
    edges: e,page_info: PageInfo {has_previous_page:false,has_next_page:first < 1000}
  }
}

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
