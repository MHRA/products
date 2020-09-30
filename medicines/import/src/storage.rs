use crate::model::ImportError;
use azure_sdk_core::{
    BlobNameSupport, BodySupport, ContainerNameSupport, ContentMD5Support, ContentTypeSupport,
    MetadataSupport,
};
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;
use std::{collections::HashMap, fs, path::Path, path::PathBuf};

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

    let mut pdf_file_path: Option<PathBuf> = None;
    let mut html_file_path: Option<PathBuf> = None;
    let mut asset_dir: Option<PathBuf> = None;

    let dir_entries = fs::read_dir(&report_dir)
        .map_err(|e| ImportError::FileOpenError(e.to_string()))?
        .filter_map(Result::ok);
    for entry in dir_entries {
        if entry.file_type().unwrap().is_dir() {
            asset_dir = Some(entry.path());
            continue;
        }

        let entry_file_name = entry
            .file_name()
            .as_os_str()
            .to_str()
            .unwrap_or_default()
            .to_string();

        if entry_file_name.ends_with("pdf") {
            pdf_file_path = Some(entry.path());
        } else if entry_file_name.ends_with("html") {
            html_file_path = Some(entry.path());
        }
    }

    match pdf_file_path {
        Some(path) => {
            upload_file(
                &path,
                &format!("{}/{}.pdf", &report_name, &report_name),
                &"application/pdf",
                &metadata_ref,
                client,
                &container_name,
                dry_run,
            )
            .await?;
        }
        None => return Err(ImportError::FileOpenError("PDF file not found".to_string())),
    }

    match html_file_path {
        Some(path) => {
            upload_file(
                &path,
                &format!("{}/{}.html", &report_name, &report_name),
                &"text/html",
                &empty_metadata,
                client,
                &container_name,
                dry_run,
            )
            .await?;
        }
        None => {
            return Err(ImportError::FileOpenError(
                "HTML file not found".to_string(),
            ))
        }
    }

    if let Some(path) = asset_dir {
        let _ = upload_asset_files(
            &path,
            &report_name,
            &empty_metadata,
            client,
            &container_name,
            dry_run,
        )
        .await?;
    }

    trace!("created {}", report_name);
    Ok(())
}

async fn upload_asset_files(
    asset_dir: &Path,
    report_name: &str,
    metadata: &HashMap<&str, &str>,
    client: &Box<dyn Client>,
    container_name: &str,
    dry_run: bool,
) -> Result<(), ImportError> {
    let assets = fs::read_dir(&asset_dir)
        .map_err(|e| ImportError::FileOpenError(e.to_string()))?
        .filter_map(Result::ok)
        .collect::<Vec<fs::DirEntry>>();

    for asset in assets {
        let path = asset.path();
        let extension = path.extension().unwrap_or_default().to_os_string();
        let extension = extension.to_string_lossy().to_owned();
        if extension == "jpg" || extension == "png" || extension == "gif" {
            upload_file(
                &path,
                &format!(
                    "{}/assets/{}",
                    &report_name,
                    asset.file_name().as_os_str().to_str().unwrap_or_default()
                ),
                &get_content_type_from_extension(&extension),
                &metadata,
                &client,
                &container_name,
                dry_run,
            )
            .await?;
        }
    }

    Ok(())
}

fn get_content_type_from_extension(extension: &str) -> String {
    match extension {
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        _ => "image/jpeg".to_string(),
    }
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
