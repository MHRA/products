use crate::query_objects::products::product::{handle_doc, Product};
use async_graphql::SimpleObject;
use search_client::{models::IndexResults, Search};

#[SimpleObject(desc = "An active ingredient found in medical products")]
#[derive(Debug, PartialEq)]
pub struct Substance {
    name: String,
    products: Vec<Product>,
}

impl Substance {
    pub fn new(name: String, products: Vec<Product>) -> Self {
        Self { name, products }
    }
}

pub async fn get_substance_with_products(
    substance_name: &str,
    client: &impl Search,
) -> Result<Substance, anyhow::Error> {
    let azure_result = client
        .filter_by_collection_field::<IndexResults>("substance_name", substance_name)
        .await?;

    let mut products = Vec::<Product>::new();

    for result in azure_result.search_results {
        let document = result.into();

        handle_doc(&document, &mut products);
    }

    products.sort();

    Ok(Substance::new(substance_name.to_string(), products))
}
