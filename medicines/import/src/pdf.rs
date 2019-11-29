use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_pdfs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map(|s| s == "pdf").is_some())
        .filter_map(|entry| {
            entry
                .metadata()
                .map_or_else(|_e| None, |m| Some((entry, m)))
        })
        .filter_map(|(entry, m)| if m.len() > 0 { Some(entry) } else { None })
        .map(|entry| entry.path())
        .collect())
}
