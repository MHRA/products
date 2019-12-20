use crate::model::Record;
use csv;
use std::{
    collections::HashMap,
    fs,
    fs::{DirEntry, File},
    io::BufReader,
    path::Path,
};

pub fn find_csv(dir: &Path) -> Result<File, std::io::Error> {
    if let Some(Ok(f)) =
        fs::read_dir(dir)?.find(|f| is_csv(f.as_ref().expect("No CSV file found!")))
    {
        println!("Found CSV file: {:?}", f);
        File::open(&f.path())
    } else {
        panic!("shouldn't get here");
    }
}

pub fn load_csv(file: File) -> Result<HashMap<String, Record>, std::io::Error> {
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    Ok(rdr
        .deserialize()
        .map(|r: Result<Record, csv::Error>| {
            let r = r.expect("Failed to deserialize");
            (r.filename.clone().to_lowercase(), r)
        })
        .collect::<HashMap<String, Record>>())
}

fn is_csv(f: &DirEntry) -> bool {
    "csv" == f.path().extension().unwrap_or_default()
}
