use crate::models::{AzureIndexChangedResults, IndexEntry, IndexResults};
use async_trait::async_trait;
use core::fmt::Debug;
use serde::ser::Serialize;
use std::collections::HashMap;

use regex::Regex;

pub fn extract_normalized_product_licences(search_term: &str) -> String {
    lazy_static! {
      static ref RE_PRODUCT_LICENCE: Regex = Regex::new(r"(?P<letterprefix>PL|HR|THR)(\s+|/|_|-)*(?P<fivenumbers>\d{5})(\s+|/|_|-)*(?P<fournumbers>\d{4})").unwrap();
    }
    RE_PRODUCT_LICENCE
        .replace_all(search_term, "$letterprefix$fivenumbers$fournumbers")
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("PL 12345/1234", "PL123451234")]
    #[test_case("HR 12345/1234", "HR123451234")]
    #[test_case("THR 12345/1234", "THR123451234")]
    #[test_case("INVALID 12345/1234", "INVALID 12345/1234")]
    #[test_case("PL/23456/1234", "PL234561234")]
    #[test_case("PL-34567-1234", "PL345671234")]
    #[test_case("PL_45678_1234", "PL456781234")]
    fn test_build_search_without_pagination(input: &str, expected: &str) {
        let result = extract_normalized_product_licences(&input);
        assert_eq!(result, expected);
    }
}
