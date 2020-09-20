use crate::{
    azure_context::AzureContext,
    query_objects::products::{
        document::{get_documents, Documents},
        product::{
            get_product, get_products_index, get_substance_with_products, Product, ProductIndex,
        },
        substance::{get_substances_index, Substance, SubstanceIndex},
    },
};
use async_graphql::{Context, FieldResult, Object};
use search_client::models::DocumentType;

pub struct Products {
    substance: Option<Substance>,
    product: Option<Product>,
    substances_index: Option<Vec<SubstanceIndex>>,
    products_index: Option<Vec<ProductIndex>>,
    documents: Option<Documents>,
}

impl Products {
    pub fn new() -> Self {
        Self {
            substance: None,
            product: None,
            substances_index: None,
            products_index: None,
            documents: None,
        }
    }
}

#[Object(desc = "Entrypoint for products")]
impl Products {
    #[field(desc = "substance")]
    async fn substance(
        &self,
        context: &Context<'_>,
        name: Option<String>,
    ) -> FieldResult<Substance> {
        let context = context.data::<AzureContext>()?;
        match name {
            Some(name) => get_substance_with_products(&name, &context.products_client)
                .await
                .map_err(|e| {
                    tracing::error!("Error fetching results from Azure search service: {:?}", e);
                    e.into()
                }),
            None => Err(anyhow::anyhow!(
                "Getting a substance without providing a substance name is not supported."
            )
            .into()),
        }
    }
    #[field(desc = "product")]
    async fn product(&self, _context: &Context<'_>, name: String) -> FieldResult<Product> {
        get_product(name).await.map_err(|e| {
            tracing::error!("Error fetching results from Azure search service: {:?}", e);
            e.into()
        })
    }

    #[field(desc = "substances_index")]
    async fn substances_index(
        &self,
        context: &Context<'_>,
        letter: String,
    ) -> FieldResult<Vec<SubstanceIndex>> {
        let context = context.data::<AzureContext>()?;
        get_substances_index(&context.products_client, letter.chars().next().unwrap())
            .await
            .map_err(|e| {
                tracing::error!("Error fetching results from Azure search service: {:?}", e);
                e.into()
            })
    }

    #[field(desc = "products_index")]
    async fn products_index(
        &self,
        context: &Context<'_>,
        substance: String,
    ) -> FieldResult<Vec<ProductIndex>> {
        let context = context.data::<AzureContext>()?;
        get_products_index(&context.products_client, &substance)
            .await
            .map_err(|e| {
                tracing::error!("Error fetching results from Azure search service: {:?}", e);
                e.into()
            })
    }

    #[field(desc = "documents")]
    async fn documents(
        &self,
        context: &Context<'_>,
        search: Option<String>,
        first: Option<i32>,
        skip: Option<i32>,
        after: Option<String>,
        document_types: Option<Vec<DocumentType>>,
    ) -> FieldResult<Documents> {
        let context = context.data::<AzureContext>()?;
        let offset = get_offset_or_default(skip, after, 0);

        get_documents(
            &context.products_client,
            search.as_deref().unwrap_or(" "),
            first,
            offset,
            document_types,
            None,
        )
        .await
        .map(Into::into)
        .map_err(|e| {
            tracing::error!("Error fetching results from Azure search service: {:?}", e);
            e.into()
        })
    }
}

fn get_offset_or_default(skip: Option<i32>, after: Option<String>, default: i32) -> i32 {
    match (after, skip) {
        (Some(encoded), _) => match convert_after_to_offset(encoded) {
            Ok(a) => a,
            _ => default,
        },
        (None, Some(offset)) => offset,
        _ => default,
    }
}

fn convert_after_to_offset(encoded: String) -> Result<i32, anyhow::Error> {
    let bytes = base64::decode(encoded)?;
    let string = std::str::from_utf8(&bytes)?;
    Ok(string.parse::<i32>()? + 1)
}
