use crate::{
    azure_context::AzureContext,
    query_objects::products::document::{
        self, get_documents, get_documents_graph_from_documents_vector, Document,
    },
};
use anyhow::anyhow;
use async_graphql::{Context, FieldResult, Object};
use search_client::models::DocumentType;

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

#[Object(desc = "A medical product containing active ingredients")]
impl Product {
    #[field(desc = "Name")]
    async fn name(&self) -> &str {
        &self.name
    }

    #[field(desc = "Documents related to product")]
    async fn documents(
        &self,
        context: &Context<'_>,
        first: Option<i32>,
        offset: Option<i32>,
        document_types: Option<Vec<DocumentType>>,
    ) -> FieldResult<document::Documents> {
        let context = context.data::<AzureContext>()?;

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
            get_documents(
                &context.products_client,
                "",
                first,
                offset,
                document_types,
                Some(&self.name),
            )
            .await
            .map(Into::into)
            .map_err(|e| {
                tracing::error!(
                    "Error fetching documents from Azure search service: {:?}",
                    e
                );
                anyhow!("Error retrieving results").into()
            })
        }
    }
}

pub fn handle_doc(document: &Document, products: &mut Vec<Product>) {
    if let Some(document_product_name) = document.product_name.as_ref() {
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
            doc_type: DocumentType::Spc,
            created: Some("yes".to_string()),
            facets: Vec::new(),
            file_name: "README.markdown".to_string(),
            highlights: None,
            keywords: None,
            territory: Some("UK".to_string()),
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

        result.into()
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
