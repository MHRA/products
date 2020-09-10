use crate::product::Product;
use async_graphql::SimpleObject;
use search_client::{models::FacetResults, Search};

#[SimpleObject(desc = "An active ingredient found in medical products")]
#[derive(Debug, PartialEq)]
pub struct Substance {
    name: String,
    products: Vec<Product>,
}

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

impl Substance {
    pub fn new(name: String, products: Vec<Product>) -> Self {
        Self { name, products }
    }
}

pub async fn get_substances_index(
    client: &impl Search,
    letter: char,
) -> anyhow::Result<Vec<SubstanceIndex>> {
    let upper_letter = letter.to_ascii_uppercase();

    let azure_result = client
        .search_by_facet_field("facets", &upper_letter.to_string())
        .await?;

    Ok(format_index_search_results(azure_result))
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct SubstanceName(String);

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct ProductName(String);

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct DocumentCount(i32);

fn format_index_search_results(results: FacetResults) -> Vec<SubstanceIndex> {
    results
        .facet_results
        .facets
        .into_iter()
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
    use search_client::models::{DocumentType, IndexResult, IndexResults};

    fn index_result(product_name: &str, substance_name: &[&str], facets: &[&str]) -> IndexResult {
        IndexResult {
            doc_type: DocumentType::Spc,
            file_name: "CON1587463572172".into(),
            metadata_storage_name: "4e99070c7e5d3682675b2becd972ec44ef35b20c".into(),
            metadata_storage_path: "https://mhraproductsnonprod.blob.core.windows.net/docs/4e99070c7e5d3682675b2becd972ec44ef35b20c".into(),
            product_name: Some(product_name.into()),
            substance_name: substance_name.iter().map(|&s|s.into()).collect(),
            title: "spc-doc_PL 16363-0365.pdf".into(),
            created: None,
            facets: facets.iter().map(|&s|s.into()).collect(),
            keywords: Some("".into()),
            metadata_storage_size: 107842,
            release_state: None,
            rev_label: None,
            suggestions: vec![],
            score: 1.0,
            highlights: None
        }
    }

    #[test]
    fn formats_search_results_containing_multiple_products() {
        let letter = 'Z';

        let zonismade_25mg = index_result(
            "ZONISAMIDE ARISTO 25 MG HARD CAPSULES",
            &["ZONISAMIDE"],
            &[
                "Z",
                "Z, ZONISAMIDE",
                "Z, ZONISAMIDE, ZONISAMIDE ARISTO 25 MG HARD CAPSULES",
            ],
        );
        let zonismade_50mg = index_result(
            "ZONISAMIDE ARISTO 50 MG HARD CAPSULES",
            &["ZONISAMIDE"],
            &[
                "Z",
                "Z, ZONISAMIDE",
                "Z, ZONISAMIDE, ZONISAMIDE ARISTO 50 MG HARD CAPSULES",
            ],
        );
        let zonismade_50mg_repeat = index_result(
            "ZONISAMIDE ARISTO 50 MG HARD CAPSULES",
            &["ZONISAMIDE"],
            &[
                "Z",
                "Z, ZONISAMIDE",
                "Z, ZONISAMIDE, ZONISAMIDE ARISTO 50 MG HARD CAPSULES",
            ],
        );
        let zolmitriptan = index_result(
            "ZOMIG RAPIMELT 2.5 MG ORODISPERSIBLE TABLETS",
            &["ZOLMITRIPTAN"],
            &[
                "Z",
                "Z, ZOLMITRIPTAN",
                "Z, ZOLMITRIPTAN, ZOMIG RAPIMELT 2.5 MG ORODISPERSIBLE TABLETS",
            ],
        );

        let results = IndexResults {
            search_results: vec![
                zonismade_25mg.clone(),
                zonismade_50mg.clone(),
                zonismade_50mg_repeat.clone(),
                zolmitriptan.clone(),
            ],
            context: "https://mhraproductsnonprod.search.windows.net/indexes(\'products-index\')/$metadata#docs(*)".into(),
            count: None
        };

        let zon50: Vec<Document> = vec![zonismade_50mg.into(), zonismade_50mg_repeat.into()];
        let zon25: Vec<Document> = vec![zonismade_25mg.into()];
        let zol: Vec<Document> = vec![zolmitriptan.into()];

        let formatted = format_search_results(results, letter).unwrap();

        let expected = vec![
            Substance::new(
                "ZOLMITRIPTAN".into(),
                vec![Product::new(
                    "ZOMIG RAPIMELT 2.5 MG ORODISPERSIBLE TABLETS".into(),
                    Some(zol),
                )],
            ),
            Substance::new(
                "ZONISAMIDE".into(),
                vec![
                    Product::new("ZONISAMIDE ARISTO 25 MG HARD CAPSULES".into(), Some(zon25)),
                    Product::new("ZONISAMIDE ARISTO 50 MG HARD CAPSULES".into(), Some(zon50)),
                ],
            ),
        ];

        assert_eq!(formatted, expected);
    }

    #[test]
    fn formats_products_that_contain_multiple_substances() {
        let letter = 'Z';

        let index_result = index_result(
            "LAMIVUDINE/ZIDOVUDINE 150 MG/300 MG FILM-COATED TABLETS",
            &["LAMIVUDINE", "ZIDOVUDINE"],
            &[
                "L",
                "L, LAMIVUDINE",
                "L, LAMIVUDINE, LAMIVUDINE/ZIDOVUDINE 150 MG/300 MG FILM-COATED TABLETS",
                "Z",
                "Z, ZIDOVUDINE",
                "Z, ZIDOVUDINE, LAMIVUDINE/ZIDOVUDINE 150 MG/300 MG FILM-COATED TABLETS",
            ],
        );
        let document: Document = index_result.clone().into();

        let results = IndexResults {
            search_results: vec![
                index_result,
            ],
            context: "https://mhraproductsnonprod.search.windows.net/indexes(\'products-index\')/$metadata#docs(*)".into(),
            count: None
        };

        let formatted = format_search_results(results, letter).unwrap();

        let expected = vec![Substance::new(
            "ZIDOVUDINE".into(),
            vec![Product::new(
                "LAMIVUDINE/ZIDOVUDINE 150 MG/300 MG FILM-COATED TABLETS".into(),
                Some(vec![document]),
            )],
        )];

        assert_eq!(formatted, expected);
    }
}
