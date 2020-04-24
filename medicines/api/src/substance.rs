use crate::{pagination, pagination::PageInfo, product::Product};
use base64;
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

pagination! {Substances, SubstanceEdge, Substance}

pub async fn get_substances_starting_with_letter(
    client: &impl Search,
    letter: char,
) -> reqwest::Result<Substances> {
    let upper_letter = letter.to_ascii_uppercase();

    let azure_result = client
        .filter_by_field("facets", &upper_letter.to_string())
        .await?;

    Ok(format_search_results(azure_result, upper_letter))
}

fn format_search_results(results: IndexResults, letter: char) -> Substances {
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

    let edges = substances
        .iter()
        .enumerate()
        .map(|(i, (&substance, prods))| {
            let products = prods
                .iter()
                .map(|(&name, &document_count)| Product::new(name.into(), document_count))
                .collect();

            let node = Substance::new(substance.into(), products);

            SubstanceEdge {
                node,
                cursor: base64::encode(i.to_string()),
            }
        })
        .collect();

    Substances {
        edges,
        page_info: PageInfo {
            has_previous_page: false,
            has_next_page: false,
        },
    }
}

pub async fn get_substances(first: i32) -> Substances {
    let substances: [&str; 1000] = ["Ibuprofen"; 1000];
    let edges = substances
        .iter()
        .take(first as usize)
        .map(|&x| Substance::new(x.to_string(), vec![]))
        .map(|y| SubstanceEdge {
            node: y,
            cursor: "cursor".to_owned(),
        })
        .collect();

    Substances {
        edges,
        page_info: PageInfo {
            has_previous_page: false,
            has_next_page: first < 1000,
        },
    }
}
