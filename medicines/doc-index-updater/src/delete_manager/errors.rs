use std::{error, error::Error, fmt};
#[derive(Debug, Clone)]
pub struct DocumentNotFoundInIndex {
    content_id: String,
}

impl DocumentNotFoundInIndex {
    pub fn for_content_id(content_id: String) -> DocumentNotFoundInIndex {
        DocumentNotFoundInIndex { content_id }
    }
}

impl fmt::Display for DocumentNotFoundInIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot find document with ID {}", self.content_id)
    }
}

impl Error for DocumentNotFoundInIndex {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
