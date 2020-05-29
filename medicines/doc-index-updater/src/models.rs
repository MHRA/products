use crate::service_bus_client::ProcessMessageError;
use async_trait::async_trait;
use regex::Regex;
use search_client::models::DocumentType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum JobStatus {
    Accepted,
    Done,
    NotFound,
    Error { message: String, code: String },
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            JobStatus::Accepted => write!(f, "Accepted"),
            JobStatus::Done => write!(f, "Done"),
            JobStatus::NotFound => write!(f, "NotFound"),
            JobStatus::Error { message, code } => write!(f, "Error({}: {})", code, message),
        }
    }
}

impl FromStr for JobStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<JobStatus, Self::Err> {
        match s {
            "Accepted" => Ok(JobStatus::Accepted),
            "Done" => Ok(JobStatus::Done),
            status => {
                // If this message is in the format "Error(error code: error message)",
                // reconstruct it into JobStatus::Error.
                let error_re = Regex::new(r"^Error\((?P<code>[^:]*): (?P<message>.*)\)$")
                    .expect("Regex failed to compile");
                match error_re.captures(status) {
                    Some(capture) => Ok(JobStatus::Error {
                        message: capture
                            .name("message")
                            .map_or("", |m| m.as_str())
                            .to_string(),
                        code: capture.name("code").map_or("", |m| m.as_str()).to_string(),
                    }),
                    None => Err(format!("Status unknown: {}", status)),
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobStatusResponse {
    pub id: Uuid,
    pub status: JobStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Document {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub author: String,
    pub products: Vec<String>,
    pub keywords: Option<Vec<String>>,
    pub pl_number: String,
    pub active_substances: Vec<String>,
    pub file_source: FileSource,
    pub file_path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FileSource {
    #[serde(alias = "sentinel")]
    Sentinel,
    TemporaryAzureBlobStorage,
}

impl Into<Document> for XMLDocument {
    fn into(self) -> Document {
        Document {
            id: self.id,
            name: self.name,
            document_type: self.document_type,
            author: self.author,
            products: self.products.product,
            keywords: match self.keywords {
                Some(kw) => Some(kw.keyword),
                None => None,
            },
            pl_number: self.pl_number,
            active_substances: self.active_substances.active_substance,
            file_source: self.file_source,
            file_path: self.file_path,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Products {
    #[serde(default)]
    pub product: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keywords {
    #[serde(default)]
    pub keyword: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveSubstances {
    #[serde(default)]
    pub active_substance: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XMLDocument {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub author: String,
    pub products: Products,
    pub keywords: Option<Keywords>,
    pub pl_number: String,
    pub active_substances: ActiveSubstances,
    pub file_source: FileSource,
    pub file_path: String,
}

#[derive(Serialize)]
#[serde(rename = "job")]
pub struct XMLJobStatusResponse {
    id: String,
    status: String,
}

impl Into<XMLJobStatusResponse> for JobStatusResponse {
    fn into(self) -> XMLJobStatusResponse {
        XMLJobStatusResponse {
            id: self.id.to_string(),
            status: self.status.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CreateMessage {
    pub job_id: Uuid,
    pub document: Document,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeleteMessage {
    pub job_id: Uuid,
    pub document_id: UniqueDocumentIdentifier,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum UniqueDocumentIdentifier {
    ContentId(String),
    MetadataStorageName(String),
}

/// Exists for historic backwards compatibility
/// Previously, if a DeleteMessage's document_id was set, it was a string, and identified a content_id.
/// We convert strings to content ids to allow deserialisation of old messages to work.
/// Also allows for document_manager endpoints to continue to accept `String` from the path, and easily convert these.
impl From<String> for UniqueDocumentIdentifier {
    fn from(content_id: String) -> Self {
        UniqueDocumentIdentifier::ContentId(content_id)
    }
}

#[async_trait]
pub trait Message: Sized + FromStr + Clone {
    fn get_id(&self) -> Uuid;
    fn to_json_string(&self) -> Result<String, serde_json::Error>;
    async fn process(self) -> Result<Uuid, ProcessMessageError>;
}

impl FromStr for CreateMessage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_slice::<CreateMessage>(s.as_bytes())?)
    }
}

impl FromStr for DeleteMessage {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_slice::<DeleteMessage>(s.as_bytes())?)
    }
}

#[async_trait]
impl Message for CreateMessage {
    fn get_id(&self) -> Uuid {
        self.job_id
    }

    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string(&self)?)
    }

    async fn process(self) -> std::result::Result<uuid::Uuid, ProcessMessageError> {
        crate::create_manager::process_message(self.clone()).await
    }
}

#[async_trait]
impl Message for DeleteMessage {
    fn get_id(&self) -> Uuid {
        self.job_id
    }

    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string(&self)?)
    }

    async fn process(self) -> std::result::Result<uuid::Uuid, ProcessMessageError> {
        crate::delete_manager::process_message(self.clone()).await
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use serde_xml_rs::from_reader;
    use test_case::test_case;

    #[cfg(test)]
    pub fn get_test_document() -> Document {
        Document {
            id: "id".to_string(),
            name: "name".to_string(),
            document_type: DocumentType::Pil,
            author: "author".to_string(),
            products: vec!["products".to_string()],
            keywords: Some(vec!["keywords".to_string()]),
            pl_number: "pl_number".to_string(),
            active_substances: vec!["active_substances".to_string()],
            file_source: FileSource::Sentinel,
            file_path: "file_path".to_string(),
        }
    }

    #[cfg(test)]
    pub fn get_test_create_message(id: Uuid) -> CreateMessage {
        CreateMessage {
            job_id: id,
            document: get_test_document(),
        }
    }

    #[test_case("Accepted", Ok(JobStatus::Accepted))]
    #[test_case("Done", Ok(JobStatus::Done))]
    #[test_case("Error(0x0: Error status)", Ok(JobStatus::Error {message:"Error status".to_owned(), code:"0x0".to_owned()}))]
    #[test_case("Error(0x0: )", Ok(JobStatus::Error {message: "".to_owned(), code:"0x0".to_owned()}))]
    #[test_case("Error(: Error status)", Ok(JobStatus::Error {message:"Error status".to_owned(), code: "".to_owned()}))]
    #[test_case("Error(: )", Ok(JobStatus::Error {message: "".to_owned(), code: "".to_owned()}))]
    #[test_case("Bedro", Err("Status unknown: Bedro".to_owned()))]
    fn test_parse_job_status(input: &str, output: Result<JobStatus, String>) {
        assert_eq!(input.parse::<JobStatus>(), output);
    }

    #[test]
    fn test_deserialize_xml_doc() {
        let raw_xml_body = r##"
        <document>
            <id>con33333333</id>
            <name>Name of an SPC</name>
            <type>SPC</type>
            <author>theauthor</author>
            <products>
                <product>This is a product</product>
                <product>This is another product</product>
            </products>
            <pl_number>PL 12345/0010-0001</pl_number>
            <keywords>
                <keyword>
                    Test
                </keyword>
                <keyword>
                    Test 2
                </keyword>
            </keywords>
            <active_substances>
                <active_substance>Caffeine</active_substance>
                <active_substance>Caffeine 2</active_substance>
            </active_substances>
            <file_source>Sentinel</file_source>
            <file_path>example_file.txt</file_path>
        </document>
        "##;
        let doc: XMLDocument = from_reader(raw_xml_body.as_bytes()).unwrap();
        assert_eq!(doc.id, "con33333333");
        assert_eq!(doc.name, "Name of an SPC");
        assert_eq!(doc.document_type, DocumentType::Spc);
        assert_eq!(doc.author, "theauthor");
        assert_eq!(doc.products.product[0], "This is a product");
        assert_eq!(doc.products.product[1], "This is another product");
        assert_eq!(doc.pl_number, "PL 12345/0010-0001");
        if let Some(keywords) = doc.keywords {
            assert_eq!(keywords.keyword[0], "Test");
            assert_eq!(keywords.keyword[1], "Test 2");
        } else {
            panic!("Keywords not deserialized properly");
        }
        assert_eq!(doc.active_substances.active_substance[0], "Caffeine");
        assert_eq!(doc.active_substances.active_substance[1], "Caffeine 2");
        assert_eq!(doc.file_source, FileSource::Sentinel);
        assert_eq!(doc.file_path, "example_file.txt");
    }

    #[test]
    fn test_convert_xml_doc_into_standard_doc() {
        let raw_xml_body = r##"
        <document>
            <id>con33333333</id>
            <name>Name of an SPC</name>
            <type>SPC</type>
            <author>theauthor</author>
            <products>
                <product>This is a product</product>
                <product>This is another product</product>
            </products>
            <pl_number>PL 12345/0010-0001</pl_number>
            <keywords>
                <keyword>
                    Test
                </keyword>
                <keyword>
                    Test 2
                </keyword>
            </keywords>
            <active_substances>
                <active_substance>Caffeine</active_substance>
                <active_substance>Caffeine 2</active_substance>
            </active_substances>
            <file_source>Sentinel</file_source>
            <file_path>example_file.txt</file_path>
        </document>
        "##;
        let xml_doc: XMLDocument = from_reader(raw_xml_body.as_bytes()).unwrap();
        let doc = Into::<Document>::into(xml_doc);
        assert_eq!(doc.id, "con33333333");
        assert_eq!(doc.name, "Name of an SPC");
        assert_eq!(doc.document_type, DocumentType::Spc);
        assert_eq!(doc.author, "theauthor");
        assert_eq!(doc.products[0], "This is a product");
        assert_eq!(doc.products[1], "This is another product");
        assert_eq!(doc.pl_number, "PL 12345/0010-0001");
        if let Some(keywords) = doc.keywords {
            assert_eq!(keywords[0], "Test");
            assert_eq!(keywords[1], "Test 2");
        } else {
            panic!("Keywords not deserialized properly");
        }
        assert_eq!(doc.active_substances[0], "Caffeine");
        assert_eq!(doc.active_substances[1], "Caffeine 2");
        assert_eq!(doc.file_source, FileSource::Sentinel);
        assert_eq!(doc.file_path, "example_file.txt");
    }
}
