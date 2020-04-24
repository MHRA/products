use crate::{pagination, pagination::PageInfo, product::Product};
use search_client::{models::IndexResult, Search};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A document")]
pub struct Document {
    name: String,
    // Yes, more fields will be added here.
}

impl Document {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pagination! {Documents, DocumentEdge, Document}

pub async fn get_documents(
    client: &impl Search,
    search: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    before: Option<String>,
    after: Option<String>,
) -> Documents {
    let placeholder_names: [&str; 1000] = ["A cool document"; 1000];
    let edges = placeholder_names
        .iter()
        // .take(first as usize)
        .map(|&name| Document::new(name.to_string()))
        .map(|document| DocumentEdge {
            node: document,
            cursor: "cursor".to_owned(),
        })
        .collect();

    Documents {
        edges,
        total_count: 1000,
        page_info: PageInfo {
            has_previous_page: false,
            has_next_page: false,
            start_cursor: "start cursor here".to_string(),
            end_cursor: "end cursor here".to_string(),
        },
    }
}
