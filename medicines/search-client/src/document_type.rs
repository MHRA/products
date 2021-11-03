use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[cfg_attr(
    feature = "graphql",
    async_graphql::Enum(desc = "Document type (SPC/PIL/PAR)"),
    derive(Serialize, Deserialize, Debug, Ord, PartialOrd)
)]
#[cfg_attr(
    not(feature = "graphql"),
    derive(
        Serialize,
        Deserialize,
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        Ord,
        PartialOrd
    )
)]
pub enum DocumentType {
    #[serde(alias = "SPC")]
    Spc,
    #[serde(alias = "PIL")]
    Pil,
    #[serde(alias = "PAR")]
    Par,
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
            DocumentType::Spc => write!(f, "Spc"),
            DocumentType::Pil => write!(f, "Pil"),
            DocumentType::Par => write!(f, "Par"),
        }
    }
}

#[cfg_attr(
    feature = "graphql",
    async_graphql::Enum(desc = "Territory type (UK/GB/NI)"),
    derive(Serialize, Deserialize, Debug, Ord, PartialOrd)
)]
#[cfg_attr(
    not(feature = "graphql"),
    derive(
        Serialize,
        Deserialize,
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        Ord,
        PartialOrd
    )
)]
pub enum TerritoryType {
    #[serde(alias = "Uk")]
    UK,
    #[serde(alias = "Gb")]
    GB,
    #[serde(alias = "Ni")]
    NI,
}

impl FromStr for TerritoryType {
    type Err = TerritoryTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "UK" => Ok(Self::UK),
            "GB" => Ok(Self::GB),
            "NI" => Ok(Self::NI),
            _ => Err(TerritoryTypeParseError {
                source: s.to_string(),
            }),
        }
    }
}

impl Display for TerritoryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TerritoryType::UK => write!(f, "UK"),
            TerritoryType::GB => write!(f, "GB"),
            TerritoryType::NI => write!(f, "NI"),
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

#[derive(Debug, Clone)]
pub struct TerritoryTypeParseError {
    source: String,
}

impl Display for TerritoryTypeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Could not parse TerritoryType from string: {}",
            self.source
        )
    }
}

impl std::error::Error for TerritoryTypeParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Spc")]
    #[test_case("Pil")]
    #[test_case("Par")]
    fn parses_document_and_formats_to_a_string(doc_type: &str) {
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
    fn deserializes_document_cases_insensitively(input: &str, expected: DocumentType) {
        use pretty_assertions::assert_eq;

        let from_str: DocumentType = serde_json::from_str(input).unwrap();

        assert_eq!(from_str, expected);
    }

    #[test_case("UK")]
    #[test_case("GB")]
    #[test_case("NI")]
    fn parses_territory_and_formats_to_a_string(territory_type: &str) {
        use pretty_assertions::assert_eq;

        let from_str: TerritoryType = territory_type.parse().unwrap();

        let as_str = from_str.to_string();

        assert_eq!(territory_type, as_str);
    }

    #[test_case("\"UK\"", TerritoryType::UK; "uk uppercase")]
    #[test_case("\"Uk\"", TerritoryType::UK; "uk titlecase")]
    #[test_case("\"GB\"", TerritoryType::GB; "gb uppercase")]
    #[test_case("\"Gb\"", TerritoryType::GB; "gb titlecase")]
    #[test_case("\"NI\"", TerritoryType::NI; "ni uppercase")]
    #[test_case("\"Ni\"", TerritoryType::NI; "ni titlecase")]
    fn deserializes_territory_cases_insensitively(input: &str, expected: TerritoryType) {
        use pretty_assertions::assert_eq;

        let from_str: TerritoryType = serde_json::from_str(input).unwrap();

        assert_eq!(from_str, expected);
    }
}
