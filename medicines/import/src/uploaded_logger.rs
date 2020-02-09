use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub fn get_uploaded_files() -> Vec<String> {
  let file = BufReader::new(File::open("./files_written.txt").unwrap());
  let mut file_names = Vec::new();
  for line in file.lines() {
    let my_line = line.unwrap();
    println!("{}", my_line);
    file_names.push(my_line);
  }
  return file_names;
}

pub fn log_uploaded_file(file_name: &String) {
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("./files_written.txt")
    .unwrap();

  if let Err(e) = writeln!(file, "{}", file_name) {
    eprintln!("Couldn't write to file: {}", e);
  }
}
