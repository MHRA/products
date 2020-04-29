use crate::{document::Document, product::Product};
use search_client::{models::IndexResults, Search};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Substance {
    name: String,
    products: Vec<Product>,
}

impl Substance {
    pub fn new(name: String, products: Vec<Product>) -> Self {
        Self { name, products }
    }
}

#[juniper::graphql_object]
#[graphql(description = "An active ingredient found in medical products")]
impl Substance {
    fn name(&self) -> &str {
        &self.name
    }
    fn products(&self) -> &Vec<Product> {
        &self.products
    }
}

pub async fn get_substances_starting_with_letter(
    client: &impl Search,
    letter: char,
) -> reqwest::Result<Vec<Substance>> {
    let upper_letter = letter.to_ascii_uppercase();

    let azure_result = client
        .filter_by_collection_field("facets", &upper_letter.to_string())
        .await?;

    Ok(format_search_results(azure_result, upper_letter))
}

fn format_search_results(results: IndexResults, letter: char) -> Vec<Substance> {
    let mut substances: BTreeMap<&str, BTreeMap<&str, Vec<Document>>> = BTreeMap::new();

    let letter_string = letter.to_string();

    results
        .search_results
        .iter()
        .filter(|result| result.facets.first() == Some(&letter_string))
        .filter_map(|result| {
            let substance_name = result.substance_name.first()?.as_str();
            let product_name = result.product_name.as_ref()?.as_str();

            let doc: Document = result.clone().into();

            Some((substance_name, product_name, doc))
        })
        .for_each(
            |(substance, product, doc)| match substances.get_mut(substance) {
                None => {
                    let mut map = BTreeMap::new();
                    map.insert(product, vec![doc]);
                    substances.insert(substance, map);
                }
                Some(map) => match map.get_mut(product) {
                    Some(docs) => {
                        docs.push(doc);
                    }
                    None => {
                        map.insert(product, vec![doc]);
                    }
                },
            },
        );

    substances
        .iter()
        .map(|(&substance, prods)| {
            let products = prods
                .into_iter()
                .map(|(&name, docs)| Product::new(name.into(), docs.clone()))
                .collect();

            Substance::new(substance.into(), products)
        })
        .collect()
}
