struct Uploaded {
    file_name: String,
    hash: String,
}

struct SkippedDuplicate {
    file_name: String,
    hash: String,
}

struct SkippedIncomplete {
    file_path: String,
}

struct SkippedUnreleased {
    file_name: String,
    release_state: String,
}

pub(crate) struct Report {
    uploaded: Vec<Uploaded>,
    skipped_duplicates: Vec<SkippedDuplicate>,
    skipped_incompletes: Vec<SkippedIncomplete>,
    skipped_unreleaseds: Vec<SkippedUnreleased>,
}

impl Report {
    pub(crate) fn new() -> Report {
        Report {
            uploaded: Vec::new(),
            skipped_duplicates: Vec::new(),
            skipped_incompletes: Vec::new(),
            skipped_unreleaseds: Vec::new()
        }
    }

    pub(crate) fn add_uploaded(&mut self, file_name: &str, hash: &str) {
        self.uploaded.push(Uploaded {
            file_name: file_name.to_string(),
            hash: hash.to_string()
        });
    }

    pub(crate) fn add_skipped_duplicate(&mut self, file_name: &str, hash: &str) {
        self.skipped_duplicates.push(SkippedDuplicate {
            file_name: file_name.to_string(),
            hash: hash.to_string()
        });
    }

    pub(crate) fn add_skipped_incomplete(&mut self, file_path: &str) {
        self.skipped_incompletes.push(SkippedIncomplete {
            file_path: file_path.to_string()
        });
    }

    pub(crate) fn add_skipped_unreleased(&mut self, file_name: &str, release_state: &str) {
        self.skipped_unreleaseds.push(SkippedUnreleased {
            file_name: file_name.to_string(),
            release_state: release_state.to_string()
        });
    }

    pub(crate) fn already_uploaded_file_with_hash(&self, hash: &str) -> bool {
        self.uploaded.iter().any(|uploaded| uploaded.hash == hash)
    }

    pub(crate) fn print_report(&self) {
        println!("---------------");
        println!("Number of uploaded files: {}", self.uploaded.len());
        println!(
            "Number of skipped files: {}",
            self.skipped_unreleaseds.len() + self.skipped_incompletes.len() + self.skipped_duplicates.len()
        );

        println!("---------------");
        println!("Unreleased files ({}):", self.skipped_unreleaseds.len());
        self.skipped_unreleaseds.iter().for_each(
            |f| println!(
                "- File {} has release state '{}'",
                f.file_name,
                f.release_state
            )
        );

        println!("---------------");
        println!("Duplicate files ({}):", self.skipped_duplicates.len());
        self.skipped_duplicates.iter().for_each(
            |f| println!(
                "- File {} with content hash '{}' is a duplicate.",
                f.file_name,
                f.hash
            )
        );

        println!("---------------");
        println!("Files missing metadata ({}):", self.skipped_incompletes.len());
        self.skipped_incompletes.iter().for_each(
            |f| println!(
                "- File at path '{}' has no metadata.",
                f.file_path
            )
        );
    }
}