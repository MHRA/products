use crate::{pagination};
use crate::pagination::{PageInfo};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "An active ingredient found in medical products")]
struct Substance {
    name: String,
}

impl Substance {
    #[allow(dead_code)]
    fn name(&self) -> &str {
        &self.name
    }
}

pagination!{Substances, SubstanceEdge, Substance}

pub fn get_substances(first: i32) -> Substances {
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

