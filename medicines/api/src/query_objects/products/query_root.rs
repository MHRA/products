use crate::{
    azure_context::AzureContext,
    pagination::get_offset_or_default,
    query_objects::products::{
        document::{get_documents, Documents},
        product::{get_product, Product},
        products_index::{get_products_index, ProductIndex},
        substance::{get_substance_with_products, Substance},
    },
    query_objects::shared::substances_index::{get_substances_index, SubstanceIndex},
};
use anyhow::anyhow;
use async_graphql::{Context, FieldResult, Object};
use search_client::models::{DocumentType, TerritoryType};

pub struct Products {}

#[Object(desc = "Entrypoint for products, where you can find associated SPCs, PILs and PARs")]
impl Products {
    #[field(desc = "Retrieves all products associated with the queried active substance")]
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
                    anyhow!("Error retrieving results").into()
                }),
            None => Err(anyhow::anyhow!(
                "Getting a substance without providing a substance name is not supported."
            )
            .into()),
        }
    }
    #[field(desc = "Retrieves all documents associated with the queried product")]
    async fn product(&self, _context: &Context<'_>, name: String) -> FieldResult<Product> {
        get_product(name).await.map_err(|e| {
            tracing::error!("Error fetching results from Azure search service: {:?}", e);
            anyhow!("Error retrieving results").into()
        })
    }

    #[field(
        desc = "List of active substances beginning with the provided letter that have reports associated with them, along with the count of documents for each"
    )]
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
                anyhow!("Error retrieving results").into()
            })
    }

    #[field(
        desc = "List of products associated with the provided active substances that have reports associated with them, along with the count of documents for each"
    )]
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
                anyhow!("Error retrieving results").into()
            })
    }

    #[allow(clippy::too_many_arguments)]
    #[field(desc = "SPC, PIL and PAR Documents related to products")]
    async fn documents(
        &self,
        context: &Context<'_>,
        search: Option<String>,
        first: Option<i32>,
        skip: Option<i32>,
        after: Option<String>,
        document_types: Option<Vec<DocumentType>>,
        territory_types: Option<Vec<TerritoryType>>,
    ) -> FieldResult<Documents> {
        let context = context.data::<AzureContext>()?;
        let offset = get_offset_or_default(skip, after, 0);

        get_documents(
            &context.products_client,
            search.as_deref().unwrap_or(" "),
            first,
            offset,
            document_types,
            territory_types,
            None,
        )
        .await
        .map(Into::into)
        .map_err(|e| {
            tracing::error!("Error fetching results from Azure search service: {:?}", e);
            anyhow!("Error retrieving results").into()
        })
    }
}
