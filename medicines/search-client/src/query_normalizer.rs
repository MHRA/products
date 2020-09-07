use crate::models::{AzureIndexChangedResults, IndexEntry, IndexResults};
use async_trait::async_trait;
use core::fmt::Debug;
use regex::Captures;
use serde::ser::Serialize;
use std::collections::HashMap;
use urlencoding::encode;

use regex::Regex;

pub fn extract_normalized_product_licences(search_term: &str) -> String {
    lazy_static! {
        static ref RE_PRODUCT_LICENCE: Regex = Regex::new(r"(?P<prefix>^|\s+|PL|HR|THR)(\s+|/|_|-)*(?P<fivenumbers>\d{5})(\s+|/|_|-)*(?P<fournumbers>\d{4})").unwrap();
    }

    RE_PRODUCT_LICENCE
        .replace_all(&search_term, |caps: &Captures| {
            let prefix = if caps[1].trim().chars().count() > 0 {
                caps[1].to_string()
            } else {
                String::from("PL")
            };
            format!("{}{}{}", prefix, &caps[3], &caps[5])
        })
        .to_string()
}

pub fn prefer_exact_match_but_support_fuzzy_match(word: &str) -> String {
    format!(
        "{word}~{search_word_fuzziness} || {word}^{search_exactness_boost}",
        word = word,
        search_word_fuzziness = 1,
        search_exactness_boost = 4
    )
}

pub fn escape_special_characters(word: &str) -> String {
    lazy_static! {
        static ref RE_SPECIAL_CHARACTERS: Regex =
            Regex::new(r#"(?P<special_character>[+\-&\|!!\(\)\{\}\[\]\^""\~\*\?\\:\\\\/])"#)
                .unwrap();
    }
    RE_SPECIAL_CHARACTERS
        .replace_all(word, r"\${special_character}")
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("PL 12345/1234", "PL123451234")]
    #[test_case("HR 12345/1234", "HR123451234")]
    #[test_case("THR 12345/1234", "THR123451234")]
    #[test_case("12345/1234", "PL123451234")]
    #[test_case("PRETEXT PL 12345/1234 POSTTEXT", "PRETEXT PL123451234 POSTTEXT")]
    #[test_case("PRETEXT 12345/1234 POSTTEXT", "PRETEXT PL123451234 POSTTEXT")]
    #[test_case("PL/23456/1234", "PL234561234")]
    #[test_case("PL-34567-1234", "PL345671234")]
    #[test_case("PL_45678_1234", "PL456781234")]
    fn test_extract_normalized_product_licences(input: &str, expected: &str) {
        let result = extract_normalized_product_licences(&input);
        assert_eq!(result, expected);
    }

    #[test_case("ibuprofen", "ibuprofen~1 || ibuprofen^4")]
    fn test_prefer_exact_match_but_support_fuzzy_match(input: &str, expected: &str) {
        let result = prefer_exact_match_but_support_fuzzy_match(&input);
        assert_eq!(result, expected);
    }

    //todo: encode for special characters ; / ? : @ = + &
    #[test]
    fn test_escape_special_characters() {
        let input = "+ & - | ! ( ) { } [ ] ^ \" ~ * ? : \\ /";
        let expected =
            "\\+ \\& \\- \\| \\! \\( \\) \\{ \\} \\[ \\] \\^ \\\" \\~ \\* \\? \\: \\\\ \\/";
        let result = escape_special_characters(&input);
        assert_eq!(result, expected);
    }
}
