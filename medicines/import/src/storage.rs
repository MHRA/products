use crate::model::ImportError;
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
    dry_run: bool,
) -> Result<(), ImportError> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");

    if verbosity >= 2 {
        println!("Metadata for file is:");
        println!("{:?}", metadata);
    }

    let empty_metadata = HashMap::new();
    let mut metadata_ref: HashMap<&str, &str> = HashMap::new();
    for (key, val) in metadata {
        metadata_ref.insert(&key, &val);
    }

    let file_name = metadata.get("file_name").unwrap();

    let report_name = metadata.get("report_name").unwrap();
    let report_dir = format!(
        "{}/{}/",
        path.parent().unwrap().to_str().unwrap(),
        &report_name,
    );

    let pdf_file_name = format!("{}.pdf", &file_name);
    let _ = upload_file(
        &pdf_file_name,
        &"application/pdf",
        &report_name,
        &report_dir,
        &"",
        &metadata_ref,
        client,
        &container_name,
        dry_run,
    )
    .await?;

    let html_file_name = format!("{}.html", &file_name);
    let _ = upload_file(
        &html_file_name,
        &"text/html",
        &report_name,
        &report_dir,
        &"",
        &empty_metadata,
        client,
        &container_name,
        dry_run,
    )
    .await?;

    let html_assets_dir = format!("{}{}.fld/", &report_dir, &file_name,);
    let report_images = fs::read_dir(&html_assets_dir)
        .map_err(|e| ImportError::FileOpenError(e.to_string()))?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().unwrap_or_default() == "jpg")
        .collect::<Vec<fs::DirEntry>>();

    for image in report_images {
        upload_file(
            image.path().file_name().unwrap().to_str().unwrap(),
            &"image/jpeg",
            &report_name,
            &html_assets_dir,
            &"assets/",
            &empty_metadata,
            &client,
            &container_name,
            dry_run,
        )
        .await?;
    }

    trace!("created {}", report_name);
    Ok(())
}

async fn upload_file(
    file_name: &str,
    content_type: &str,
    report_name: &str,
    local_dir_path: &str,
    azure_storage_prefix: &str,
    metadata: &HashMap<&str, &str>,
    client: &Box<dyn Client>,
    container_name: &str,
    dry_run: bool,
) -> Result<(), ImportError> {
    let local_file_path = format!("{}{}", &local_dir_path, &file_name,);
    let file_contents =
        fs::read(local_file_path).map_err(|e| ImportError::FileOpenError(e.to_string()))?;
    let file_digest = md5::compute(&file_contents[..]);
    let file_azure_storage_name =
        format!("{}/{}{}", &report_name, &azure_storage_prefix, &file_name);

    if dry_run {
        return Ok(());
    }

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&file_azure_storage_name)
        .with_content_type(content_type)
        .with_metadata(&metadata)
        .with_body(&file_contents)
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(ImportError::AzureError)?;

    Ok(())
}

pub async fn upload_index_file(
    file_contents: &[u8],
    client: &Box<dyn Client>,
    dry_run: bool,
) -> Result<(), ImportError> {
    let container_name =
        std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER first!");
    let file_digest = md5::compute(&file_contents[..]);

    if dry_run {
        return Ok(());
    }

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&"upload-index.txt")
        .with_content_type("text/text")
        .with_body(file_contents)
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(ImportError::AzureError)?;

    Ok(())
}
