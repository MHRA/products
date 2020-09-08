use regex::Captures;

use regex::Regex;

pub fn extract_normalized_product_licences(search_term: &str) -> String {
    lazy_static! {
        static ref RE_PRODUCT_LICENCE: Regex = Regex::new(r"(?i)(?P<prefix>PL|HR|THR|\s|^)(\s+|/|_|-)*(?P<fivenumbers>\d{5})(\s+|/|_|-)*(?P<fournumbers>\d{4})").unwrap();
    }

    RE_PRODUCT_LICENCE
        .replace_all(&search_term, |caps: &Captures| {
            let prefix = if caps[1].is_empty() {
                String::from("PL")
            } else if caps[1].trim().is_empty() {
                String::from(" PL")
            } else {
                caps[1].to_uppercase()
            };
            format!(
                "{prefix}{five_numbers}{four_numbers}",
                prefix = prefix,
                five_numbers = &caps[3],
                four_numbers = &caps[5]
            )
        })
        .to_string()
}

pub fn prefer_exact_match_but_support_fuzzy_match(
    word: &str,
    search_word_fuzziness: &str,
    search_exactness_boost: &str,
) -> String {
    format!(
        "({word}~{search_word_fuzziness} || {word}^{search_exactness_boost})",
        word = word,
        search_word_fuzziness = search_word_fuzziness,
        search_exactness_boost = search_exactness_boost
    )
}

pub fn escape_special_characters(search_term: &str) -> String {
    lazy_static! {
        static ref RE_SPECIAL_CHARACTERS: Regex =
            Regex::new(r#"(?P<special_character>[\+\-\\/\\\^\|\?\*\\(\)\{\}\[\]&!"~:])"#).unwrap();
    }
    RE_SPECIAL_CHARACTERS
        .replace_all(search_term, r"\${special_character}")
        .to_string()
}

pub fn escape_special_words(search_term: &str) -> String {
    lazy_static! {
        static ref RE_SPECIAL_WORDS: Regex =
            Regex::new(r#"(?P<special_words>[AND|OR|NOT])"#).unwrap();
    }
    RE_SPECIAL_WORDS
        .replace_all(search_term, |caps: &Captures| caps[1].to_lowercase())
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
    #[test_case("PRETEXT pl 12345/1234", "PRETEXT PL123451234")]
    #[test_case("PL/23456/1234", "PL234561234")]
    #[test_case("PL-34567-1234", "PL345671234")]
    #[test_case("PL_45678_1234", "PL456781234")]
    fn test_extract_normalized_product_licences(input: &str, expected: &str) {
        let result = extract_normalized_product_licences(&input);
        assert_eq!(result, expected);
    }

    #[test_case("ibuprofen", "(ibuprofen~1 || ibuprofen^4)")]
    fn test_prefer_exact_match_but_support_fuzzy_match(input: &str, expected: &str) {
        let result = prefer_exact_match_but_support_fuzzy_match(&input, "1", "4");
        assert_eq!(result, expected);
    }

    //todo: encode for special characters ; / ? : @ = + &
    #[test]
    fn test_escape_special_characters() {
        let input = r#"+ & - | ! ( ) { } [ ] ^ " ~ * ? : \ /"#;
        let expected = r#"\+ \& \- \| \! \( \) \{ \} \[ \] \^ \" \~ \* \? \: \\ \/"#;
        let result = escape_special_characters(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_escape_special_words() {
        let input = "this AND that OR something else NOT the other";
        let expected = "this and that or something else not the other";
        let result = escape_special_words(&input);
        assert_eq!(result, expected);
    }
}
