// use crate::{pagination, pagination::PageInfo};
// use async_graphql::SimpleObject;
// use search_client::{
//     models::{DocumentType, IndexResult},
//     Search,
// };

// #[SimpleObject(desc = "A document")]
// #[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
// pub struct Document {
//     #[field(desc = "product name")]
//     pub product_name: Option<String>,
//     #[field(desc = "active substances")]
//     pub active_substances: Option<Vec<String>>,
//     #[field(desc = "title")]
//     pub title: Option<String>,
//     #[field(desc = "highlights")]
//     pub highlights: Option<Vec<String>>,
//     #[field(desc = "created")]
//     pub created: Option<String>,
//     #[field(desc = "doc type")]
//     pub doc_type: Option<DocumentType>,
//     #[field(desc = "file size")]
//     pub file_size_in_bytes: Option<i32>,
//     #[field(desc = "name")]
//     pub name: Option<String>,
//     #[field(desc = "url")]
//     pub url: Option<String>,
// }

// impl Document {
//     pub fn is_doc_type(&self, doc_type: DocumentType) -> bool {
//         self.doc_type == Some(doc_type)
//     }
// }

// impl From<IndexResult> for Document {
//     fn from(r: IndexResult) -> Self {
//         Self {
//             product_name: r.product_name,
//             active_substances: Some(r.substance_name),
//             title: Some(r.title),
//             created: r.created,
//             doc_type: Some(r.doc_type),
//             file_size_in_bytes: Some(r.metadata_storage_size),
//             name: Some(r.file_name),
//             url: Some(r.metadata_storage_path),
//             highlights: match r.highlights {
//                 Some(a) => Some(a.content),
//                 _ => None,
//             },
//         }
//     }
// }

// pagination! {Documents, DocumentEdge, Document}

// fn get_document_edges(docs: Vec<Document>, offset: i32) -> Vec<DocumentEdge> {
//     docs.into_iter()
//         .enumerate()
//         .map(|(i, document)| DocumentEdge {
//             node: document,
//             cursor: base64::encode((i as i32 + offset).to_string()),
//         })
//         .collect()
// }

// fn get_documents_from_edges(edges: Vec<DocumentEdge>, offset: i32, total_count: i32) -> Documents {
//     let result_count = edges.len() as i32;

//     Documents {
//         edges,
//         total_count,
//         page_info: PageInfo::build(offset, result_count, total_count),
//     }
// }

// pub fn get_documents_graph_from_documents_vector(
//     docs: Vec<Document>,
//     offset: i32,
//     total_count: i32,
// ) -> Documents {
//     let edges = get_document_edges(docs, offset);
//     get_documents_from_edges(edges, offset, total_count)
// }

// pub struct AzureDocumentResult {
//     docs: Vec<Document>,
//     offset: i32,
//     total_count: i32,
// }

// impl Into<Documents> for AzureDocumentResult {
//     fn into(self) -> Documents {
//         get_documents_graph_from_documents_vector(self.docs, self.offset, self.total_count)
//     }
// }

// pub async fn get_documents(
//     client: &impl Search,
//     search: &str,
//     first: Option<i32>,
//     offset: i32,
//     document_types: Option<Vec<DocumentType>>,
//     product_name: Option<&str>,
// ) -> Result<AzureDocumentResult, anyhow::Error> {
//     let result_count = first.unwrap_or(10);

//     let azure_result = client
//         .search_with_pagination_and_filter(
//             &search,
//             search_client::AzurePagination {
//                 result_count,
//                 offset,
//             },
//             true,
//             build_filter(document_types, product_name).as_deref(),
//         )
//         .await?;

//     let docs = azure_result
//         .search_results
//         .into_iter()
//         .map(Document::from)
//         .collect();

//     let total_count = azure_result.count.unwrap_or(0);

//     Ok(AzureDocumentResult {
//         docs,
//         total_count,
//         offset,
//     })
// }

// fn build_filter(
//     document_types: Option<Vec<DocumentType>>,
//     product_name: Option<&str>,
// ) -> Option<String> {
//     let docs_filter = document_types.and_then(build_document_types_filter);
//     let products_filter = product_name.map(build_product_name_filter);

