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
    search: String,
    first: Option<i32>,
    last: Option<i32>,
    before: Option<String>,
    after: Option<String>,
) -> Result<Documents, anyhow::Error> {

    // TODO: Do something with last and after.

    let offset = after.unwrap_or("-1".to_string()).parse::<i32>().unwrap() + 1;
    let result_count = first.unwrap_or(10);

    let azure_result = client
        .search_with_pagination(
            &search,
            search_client::AzurePagination {
                result_count: result_count,
                offset: offset,
            },
            true,
        )
        .await?;

    let mut cursor = offset.clone();
    let edges = azure_result.search_results
        .iter()
        .map(|search_result| Document::from(search_result.clone()))
        .map(|document| {
            let edge = DocumentEdge {
                node: document,
                cursor: cursor.to_string(),
            };
            cursor += 1;
            return edge;
        })
        .collect();

    let total_count = azure_result.count.unwrap_or(0);
    let has_previous_page = offset != 0;
    let has_next_page = offset + result_count <= total_count;
    let start_cursor = offset.to_string();
    let end_cursor = std::cmp::min(total_count, offset + result_count - 1).to_string();

    Ok(Documents {
        edges,
        total_count: total_count,
        page_info: PageInfo {
            has_previous_page: has_previous_page,
            has_next_page: has_next_page,
            start_cursor: start_cursor,
            end_cursor: end_cursor,
        },
    })
}
