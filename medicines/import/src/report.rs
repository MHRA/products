struct Uploaded {
    file_name: String,
    hash: String,
    pl_numbers: u8,
}

struct Deleted {
    file_name: String,
    hashes: Vec<String>,
}

struct Replaced {
    file_name: String,
    hashes: Vec<String>,
}

struct SkippedDuplicate {
    file_name: String,
    hash: String,
}

struct SkippedIncomplete {
    file_path: String,
}

struct SkippedUnchanged {
    file_name: String,
}

struct SkippedUnreleased {
    file_name: String,
    release_state: String,
}

pub(crate) struct Report {
    uploaded: Vec<Uploaded>,
    deleted: Vec<Deleted>,
    replaced: Vec<Replaced>,
    skipped_duplicates: Vec<SkippedDuplicate>,
    skipped_incompletes: Vec<SkippedIncomplete>,
    skipped_unchangeds: Vec<SkippedUnchanged>,
    skipped_unreleaseds: Vec<SkippedUnreleased>,
    verbosity: i8,
}

impl Report {
    pub(crate) fn new(verbosity: i8) -> Report {
        Report {
            uploaded: Vec::new(),
            deleted: Vec::new(),
            replaced: Vec::new(),
            skipped_duplicates: Vec::new(),
            skipped_incompletes: Vec::new(),
            skipped_unchangeds: Vec::new(),
            skipped_unreleaseds: Vec::new(),
            verbosity,
        }
    }

    pub(crate) fn add_uploaded(&mut self, file_name: &str, hash: &str, pls: &str) {
        let pl_numbers: Vec<&str> = serde_json::from_str(pls).unwrap();
        let pl_numbers_count: u8 = pl_numbers.len() as u8;

        self.uploaded.push(Uploaded {
            file_name: file_name.to_string(),
            hash: hash.to_string(),
            pl_numbers: pl_numbers_count,
        });

        if self.verbosity >= 2 {
            println!("Uploading {} to blob storage.", file_name);
        }
    }

    pub(crate) fn add_skipped_duplicate(&mut self, file_name: &str, hash: &str) {
        self.skipped_duplicates.push(SkippedDuplicate {
            file_name: file_name.to_string(),
            hash: hash.to_string(),
        });

        if self.verbosity >= 1 {
            println!(
                "Skipping {} with sha1 hash '{}' as a duplicate of an already-uploaded file.",
                file_name, hash
            );
        }
    }

    pub(crate) fn add_deleted(&mut self, file_name: &str, hashes: Vec<String>) {
        self.deleted.push(Deleted {
            file_name: file_name.to_string(),
            hashes: hashes.clone()
        });

        if self.verbosity >= 1 {
            println!(
                "Deleting {} from blob storage (deleted hashes: {}).",
                file_name,
                hashes.join(", ")
            );
        }
    }

    pub(crate) fn add_replaced(&mut self, file_name: &str, hashes: Vec<String>) {
        self.replaced.push(Replaced {
            file_name: file_name.to_string(),
            hashes: hashes.clone()
        });

        if self.verbosity >= 1 {
            println!(
                "Replacing {} in blob storage (deleted hashes: {}).",
                file_name,
                hashes.join(", ")
            );
        }
    }

    pub(crate) fn add_skipped_unchanged(&mut self, file_name: &str) {
        self.skipped_unchangeds.push(SkippedUnchanged {
            file_name: file_name.to_string(),
        });

        if self.verbosity >= 1 {
            println!("Skipping {} because it has not changed.", file_name);
        }
    }

    pub(crate) fn add_skipped_incomplete(&mut self, file_path: &str) {
        self.skipped_incompletes.push(SkippedIncomplete {
            file_path: file_path.to_string(),
        });

        if self.verbosity >= 1 {
            println!(
                "Skipping file at '{}' because it has no metadata",
                file_path
            );
        }
    }

    pub(crate) fn add_skipped_unreleased(&mut self, file_name: &str, release_state: &str) {
        self.skipped_unreleaseds.push(SkippedUnreleased {
            file_name: file_name.to_string(),
            release_state: release_state.to_string(),
        });

        if self.verbosity >= 1 {
            println!(
                "Skipping {} with release state '{}' as it is not released to the public.",
                file_name, release_state
            );
        }
    }

    pub(crate) fn already_uploaded_file_with_hash(&self, hash: &str) -> bool {
        self.uploaded.iter().any(|uploaded| uploaded.hash == hash)
    }

    pub(crate) fn print_report(&self) {
        println!("---------------");
        println!("Number of uploaded files: {}", self.uploaded.len());
        println!("Number of replaced files: {}", self.replaced.len());
        println!("Number of deleted files: {}", self.deleted.len());
        println!(
            "Number of skipped files: {}",
            self.skipped_unreleaseds.len()
                + self.skipped_incompletes.len()
                + self.skipped_duplicates.len()
                + self.skipped_unchangeds.len()
        );

        println!("---------------");
        println!("List of uploaded files ({}):", self.uploaded.len());
        self.uploaded
            .iter()
            .for_each(|f| {
                println!(
                    "File {} was uploaded with {} PL numbers associated.",
                    f.file_name,
                    f.pl_numbers
                )
            });

        println!("---------------");
        println!("List of replaced files ({}):", self.replaced.len());
        self.replaced
            .iter()
            .for_each(|f| {
                println!(
                    "File {} replaced the following old file hashes: {}.",
                    f.file_name,
                    f.hashes.join(", ")
                )
            });

        println!("---------------");
        println!("List of deleted files ({}):", self.deleted.len());
        self.deleted
            .iter()
            .for_each(|f| {
                println!(
                    "File {} was deleted, with the following old file hashes: {}.",
                    f.file_name,
                    f.hashes.join(", ")
                )
            });

        if !self.skipped_unchangeds.is_empty() {
            println!("---------------");
            println!("Unchanged files ({}):", self.skipped_unchangeds.len());
            self.skipped_unchangeds.iter().for_each(|f| {
                println!(
                    "- File {} has not changed since the last batch.",
                    f.file_name
                )
            });
        }

        println!("---------------");
        println!("Unreleased files ({}):", self.skipped_unreleaseds.len());
        self.skipped_unreleaseds.iter().for_each(|f| {
            println!(
                "- File {} has release state '{}'",
                f.file_name, f.release_state
            )
        });

        println!("---------------");
        println!("Duplicate files ({}):", self.skipped_duplicates.len());
        self.skipped_duplicates.iter().for_each(|f| {
            println!(
                "- File {} with content hash '{}' is a duplicate.",
                f.file_name, f.hash
            )
        });

        println!("---------------");
        println!(
            "Files missing metadata ({}):",
            self.skipped_incompletes.len()
        );
        self.skipped_incompletes
            .iter()
            .for_each(|f| println!("- File at path '{}' has no metadata.", f.file_path));
    }
}
