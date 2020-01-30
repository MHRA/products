use juniper::GraphQLObject;
use crate::azure_search::{azure_search};
use crate::{pagination};
use crate::pagination::{PageInfo};

#[derive(GraphQLObject)]
#[graphql(description = "A medical product containing active ingredients")]
pub struct Product {
    id: String,
    name: Option<String>,
    pdf_url: Option<String>,
    substances: Option<Vec<String>>,
    file_name: Option<String>,
    release_state: Option<String>,
    doc_type: Option<String>,
    title: Option<String>,
}

pagination!{Products, ProductEdge, Product}

pub async fn get_product(search_term: String) -> Option<Products> {
    let azure_result = azure_search(search_term).await;
    let r = match azure_result {
        Ok(n)  => n,
        Err(_) => return None,
    };

    let products = r.value.into_iter().map(|x| Product {
        id: x.metadata_storage_name,
        name: x.product_name,
        pdf_url: Some(x.metadata_storage_path),
        substances: Some(x.substance_name),
        file_name: Some(x.file_name),
        release_state: None,
        doc_type: Some(x.doc_type),
        title: Some(x.title),
    });

    let e = products
    .map(|y| ProductEdge {
        node: y,
        cursor: "cursor".to_owned()
    }).collect();

    Some(Products {
        edges: e,
        page_info: PageInfo {has_previous_page:false,has_next_page:false}
    })
}
