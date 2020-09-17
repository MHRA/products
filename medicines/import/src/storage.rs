use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    BlobNameSupport, BodySupport, ContainerNameSupport, ContentMD5Support, ContentTypeSupport,
    MetadataSupport,
};
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;

use std::{collections::HashMap, fs, path::Path};

pub async fn upload_report(
    client: &Box<dyn Client>,
    path: &Path,
    metadata: &HashMap<String, String>,
    verbosity: i8,
) -> Result<(), AzureError> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");

    if verbosity >= 2 {
        println!("Metadata for file is:");
        println!("{:?}", metadata);
    }

    let metadata_empty = HashMap::new();
    let mut metadata_ref: HashMap<&str, &str> = HashMap::new();
    for (key, val) in metadata {
        metadata_ref.insert(&key, &val);
    }

    let report_name = metadata.get("report_name").unwrap();
    let file_name = metadata.get("file_name").unwrap();
    let reports_dir = format!(
        "{}/{}/",
        path.parent().unwrap().to_str().unwrap(),
        &report_name,
    );

    let pdf_file_name = format!("{}.pdf", &file_name);
    let _ = upload_file(
        &pdf_file_name,
        &"application/pdf",
        &report_name,
        &reports_dir,
        &"",
        &metadata_ref,
        client,
        &container_name,
    )
    .await?;

    let html_file_name = format!("{}.html", &file_name);
    let _ = upload_file(
        &html_file_name,
        &"text/html",
        &report_name,
        &reports_dir,
        &"",
        &metadata_empty,
        client,
        &container_name,
    )
    .await?;

    let html_assets_dir = format!("{}{}.fld/", &reports_dir, &file_name,);
    let report_images = fs::read_dir(&html_assets_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().unwrap_or_default() == "jpg")
        .collect::<Vec<fs::DirEntry>>();

    for image in report_images {
        println!("{}", image.path().file_name().unwrap().to_str().unwrap());
        upload_file(
            image.path().file_name().unwrap().to_str().unwrap(),
            &"image/jpeg",
            &report_name,
            &html_assets_dir,
            &"assets/",
            &metadata_empty,
            &client,
            &container_name,
        )
        .await?;
    }

    trace!("created {}", report_name);
    Ok(())
}

pub async fn upload_file(
    file_name: &str,
    content_type: &str,
    report_name: &str,
    local_dir_path: &str,
    azure_storage_prefix: &str,
    metadata: &HashMap<&str, &str>,
    client: &Box<dyn Client>,
    container_name: &str,
) -> Result<(), AzureError> {
    let local_file_path = format!("{}{}", &local_dir_path, &file_name,);
    let file_contents = fs::read(local_file_path)?;
    let file_digest = md5::compute(&file_contents[..]);
    let file_azure_storage_name =
        format!("{}/{}{}", &report_name, &azure_storage_prefix, &file_name);

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&file_azure_storage_name)
        .with_content_type(content_type)
        .with_metadata(&metadata)
        .with_body(&file_contents)
        .with_content_md5(&file_digest[..])
        .finalize()
        .await?;

    // trace!("created {:?}", pdf_file_path);
    Ok(())
}
