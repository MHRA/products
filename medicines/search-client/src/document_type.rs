use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
pub enum DocumentType {
    #[serde(alias = "SPC")]
    Spc,
    #[serde(alias = "PIL")]
    Pil,
    #[serde(alias = "PAR")]
    Par,
}

impl DocumentType {
    pub fn to_search_str(&self) -> &str {
        match self {
            DocumentType::Spc => "Spc",
            DocumentType::Pil => "Pil",
            DocumentType::Par => "Par",
        }
    }
}

impl FromStr for DocumentType {
    type Err = DocTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "SPC" => Ok(Self::Spc),
            "PIL" => Ok(Self::Pil),
            "PAR" => Ok(Self::Par),
            _ => Err(DocTypeParseError {
                source: s.to_string(),
            }),
        }
    }
}

impl Display for DocumentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentType::Spc => write!(f, "SPC"),
            DocumentType::Pil => write!(f, "PIL"),
            DocumentType::Par => write!(f, "PAR"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocTypeParseError {
    source: String,
}

impl Display for DocTypeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Could not parse DocumentType from string: {}",
            self.source
        )
    }
}

impl std::error::Error for DocTypeParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("SPC")]
    #[test_case("PIL")]
    #[test_case("PAR")]
    fn parses_and_formats_to_a_string(doc_type: &str) {
        use pretty_assertions::assert_eq;

        let from_str: DocumentType = doc_type.parse().unwrap();

        let as_str = from_str.to_string();

        assert_eq!(doc_type, as_str);
    }

    #[test_case("\"SPC\"", DocumentType::Spc; "spc uppercase")]
    #[test_case("\"Spc\"", DocumentType::Spc; "spc titlecase")]
    #[test_case("\"PIL\"", DocumentType::Pil; "pil uppercase")]
    #[test_case("\"Pil\"", DocumentType::Pil; "pil titlecase")]
    #[test_case("\"PAR\"", DocumentType::Par; "par uppercase")]
    #[test_case("\"Par\"", DocumentType::Par; "par titlecase")]
    fn deserializes_cases_insensitively(input: &str, expected: DocumentType) {
        use pretty_assertions::assert_eq;

        let from_str: DocumentType = serde_json::from_str(input).unwrap();

        assert_eq!(from_str, expected);
    }
}
