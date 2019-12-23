use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

pub fn read_hashfile(file: File) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let reader = BufReader::new(file);
    let hash_list = serde_json::from_reader(reader)?;

    Ok(hash_list)
}