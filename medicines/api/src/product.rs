use crate::{
    document::{self, get_documents_graph_from_documents_vector, Document},
    substance::Substance,
};
use search_client::{models::IndexResult, Search};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Product {
    name: String,
    document_count: i32,
    documents: Vec<Document>,
}

impl Product {
    pub fn new(name: String, documents: Vec<Document>) -> Self {
        Self {
            name,
            document_count: documents.len() as i32,
            documents,
        }
    }

    pub fn add(&mut self, document: Document) {
        self.documents.push(document);
        self.document_count += 1;
    }
}

#[juniper::graphql_object]
#[graphql(description = "A medical product containing active ingredients")]
impl Product {
    fn name(&self) -> &str {
        &self.name
    }
    fn documents(&self, first: Option<i32>, skip: Option<i32>) -> document::Documents {
        let docs = self.documents.clone().into_iter();
        let docs = match first {
            Some(t) => docs.take(t as usize).collect(),
            None => docs.collect(),
        };

        let offset = match skip {
            Some(a) => a,
            None => 0,
        };

        get_documents_graph_from_documents_vector(docs, offset, self.document_count)
    }
}

pub fn handle_doc(document: &IndexResult, products: &mut Vec<Product>) {
    match &document.product_name {
        Some(document_product_name) => {
            // Try to find an existing product.
            let existing_product = products
                .iter_mut()
                .find(|product| document_product_name == &product.name);

            match existing_product {
                Some(existing_product) => existing_product.add(document.to_owned().into()),
                None => products.push(Product::new(
                    document_product_name.to_owned(),
                    vec![document.to_owned().into()],
                )),
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

pub async fn get_product(
    product_name: String,
    client: &impl Search,
) -> Result<Product, reqwest::Error> {
    let azure_result = client
        .filter_by_non_collection_field("product_name", &product_name)
        .await?;

    Ok(Product::new(
        product_name,
        azure_result
            .search_results
            .into_iter()
            .map(Into::<Document>::into)
            .collect(),
    ))
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
        assert_eq!(products[0].document_count, 1);
    }

    fn gimme_x_docs(x: i32) -> Vec<Document> {
        let mut docs: Vec<Document> = vec![];
        for _ in 0..x {
            docs.push(azure_result_factory(Some("Craig's Cool Product".to_string())).into())
        }
        docs
    }

    #[test]
    fn test_handle_doc_with_existing_product() {
        let doc = azure_result_factory(Some("My Cool Product".to_string()));
        let mut products = Vec::<Product>::new();
        products.push(Product::new("My Cool Product".to_string(), gimme_x_docs(5)));
        handle_doc(&doc, &mut products);
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].name, "My Cool Product".to_string());
        assert_eq!(products[0].document_count, 6);
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
        products.push(Product::new("B".to_owned(), gimme_x_docs(1)));
        products.push(Product::new("C".to_owned(), gimme_x_docs(1)));
        products.push(Product::new("A".to_owned(), gimme_x_docs(1)));
        products.sort();
        assert_eq!(products[0].name, "A");
        assert_eq!(products[1].name, "B");
        assert_eq!(products[2].name, "C");
    }
}
