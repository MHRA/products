use juniper::GraphQLObject;
use search_client::{models::IndexResult, Search};
use crate::substance::Substance;

#[derive(GraphQLObject, Eq, Ord, PartialEq, PartialOrd)]
#[graphql(description = "A medical product containing active ingredients")]
pub struct Product {
    name: String,
    document_count: i32,
}

pub fn handle_doc(document: &IndexResult, products: &mut Vec<Product>) {
    match &document.product_name {
        Some(document_product_name) => {
            // Try to find an existing product.
            let existing_product = products
                .into_iter()
                .find(|product| product.name == document_product_name.to_string());
            match existing_product {
                Some(existing_product) => {
                    // Product exists! Increment its document count.
                    existing_product.document_count += 1;
                }
                None => {
                    // Product does not exist! Create it!
                    products.push(Product {
                        name: document_product_name.to_string(),
                        document_count: 1,
                    });
                }
            }
        }
        None => {}
    }
}

pub async fn get_substance_with_products(
    name: &str,
    client: &impl Search,
) -> Substance {
    // Get a list of documents from Azure which are about products containing the
    // substance name.
    let azure_result = client
        .filter_by_field("substance_name", name)
        .await
        .unwrap();

    // Extract a list of products while keeping track of the number of documents that
    // product has.
    let mut products = Vec::<Product>::new();
    for document in azure_result.search_results {
        handle_doc(&document, &mut products);
    }

    products.sort();

    Substance::new(name.to_string(), Some(products))
}

#[cfg(test)]
mod test {
    use super::*;

    fn azure_result_factory(product_name: Option<String>) -> IndexResult {
        IndexResult {
            product_name: product_name,
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

    #[test]
    fn test_handle_doc_with_existing_product() {
        let doc = azure_result_factory(Some("My Cool Product".to_string()));
        let mut products = Vec::<Product>::new();
        products.push(Product {
            name: "My Cool Product".to_string(),
            document_count: 5,
        });
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
        products.push(Product {
            name: "B".to_string(),
            document_count: 1,
        });
        products.push(Product {
            name: "C".to_string(),
            document_count: 1,
        });
        products.push(Product {
            name: "A".to_string(),
            document_count: 1,
        });
        products.sort();
        assert_eq!(products[0].name, "A");
        assert_eq!(products[1].name, "B");
        assert_eq!(products[2].name, "C");
    }
}
