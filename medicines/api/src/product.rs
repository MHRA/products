use crate::{
    document::{self, get_documents, Document, DocumentType},
    substance::Substance,
};
use juniper::FieldResult;
use search_client::{models::IndexResult, Search};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Product {
    name: String,
}

impl Product {
    pub fn new(name: String) -> Self {
        Self { name }
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
        offset: i32,
        document_types: Option<Vec<DocumentType>>,
    ) -> FieldResult<document::Documents> {
        let docs = get_documents(
            &search_client::AzureSearchClient::new(),
            "",
            first,
            offset,
            document_types,
            Some(self.name.clone()),
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

pub fn handle_doc(document: &IndexResult, products: &mut Vec<Product>) {
    match &document.product_name {
        Some(document_product_name) => {
            // Try to find an existing product.
            let existing_product = products
                .iter_mut()
                .find(|product| document_product_name == &product.name);

            if existing_product.is_none() {
                products.push(Product::new(document_product_name.to_owned()));
            }
        }
        None => {}
    }
}

pub async fn get_substance_with_products(
    substance_name: &str,
    client: &impl Search,
) -> Result<Substance, reqwest::Error> {
    let azure_result = client
        .filter_by_collection_field("substance_name", substance_name)
        .await?;

    let mut products = Vec::<Product>::new();
    for document in azure_result.search_results {
        handle_doc(&document, &mut products);
    }

    products.sort();

    Ok(Substance::new(substance_name.to_string(), products))
}

pub async fn get_product(product_name: String) -> Result<Product, reqwest::Error> {
    Ok(Product::new(product_name))
}

#[cfg(test)]
mod test {
    use super::*;

    fn azure_result_factory(product_name: Option<String>) -> IndexResult {
        IndexResult {
            product_name,
            doc_type: "dummy".to_string(),
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
        }
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
        products.push(Product::new("My Cool Product".to_string()));
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
        products.push(Product::new("B".to_owned()));
        products.push(Product::new("C".to_owned()));
        products.push(Product::new("A".to_owned()));
        products.sort();
        assert_eq!(products[0].name, "A");
        assert_eq!(products[1].name, "B");
        assert_eq!(products[2].name, "C");
    }
}
