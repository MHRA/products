use async_graphql::SimpleObject;
use search_client::{models::FacetResults, Search};

#[SimpleObject(desc = "An active ingredient found in medical products")]
#[derive(Debug, PartialEq)]
pub struct SubstanceIndex {
    name: String,
    count: i32,
}

impl SubstanceIndex {
    pub fn new(name: String, count: i32) -> Self {
        Self { name, count }
    }
}

pub async fn get_substances_index(
    client: &impl Search,
    letter: char,
) -> anyhow::Result<Vec<SubstanceIndex>> {
    let upper_letter = letter.to_ascii_uppercase().to_string();

    let azure_result = client
        .search_by_facet_field("facets", &upper_letter)
        .await?;

    Ok(format_index_search_results(azure_result, &upper_letter))
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct SubstanceName(String);

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct DocumentCount(i32);

fn format_index_search_results(results: FacetResults, letter: &str) -> Vec<SubstanceIndex> {
    results
        .facet_results
        .facets
        .into_iter()
        .filter(|result| result.value.starts_with(&letter))
        .filter_map(|result| {
            let facets = result.value.split(',').collect::<Vec<&str>>();
            if facets.len() != 2 {
                return None;
            }
            let substance = facets[1];
            Some(SubstanceIndex::new(
                substance.trim().to_string(),
                result.count,
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use search_client::models::{Facet, FacetResult};

    #[test]
    fn formats_substance_index_results() {
        let letter = 'Z';

        let zanamivir_facet = Facet {
            value: "Z, ZANAMIVIR".into(),
            count: 42,
        };

        let zonismade_facet = Facet {
            value: "Z, ZONISAMIDE".into(),
            count: 20,
        };

        let zolmitriptan_facet = Facet {
            value: "Z, ZOLMITRIPTAN".into(),
            count: 30,
        };

        let facet_results = FacetResult {
            facets: vec![zanamivir_facet, zonismade_facet, zolmitriptan_facet],
        };

        let results = FacetResults {
            facet_results,
            search_results: vec![],
            context: "https://mhraproductsnonprod.search.windows.net/indexes(\'products-index\')/$metadata#docs(*)".into(),
        };

        let formatted = format_index_search_results(results, &letter.to_string());

        let zanamivir = SubstanceIndex::new("ZANAMIVIR".into(), 42);
        let zonismade = SubstanceIndex::new("ZONISAMIDE".into(), 20);
        let zolmitriptan = SubstanceIndex::new("ZOLMITRIPTAN".into(), 30);

        let expected = vec![zanamivir, zonismade, zolmitriptan];

        assert_eq!(formatted, expected);
    }
}
