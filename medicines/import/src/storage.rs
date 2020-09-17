use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    BlobNameSupport, BodySupport, ContainerNameSupport, ContentMD5Support, ContentTypeSupport,
    MetadataSupport,
};
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub async fn upload(
    client: &Box<dyn Client>,
    path: &Path,
    metadata: &HashMap<&str, &str>,
    verbosity: i8,
) -> Result<(), AzureError> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");

    if verbosity >= 2 {
        println!("Metadata for file is:");
        println!("{:?}", metadata);
    }

    let report_name = metadata.get("report_name").unwrap();
    let report_dir = format!("{:?}/{}/", path.parent().unwrap(), &report_name,);
    let pdf_file_name = format!("{}.pdf", metadata.get("file_name").unwrap());
    let pdf_file_path = format!("{}{}", &report_dir, &pdf_file_name,);
    let pdf_file = fs::read(pdf_file_path)?;
    let pdf_file_digest = md5::compute(&pdf_file[..]);
    let pdf_file_storage_name = format!("{}/{}", &report_name, &pdf_file_name);

    let html_file_name = format!("{}.html", metadata.get("file_name").unwrap());
    let html_file_path = format!("{}{}.html", &report_dir, &html_file_name,);
    let html_file = fs::read(html_file_path)?;
    let html_file_storage_name = format!("{}/{}", &report_name, &html_file_name);

    let html_assets_dir = format!("{}{}.fld", &report_dir, metadata.get("file_name").unwrap(),);

    let report_images = fs::read_dir(html_assets_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().unwrap_or_default() == "jpg")
        .filter_map(|entry| {
            if let Ok(file) = fs::read(entry.path()) {
                return Some(file);
            }
            None
        })
        .collect::<Vec<Vec<u8>>>();

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&pdf_file_storage_name)
        .with_content_type("application/pdf")
        .with_metadata(metadata)
        .with_body(pdf_file)
        .with_content_md5(&pdf_file_digest[..])
        .finalize()
        .await?;
    // calculate md5 too!
    // let digest = md5::compute(&data[..]);
    // let future = client
    //     .put_block_blob()
    //     .with_container_name(&container_name)
    //     .with_blob_name(&blob_name)
    //     .with_content_type("application/pdf")
    //     .with_metadata(metadata)
    //     .with_body(&data[..])
    //     .with_content_md5(&digest[..])
    //     .finalize();

    // trace!("created {:?}", pdf_file_path);
    Ok(())
}
