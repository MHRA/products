use crate::product::Product;
use search_client::{models::IndexResults, Search};
use std::collections::BTreeMap;

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(description = "An active ingredient found in medical products")]
pub struct Substance {
    name: String,
    products: Vec<Product>,
}

impl Substance {
    pub fn new(name: String, products: Vec<Product>) -> Self {
        Self { name, products }
    }
}

pub async fn get_substances_starting_with_letter(
    client: &impl Search,
    letter: char,
) -> reqwest::Result<Vec<Substance>> {
    let upper_letter = letter.to_ascii_uppercase();

    let azure_result = client
        .filter_by_field("facets", &upper_letter.to_string())
        .await?;

    Ok(format_search_results(azure_result, upper_letter))
}

fn format_search_results(results: IndexResults, letter: char) -> Vec<Substance> {
    let mut substances: BTreeMap<&str, BTreeMap<&str, i32>> = BTreeMap::new();

    let letter_string = letter.to_string();

    results
        .search_results
        .iter()
        .filter(|result| result.facets.first() == Some(&letter_string))
        .filter_map(|result| {
            let substance_name = result.substance_name.first()?.as_str();
            let product_name = result.product_name.as_ref()?.as_str();

            Some((substance_name, product_name))
        })
        .for_each(|(substance, product)| match substances.get_mut(substance) {
            None => {
                let mut map = BTreeMap::new();
                map.insert(product, 1);
                substances.insert(substance, map);
            }
            Some(map) => {
                let count = map.get(product).copied().unwrap_or_default();

                map.insert(product, count + 1);
            }
        });

    substances
        .iter()
        .map(|(&substance, prods)| {
            let products = prods
                .iter()
                .map(|(&name, &document_count)| Product::new(name.into(), document_count))
                .collect();

            Substance::new(substance.into(), products)
        })
        .collect()
}
