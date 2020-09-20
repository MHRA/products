use crate::query_objects::products::{
    document::{self, get_documents, get_documents_graph_from_documents_vector, Document},
    substance::Substance,
};
use async_graphql::{FieldResult, Object, SimpleObject};
use search_client::{
    models::{DocumentType, FacetResults},
    Search,
};

#[SimpleObject(desc = "A medical product containing active ingredients")]
#[derive(Debug, PartialEq)]
pub struct ProductIndex {
    name: String,
    count: i32,
}

impl ProductIndex {
    pub fn new(name: String, count: i32) -> Self {
        Self { name, count }
    }
}

pub async fn get_products_index(
    client: &impl Search,
    substance: &str,
) -> anyhow::Result<Vec<ProductIndex>> {
    let substance = substance.to_ascii_uppercase();
    let substance_first_letter = substance.chars().next().unwrap_or_default().to_string();
    let facet_match = &format!("{}, {}", &substance_first_letter, &substance);
    let azure_result = client.search_by_facet_field("facets", &facet_match).await?;

    Ok(format_index_search_results(azure_result, &facet_match))
}

fn format_index_search_results(results: FacetResults, facet_match: &str) -> Vec<ProductIndex> {
    results
        .facet_results
        .facets
        .into_iter()
        .filter(|result| result.value.starts_with(facet_match))
        .filter_map(|result| {
            let facets = result.value.splitn(3, ',').collect::<Vec<&str>>();

            if facets.len() != 3 {
                return None;
            }

            let product = facets[2];
            Some(ProductIndex::new(product.trim().to_string(), result.count))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use search_client::models::{Facet, FacetResult, IndexResult};

    #[test]
    fn formats_products_index_results() {
        let substance = String::from("TACROLIMUS");
        let substance_first_letter = substance.chars().next().unwrap_or_default().to_string();
        let facet_match = &format!("{}, {}", &substance_first_letter, &substance);

        let adoport_facet = Facet {
            value: "T, TACROLIMUS, ADOPORT 5MG CAPSULES HARD".into(),
            count: 42,
        };

        let dermitopic_facet = Facet {
            value: "T, TACROLIMUS, DERMITOPIC 0.1% OINTMENT".into(),
            count: 20,
        };

        let tacrolimus_facet = Facet {
            value: "T, TACROLIMUS, TACROLIMUS ACCORD 0.1 % OINTMENT".into(),
            count: 30,
        };

        let facet_results = FacetResult {
            facets: vec![adoport_facet, dermitopic_facet, tacrolimus_facet],
        };

        let results = FacetResults {
            facet_results,
            search_results: vec![],
            context: "https://mhraproductsnonprod.search.windows.net/indexes(\'products-index\')/$metadata#docs(*)".into(),
        };

        let formatted = format_index_search_results(results, &facet_match);

        let adoport = ProductIndex::new("ADOPORT 5MG CAPSULES HARD".into(), 42);
        let dermitopic = ProductIndex::new("DERMITOPIC 0.1% OINTMENT".into(), 20);
        let tacrolimus = ProductIndex::new("TACROLIMUS ACCORD 0.1 % OINTMENT".into(), 30);

        let expected = vec![adoport, dermitopic, tacrolimus];

        assert_eq!(formatted, expected);
    }
}
