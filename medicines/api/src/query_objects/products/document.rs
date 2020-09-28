use crate::{pagination, pagination::PageInfo};
use async_graphql::SimpleObject;
use search_client::{
    models::{DocumentType, IndexResult, IndexResults},
    Search,
};

#[SimpleObject(desc = "An SPC, PIL or PAR document")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Document {
    #[field(desc = "Product associated with document")]
    pub product_name: Option<String>,
    #[field(desc = "Active substances associated with document")]
    pub active_substances: Option<Vec<String>>,
    #[field(desc = "Title")]
    pub title: Option<String>,
    #[field(desc = "Highlights")]
    pub highlights: Option<Vec<String>>,
    #[field(desc = "Created date")]
    pub created: Option<String>,
    #[field(desc = "Document type")]
    pub doc_type: Option<DocumentType>,
    #[field(desc = "File size")]
    pub file_size_in_bytes: Option<i32>,
    #[field(desc = "PDF file name")]
    pub name: Option<String>,
    #[field(desc = "PDF file url")]
    pub url: Option<String>,
}

impl Document {
    pub fn is_doc_type(&self, doc_type: DocumentType) -> bool {
        self.doc_type == Some(doc_type)
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

fn get_document_edges(docs: Vec<Document>, offset: i32) -> Vec<DocumentEdge> {
    docs.into_iter()
        .enumerate()
        .map(|(i, document)| DocumentEdge {
            node: document,
            cursor: base64::encode((i as i32 + offset).to_string()),
        })
        .collect()
}

fn get_documents_from_edges(edges: Vec<DocumentEdge>, offset: i32, total_count: i32) -> Documents {
    let result_count = edges.len() as i32;

    Documents {
        edges,
        total_count,
        page_info: PageInfo::build(offset, result_count, total_count),
    }
}

pub fn get_documents_graph_from_documents_vector(
    docs: Vec<Document>,
    offset: i32,
    total_count: i32,
) -> Documents {
    let edges = get_document_edges(docs, offset);
    get_documents_from_edges(edges, offset, total_count)
}

pub struct AzureDocumentResult {
    docs: Vec<Document>,
    offset: i32,
    total_count: i32,
}

impl Into<Documents> for AzureDocumentResult {
    fn into(self) -> Documents {
        get_documents_graph_from_documents_vector(self.docs, self.offset, self.total_count)
    }
}

pub async fn get_documents(
    client: &impl Search,
    search: &str,
    first: Option<i32>,
    offset: i32,
    document_types: Option<Vec<DocumentType>>,
    product_name: Option<&str>,
) -> Result<AzureDocumentResult, anyhow::Error> {
    let result_count = first.unwrap_or(10);

    let azure_result = client
        .search_with_pagination_and_filter::<IndexResults>(
            &search,
            search_client::AzurePagination {
                result_count,
                offset,
            },
            true,
            build_filter(document_types, product_name).as_deref(),
        )
        .await?;

    Ok(map_azure_result(azure_result, offset))
}

fn map_azure_result(result: IndexResults, offset: i32) -> AzureDocumentResult {
    let docs = result
        .search_results
        .into_iter()
        .map(Document::from)
        .collect();

    let total_count = result.count.unwrap_or(0);

    AzureDocumentResult {
        docs,
        total_count,
        offset,
    }
}

fn build_filter(
    document_types: Option<Vec<DocumentType>>,
    product_name: Option<&str>,
) -> Option<String> {
    let docs_filter = document_types.and_then(build_document_types_filter);
    let products_filter = product_name.map(build_product_name_filter);

    let filters: Vec<String> = products_filter.into_iter().chain(docs_filter).collect();

    match &filters[..] {
        [] => None,
        [filter] => Some(filter.clone()),
        _ => Some(format!("({})", filters.join(" and "))),
    }
}

fn build_document_types_filter(document_types: Vec<DocumentType>) -> Option<String> {
    if document_types.is_empty() {
        return None;
    }

    Some(format!(
        "({})",
        document_types
            .into_iter()
            .map(|document_type| format!("doc_type eq '{}'", document_type))
            .collect::<Vec<_>>()
            .join(" or ")
    ))
}

fn build_product_name_filter(product_name: &str) -> String {
    format!("(product_name eq '{}')", product_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use search_client::models::{AzureHighlight, IndexResult};
    use test_case::test_case;

    fn given_a_search_result(product_name: &str) -> IndexResult {
        IndexResult {
            product_name: Some(product_name.to_string()),
            doc_type: DocumentType::Spc,
            file_name: "our_id".to_string(),
            metadata_storage_name: "storage_name".to_string(),
            metadata_storage_path: "test/path".to_string(),
            substance_name: vec!["substance".to_string()],
            title: "title".to_string(),
            created: Some("created".to_string()),
            facets: vec!["facet".to_string()],
            keywords: None,
            metadata_storage_size: 300,
            release_state: None,
            rev_label: None,
            suggestions: vec!["suggestion".to_string()],
            score: 1.0,
            highlights: Some(AzureHighlight {
                content: vec![String::from("highlight")],
            }),
        }
    }

    fn given_azure_search_results(reports: Vec<IndexResult>, count: i32) -> IndexResults {
        IndexResults {
            search_results: reports,
            context: String::default(),
            count: Some(count),
        }
    }

    fn given_a_single_search_result() -> IndexResults {
        let results = vec![given_a_search_result("first")];
        given_azure_search_results(results, 1234)
    }

    fn given_search_results() -> IndexResults {
        let results = vec![
            given_a_search_result("first"),
            given_a_search_result("second"),
            given_a_search_result("third"),
            given_a_search_result("fourth"),
            given_a_search_result("fifth"),
            given_a_search_result("sixth"),
            given_a_search_result("seventh"),
            given_a_search_result("eighth"),
            given_a_search_result("ninth"),
            given_a_search_result("tenth"),
        ];
        given_azure_search_results(results, 1234)
    }

    fn when_we_map_the_results(results: IndexResults) -> AzureDocumentResult {
        map_azure_result(results, 0)
    }

    fn then_all_fields_map_correctly(reports_response: AzureDocumentResult) {
        let first_result = reports_response.docs[0].clone();
        assert_eq!(first_result.product_name.unwrap(), "first");
        assert_eq!(
            first_result.active_substances.unwrap().first().unwrap(),
            "substance"
        );
        assert_eq!(first_result.title.unwrap(), "title");
        assert_eq!(first_result.file_size_in_bytes.unwrap(), 300);
        assert_eq!(first_result.created.unwrap(), "created");
        assert_eq!(first_result.doc_type.unwrap(), DocumentType::Spc);
        assert_eq!(first_result.name.unwrap(), "our_id");
        assert_eq!(first_result.url.unwrap(), "test/path");
        assert_eq!(
            first_result.highlights.unwrap().first().unwrap(),
            "highlight"
        );
    }

    fn then_we_have_the_expected_output(documents_response: AzureDocumentResult) {
        let expected_names = vec![
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth",
        ];
        let actual_names = documents_response
            .docs
            .iter()
            .filter_map(|document| document.product_name.clone())
            .collect::<Vec<String>>();
        assert!(actual_names.eq(&expected_names));

        assert_eq!(1234, documents_response.total_count);
    }

    #[test]
    fn test_map_result() {
        let search_results = given_a_single_search_result();
        let response = when_we_map_the_results(search_results);
        then_all_fields_map_correctly(response);
    }

    #[test]
    fn test_map_results() {
        let search_results = given_search_results();
        let response = when_we_map_the_results(search_results);
        then_we_have_the_expected_output(response);
    }

    #[test_case(None, None, None)]
    #[test_case(
        Some(vec![]),
        None,
        None
    )]
    #[test_case(
        Some(vec![DocumentType::Spc, DocumentType::Pil,DocumentType::Par,]),
        Some("IBUPROFEN 100MG CAPLETS"),
        Some("((product_name eq 'IBUPROFEN 100MG CAPLETS') and (doc_type eq 'Spc' or doc_type eq 'Pil' or doc_type eq 'Par'))")
    )]
    #[test_case(
        Some(vec![DocumentType::Spc,  DocumentType::Pil,DocumentType::Par,]),
        None,
        Some("(doc_type eq 'Spc' or doc_type eq 'Pil' or doc_type eq 'Par')")
    )]
    #[test_case(
        None,
        Some("IBUPROFEN 100MG CAPLETS"),
        Some("(product_name eq 'IBUPROFEN 100MG CAPLETS')")
    )]
    #[test_case(
        Some(vec![]),
        Some("IBUPROFEN 100MG CAPLETS"),
        Some("(product_name eq 'IBUPROFEN 100MG CAPLETS')")
    )]
    fn test_build_filter(
        document_types: Option<Vec<DocumentType>>,
        product_name: Option<&str>,
        expected_filter: Option<&str>,
    ) {
        assert_eq!(
            expected_filter.map(|s| s.to_string()),
            build_filter(document_types, product_name)
        );
    }
}