//     let filters: Vec<String> = products_filter.into_iter().chain(docs_filter).collect();

//     match &filters[..] {
//         [] => None,
//         [filter] => Some(filter.clone()),
//         _ => Some(format!("({})", filters.join(" and "))),
//     }
// }

// fn build_document_types_filter(document_types: Vec<DocumentType>) -> Option<String> {
//     if document_types.is_empty() {
//         return None;
//     }

//     Some(format!(
//         "({})",
//         document_types
//             .into_iter()
//             .map(|document_type| format!("doc_type eq '{}'", document_type))
//             .collect::<Vec<_>>()
//             .join(" or ")
//     ))
// }

// fn build_product_name_filter(product_name: &str) -> String {
//     format!("(product_name eq '{}')", product_name)
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use async_trait::async_trait;
//     use search_client::models::{FacetResults, IndexResults};
//     use test_case::test_case;
//     use tokio_test::block_on;

//     struct TestAzureSearchClient {
//         pub search_results: Vec<IndexResult>,
//     }

//     impl TestAzureSearchClient {
//         fn new(search_results: Vec<IndexResult>) -> Self {
//             Self { search_results }
//         }
//     }

//     #[async_trait]
//     impl Search for TestAzureSearchClient {
//         async fn search(&self, _search_term: &str) -> Result<IndexResults, reqwest::Error> {
//             unimplemented!()
//         }
//         async fn search_with_pagination(
//             &self,
//             _search_term: &str,
//             _pagination: search_client::AzurePagination,
//             _include_count: bool,
//         ) -> Result<IndexResults, reqwest::Error> {
//             unimplemented!();
//         }
//         async fn search_with_pagination_and_filter(
//             &self,
//             _search_term: &str,
//             _pagination: search_client::AzurePagination,
//             _include_count: bool,
//             _filter: Option<&str>,
//         ) -> Result<IndexResults, reqwest::Error> {
//             Ok(IndexResults {
//                 search_results: self.search_results.clone(),
//                 context: String::from(""),
//                 count: Some(1234),
//             })
//         }
//         async fn search_by_facet_field(
//             &self,
//             _field_name: &str,
//             _field_value: &str,
//         ) -> Result<FacetResults, reqwest::Error> {
//             unimplemented!()
//         }
//         async fn filter_by_collection_field(
//             &self,
//             _field_name: &str,
//             _field_value: &str,
//         ) -> Result<IndexResults, reqwest::Error> {
//             unimplemented!()
//         }
//         async fn filter_by_non_collection_field(
//             &self,
//             _field_name: &str,
//             _field_value: &str,
//         ) -> Result<IndexResults, reqwest::Error> {
//             unimplemented!()
//         }
//     }

//     fn given_a_search_result(product_name: &str) -> IndexResult {
//         IndexResult {
//             product_name: Some(product_name.to_string()),
//             doc_type: DocumentType::Spc,
//             file_name: "our_id".to_string(),
//             metadata_storage_name: "storage_name".to_string(),
//             metadata_storage_path: "test/path".to_string(),
//             substance_name: vec!["substance".to_string()],
//             title: "title".to_string(),
//             created: None,
//             facets: vec!["facet".to_string()],
//             keywords: None,
//             metadata_storage_size: 300,
//             release_state: None,
//             rev_label: None,
//             suggestions: vec!["suggestion".to_string()],
//             score: 1.0,
//             highlights: None,
//         }
//     }

//     fn given_first_page_of_search_results() -> Vec<IndexResult> {
//         vec![
//             given_a_search_result("first"),
//             given_a_search_result("second"),
//             given_a_search_result("third"),
//             given_a_search_result("fourth"),
//             given_a_search_result("fifth"),
//             given_a_search_result("sixth"),
//             given_a_search_result("seventh"),
//             given_a_search_result("eighth"),
//             given_a_search_result("ninth"),
//             given_a_search_result("tenth"),
//         ]
//     }

//     fn given_last_page_of_search_results() -> Vec<IndexResult> {
//         vec![
//             given_a_search_result("fourth last"),
//             given_a_search_result("third last"),
//             given_a_search_result("second last"),
//             given_a_search_result("last"),
//         ]
//     }

