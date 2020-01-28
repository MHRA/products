use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A medical product containing active ingredients")]
pub struct Product {
    id: String,
    name: String,
    pdf_url: Option<String>,
    substances: Option<Vec<String>>,
    file_name: Option<String>,
    release_state: Option<String>,
    doc_type: Option<String>,
    title: Option<String>,
}

pub fn get_product(_id: String) -> Product {
    Product {
        id: _id.to_owned(),
        name: "Nurofen".to_owned(),
        pdf_url: None,
        substances: None,
        file_name: None,
        release_state: None,
        doc_type: None,
        title: None,
    }
}