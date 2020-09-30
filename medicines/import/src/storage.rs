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

    let report_name = metadata.get("report_name").unwrap();
    let report_dir = format!(
        "{}/{}/",
        path.parent().unwrap().to_str().unwrap(),
        &report_name,
    );

    let report_file_name = metadata.get("file_name").unwrap();
    let mut pdf_file_path =
        Path::new(&format!("{}{}.pdf", &report_dir, &report_file_name)).to_owned();
    let mut html_file_path =
        Path::new(&format!("{}{}.html", &report_dir, &report_file_name)).to_owned();
    let mut asset_dir =
        Path::new(&format!("{}{}_files/", &report_dir, &report_file_name,)).to_owned();

    let dir_entries = fs::read_dir(&report_dir)
        .map_err(|e| ImportError::FileOpenError(e.to_string()))?
        .filter_map(Result::ok);
    for entry in dir_entries {
        if entry.file_type().unwrap().is_dir() {
            asset_dir = entry.path();
            continue;
        }

        let entry_file_name = entry
            .file_name()
            .as_os_str()
            .to_str()
            .unwrap_or_default()
            .to_string();

        if entry_file_name.ends_with("pdf") {
            pdf_file_path = entry.path();
        } else if entry_file_name.ends_with("html") {
            html_file_path = entry.path();
        }
    }

    let _ = upload_file(
        &pdf_file_path,
        &format!("{}/{}.pdf", &report_name, &report_name),
        &"application/pdf",
        &metadata_ref,
        client,
        &container_name,
        dry_run,
    )
    .await?;

    let _ = upload_file(
        &html_file_path,
        &format!("{}/{}.html", &report_name, &report_name),
        &"text/html",
        &empty_metadata,
        client,
        &container_name,
        dry_run,
    )
    .await?;

    // let report_images = fs::read_dir(&asset_dir)
    //     .map_err(|e| ImportError::FileOpenError(e.to_string()))?
    //     .filter_map(Result::ok)
    //     .filter(|entry| {
    //         let path = entry.path();
    //         let extension = path.extension().unwrap_or_default();
    //         extension == "jpg" || extension == "png" || extension == "gif"
    //     })
    //     .collect::<Vec<fs::DirEntry>>();

    // for image in report_images {
    //     upload_file(
    //         image.path().file_name().unwrap().to_str().unwrap(),
    //         &"image/jpeg",
    //         &report_name,
    //         &asset_dir,
    //         &"assets/",
    //         &empty_metadata,
    //         &client,
    //         &container_name,
    //         dry_run,
    //     )
    //     .await?;
    // }

    trace!("created {}", report_name);
    Ok(())
}

async fn upload_file(
    local_file_path: &Path,
    azure_storage_name: &str,
    content_type: &str,
    metadata: &HashMap<&str, &str>,
    client: &Box<dyn Client>,
    container_name: &str,
    dry_run: bool,
) -> Result<(), ImportError> {
    let file_contents =
        fs::read(local_file_path).map_err(|e| ImportError::FileOpenError(e.to_string()))?;
    let file_digest = md5::compute(&file_contents[..]);

    if dry_run {
        return Ok(());
    }

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&azure_storage_name)
        .with_content_type(content_type)
        .with_metadata(&metadata)
        .with_body(&file_contents)
        .with_content_md5(&file_digest[..])
        .finalize()
        .await
        .map_err(ImportError::AzureError)?;

    Ok(())
}