//     fn given_a_search_client(search_results: &[IndexResult]) -> TestAzureSearchClient {
//         TestAzureSearchClient::new(search_results.to_owned())
//     }

//     fn when_we_get_the_first_page_of_documents(search_client: impl Search) -> Documents {
//         block_on(get_documents(
//             &search_client,
//             "Search string",
//             None,
//             0,
//             None,
//             None,
//         ))
//         .unwrap()
//         .into()
//     }

//     fn when_we_get_the_last_page_of_documents(search_client: impl Search) -> Documents {
//         block_on(get_documents(
//             &search_client,
//             "Search string",
//             None,
//             1230,
//             None,
//             None,
//         ))
//         .unwrap()
//         .into()
//     }

//     fn then_we_have_the_first_page(documents_response: &Documents) {
//         let expected_names = vec![
//             "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
//             "tenth",
//         ];
//         let edges = &documents_response.edges;
//         let actual_names = edges
//             .iter()
//             .map(|edge| edge.node.product_name.as_ref().unwrap());
//         assert!(actual_names.eq(expected_names));

//         assert_eq!(1234, documents_response.total_count);

//         let expected_page_info = PageInfo {
//             has_previous_page: false,
//             has_next_page: true,
//             start_cursor: base64::encode("0"),
//             end_cursor: base64::encode("9"),
//         };
//         assert_eq!(expected_page_info, documents_response.page_info);
//     }

//     fn then_we_have_the_last_page(documents_response: &Documents) {
//         let expected_names = vec!["fourth last", "third last", "second last", "last"];
//         let edges = &documents_response.edges;
//         let actual_names = edges
//             .iter()
//             .map(|edge| edge.node.product_name.as_ref().unwrap());

//         assert!(actual_names.eq(expected_names));

//         assert_eq!(1234, documents_response.total_count);
//         let expected_page_info = PageInfo {
//             has_previous_page: true,
//             has_next_page: false,
//             start_cursor: base64::encode("1230"),
//             end_cursor: base64::encode("1233"),
//         };
//         assert_eq!(expected_page_info, documents_response.page_info);
//     }

//     #[test]
//     fn test_get_documents_first_page() {
//         let search_results = given_first_page_of_search_results();
//         let search_client = given_a_search_client(&search_results);
//         let response = when_we_get_the_first_page_of_documents(search_client);
//         then_we_have_the_first_page(&response);
//     }

//     #[test]
//     fn test_get_documents_last_page() {
//         let search_results = given_last_page_of_search_results();
//         let search_client = given_a_search_client(&search_results);
//         let response = when_we_get_the_last_page_of_documents(search_client);
//         then_we_have_the_last_page(&response);
//     }

//     #[test_case(None, None, None)]
//     #[test_case(
//         Some(vec![]),
//         None,
//         None
//     )]
//     #[test_case(
//         Some(vec![DocumentType::Spc, DocumentType::Pil,DocumentType::Par,]),
//         Some("IBUPROFEN 100MG CAPLETS"),
//         Some("((product_name eq 'IBUPROFEN 100MG CAPLETS') and (doc_type eq 'Spc' or doc_type eq 'Pil' or doc_type eq 'Par'))")
//     )]
//     #[test_case(
//         Some(vec![DocumentType::Spc,  DocumentType::Pil,DocumentType::Par,]),
//         None,
//         Some("(doc_type eq 'Spc' or doc_type eq 'Pil' or doc_type eq 'Par')")
//     )]
//     #[test_case(
//         None,
//         Some("IBUPROFEN 100MG CAPLETS"),
//         Some("(product_name eq 'IBUPROFEN 100MG CAPLETS')")
//     )]
//     #[test_case(
//         Some(vec![]),
//         Some("IBUPROFEN 100MG CAPLETS"),
//         Some("(product_name eq 'IBUPROFEN 100MG CAPLETS')")
//     )]
//     fn test_build_filter(
//         document_types: Option<Vec<DocumentType>>,
//         product_name: Option<&str>,
//         expected_filter: Option<&str>,
//     ) {
//         assert_eq!(
//             expected_filter.map(|s| s.to_string()),
//             build_filter(document_types, product_name)
//         );
//     }
// }
