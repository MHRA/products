use crate::{
    document::{self, get_documents, get_documents_graph_from_documents_vector, Document},
    document_type::DocumentType,
    substance::Substance,
};
use juniper::FieldResult;
use search_client::Search;
use std::convert::TryInto;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Product {
    name: String,
    documents: Option<Vec<Document>>,
}

impl Product {
    pub fn new(name: String, documents: Option<Vec<Document>>) -> Self {
        Self { name, documents }
    }

    pub fn add(&mut self, document: Document) {
        if let Some(ref mut v) = self.documents {
            v.push(document);
        } else {
            self.documents = Some(vec![document])
        }
    }
}

#[juniper::graphql_object]
#[graphql(description = "A medical product containing active ingredients")]
impl Product {
    fn name(&self) -> &str {
        &self.name
    }

    async fn documents(
        &self,
        first: Option<i32>,
        offset: Option<i32>,
        document_types: Option<Vec<DocumentType>>,
    ) -> FieldResult<document::Documents> {
        let offset = match offset {
            Some(a) => a,
            None => 0,
        };

        if let Some(docs) = self.documents.clone() {
            let docs = match document_types {
                Some(document_types) => docs
                    .into_iter()
                    .filter(|x| document_types.iter().any(|&f| x.is_doc_type(f)))
                    .collect(),
                None => docs,
            };

            let total_count = docs.len() as i32;

            let docs = match first {
                Some(t) => docs.into_iter().take(t as usize).collect(),
                None => docs,
            };

            Ok(get_documents_graph_from_documents_vector(
                docs,
                offset,
                total_count,
            ))
        } else {
            let docs = get_documents(
                &search_client::AzureSearchClient::new(),
                "",
                first,
                offset,
                document_types,
                Some(&self.name),
            )
            .await
            .map_err(|e| {
                tracing::error!(
                    "Error fetching documents from Azure search service: {:?}",
                    e
                );
                juniper::FieldError::new("Error fetching documents", juniper::Value::null())
            })?
            .into();
            Ok(docs)
        }
    }
}

pub fn handle_doc(document: &Document, products: &mut Vec<Product>) {
    if let Some(document_product_name) = document.product_name() {
        // Try to find an existing product.
        let existing_product = products
            .iter_mut()
            .find(|product| document_product_name == &product.name);

        match existing_product {
            Some(existing_product) => existing_product.add(document.to_owned()),
            None => products.push(Product::new(
                document_product_name.to_owned(),
                Some(vec![document.to_owned()]),
            )),
        }
    }
}

pub async fn get_substance_with_products(
    substance_name: &str,
    client: &impl Search,
) -> Result<Substance, anyhow::Error> {
    let azure_result = client
        .filter_by_collection_field("substance_name", substance_name)
        .await?;

    let docs = azure_result
        .search_results
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Document>, _>>()?;

    let mut products = Vec::<Product>::new();

    for document in docs {
        handle_doc(&document, &mut products);
    }

    products.sort();

    Ok(Substance::new(substance_name.to_string(), products))
}

pub async fn get_product(product_name: String) -> Result<Product, reqwest::Error> {
    Ok(Product::new(product_name, None))
}

#[cfg(test)]
mod test {
    use super::*;
    use search_client::models::IndexResult;

    fn azure_result_factory(product_name: Option<String>) -> Document {
        let result = IndexResult {
            product_name,
            doc_type: "SPC".to_string(),
            created: Some("yes".to_string()),
            facets: Vec::new(),
            file_name: "README.markdown".to_string(),
            highlights: None,
            keywords: None,
            metadata_storage_name: "dummy".to_string(),
            metadata_storage_path: "/".to_string(),
            metadata_storage_size: 0,
            release_state: Some("solid".to_string()),
            rev_label: None,
            score: -0.0,
            substance_name: Vec::new(),
            suggestions: Vec::new(),
            title: "dummy's guide to medicines".to_string(),
        };

        result.try_into().unwrap()
    }

    #[test]
    fn test_handle_doc_with_new_product() {
        let doc = azure_result_factory(Some("My Cool Product".to_string()));
        let mut products = Vec::<Product>::new();
        handle_doc(&doc, &mut products);
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].name, "My Cool Product".to_string());
    }

    #[test]
    fn test_handle_doc_with_existing_product() {
        let doc = azure_result_factory(Some("My Cool Product".to_string()));
        let mut products = Vec::<Product>::new();
        products.push(Product::new("My Cool Product".to_string(), None));
        handle_doc(&doc, &mut products);
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].name, "My Cool Product".to_string());
    }

    #[test]
    fn test_handle_doc_with_no_product_name() {
        let doc = azure_result_factory(None);
        let mut products = Vec::<Product>::new();
        handle_doc(&doc, &mut products);
        assert_eq!(products.len(), 0);
    }

    #[test]
    fn test_sort_products() {
        let mut products = Vec::<Product>::new();
        products.push(Product::new("B".to_owned(), None));
        products.push(Product::new("C".to_owned(), None));
        products.push(Product::new("A".to_owned(), None));
        products.sort();
        assert_eq!(products[0].name, "A");
        assert_eq!(products[1].name, "B");
        assert_eq!(products[2].name, "C");
    }
}
