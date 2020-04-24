use crate::{pagination, pagination::PageInfo};
use juniper::GraphQLObject;
use search_client::{models::IndexResult, Search};

#[derive(GraphQLObject, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
#[graphql(description = "A document")]
pub struct Document {
    product_name: Option<String>,
    active_substances: Option<Vec<String>>,
    title: Option<String>,
    highlights: Option<Vec<String>>,
    created: Option<String>,
    doc_type: Option<String>,
    file_size_in_bytes: Option<i32>,
    name: Option<String>,
    url: Option<String>,
}

impl Document {
    fn name_only(name: String) -> Self {
        Self {
            name: Some(name),
            product_name: None,
            active_substances: None,
            title: None,
            highlights: None,
            created: None,
            doc_type: None,
            file_size_in_bytes: None,
            url: None,
        }
    }
}

impl From<IndexResult> for Document {
    fn from(r: IndexResult) -> Self {
        Self {
            product_name: r.product_name,
            active_substances: Some(r.substance_name),
            title: Some(r.title),
            created: r.created,
            doc_type: Some(r.doc_type),
            file_size_in_bytes: Some(r.metadata_storage_size),
            name: Some(r.file_name),
            url: Some(r.metadata_storage_path),
            highlights: match r.highlights {
                Some(a) => Some(a.content),
                _ => None,
            },
        }
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
        .map(|&name| Document::name_only(name.to_string()))
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
