use lazy_static;
use regex::Regex;
use std::str;

pub fn sanitize(s: &str) -> String {
    s.replace(|c: char| !c.is_ascii(), "")
        .replace("\n", " ")
        .trim()
        .to_string()
}

pub fn to_json(words: Vec<String>) -> String {
    serde_json::to_string(&words).expect("Couldn't create JSON array.")
}

pub fn create_facets_by_active_substance(
    product: &str,
    active_substances: Vec<String>,
) -> Vec<String> {
    let mut facets: Vec<String> = active_substances
        .iter()
        .map(|a| {
            let first = a.chars().next().unwrap();
            vec![
                first.to_string(),
                [first.to_string(), a.to_string()].join(", "),
                [first.to_string(), a.to_string(), product.to_string()].join(", "),
            ]
        })
        .flatten()
        .collect();
    facets.sort();
    facets.dedup();
    facets
}

pub fn extract_product_licences(input: &str) -> String {
    lazy_static! {
        static ref RE_WHITESPACE: Regex = Regex::new(r"(\s+|/|_|-)").expect("cannot compile regex");
        static ref RE_PL: Regex = Regex::new(r"(?i:\b|PL)(\s+|/|_|-)*\d{5}(\s+|/|_|-)*\d{4}")
            .expect("cannot compile regex");
    }
    let product_licences: Vec<String> = RE_PL
        .find_iter(input)
        .map(|m| {
            RE_WHITESPACE
                .replace_all(m.as_str(), "")
                .to_ascii_uppercase()
        })
        .map(|s| {
            if s.starts_with("PL") {
                s
            } else {
                String::from("PL") + s.as_str()
            }
        })
        .collect();

    to_json(product_licences)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanitize_remove_newline() {
        assert_eq!(sanitize("newline\ntest"), "newline test");
    }
    #[test]
    fn sanitize_remove_non_ascii() {
        assert_eq!(sanitize("emoji🙂 ∫test"), "emoji test");
    }
    #[test]
    fn sanitize_trim() {
        assert_eq!(sanitize(" test "), "test");
    }
    #[test]
    fn jsonify_keywords() {
        let s = "ukpar, public assessment report, par, national procedure,Ibuprofen, Phenylephrine Hydrochloride";
        let json = "[\"ukpar\",\"public assessment report\",\"par\",\"national procedure\",\"Ibuprofen\",\"Phenylephrine Hydrochloride\"]";
        assert_eq!(to_json(to_array(s)), json);
    }
    #[test]
    fn jsonify_single_term() {
        let s = "Phenylephrine Hydrochloride";
        let json = "[\"Phenylephrine Hydrochloride\"]";
        assert_eq!(to_json(to_array(s)), json);
    }
    #[test]
    fn jsonify_terms_joined_with_and() {
        let s = "THIOPENTAL SODIUM AND SODIUM CARBONATE";
        let json = "[\"THIOPENTAL SODIUM\",\"SODIUM CARBONATE\"]";
        assert_eq!(to_json(to_array(s)), json);
    }
    #[test]
    fn jsonify_terms_with_multiple_spaces() {
        let s = "THIOPENTAL   SODIUM AND SODIUM  CARBONATE";
        let json = "[\"THIOPENTAL SODIUM\",\"SODIUM CARBONATE\"]";
        assert_eq!(to_json(to_array(s)), json);
    }
    #[test]
    fn jsonify_terms_joined_with_and_2() {
        let s = "THIOPENTAL SODIUMANDSODIUM CARBONATE";
        let json = "[\"THIOPENTAL SODIUMANDSODIUM CARBONATE\"]";
        assert_eq!(to_json(to_array(s)), json);
    }

    #[test]
    fn test_create_facets_by_active_substance() {
        let active_substances = vec![
            "LOSARTAN POTASSIUM".to_string(),
            "HYDROCHLOROTHIAZIDE".to_string(),
            "L-TEST".to_string(),
        ];
        let product = "LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS";
        let expected = vec![
            "H", 
            "H, HYDROCHLOROTHIAZIDE", 
            "H, HYDROCHLOROTHIAZIDE, LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS",
            "L",
            "L, L-TEST", 
            "L, L-TEST, LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS",
            "L, LOSARTAN POTASSIUM", 
            "L, LOSARTAN POTASSIUM, LOSARTAN POTASSIUM / HYDROCHLOROTHIAZIDE 100 MG /25 MG FILM-COATED TABLETS",
        ];
        assert_eq!(
            create_facets_by_active_substance(product, active_substances),
            expected
        );
    }

    #[test]
    fn extract_product_license_test() {
        let input = vec![
            "00 PL123451234",
            "01 pl123451234",
            "02 123451234",
            "03 PL 12345 1234",
            "04 PL  12345 1234",
            "05 test pl 12345   1234",
            "06 pl  12345   1234 test",
            "07 12345 1234",
            "08 PL 12345/1234",
            "09 PL/12345/1234",
            "10 pl 12345/1234",
            "11 pl/12345/1234",
            "12 12345/1234",
            "13 PL 12345_1234",
            "14 PL_12345_1234",
            "15 pl 12345_1234",
            "16 pl_12345_1234",
            "17 12345_1234",
            "18 PL 12345-1234",
            "19 PL-12345-1234",
            "20 pl 12345-1234",
            "21 pl-12345-1234",
            "22 12345-1234",
            "23 12345-1234GG",
            "leaflet MAH GENERIC_PL 12345-1234R.pdf",
        ];
        let output = "[\"PL123451234\"]";
        input
            .iter()
            .for_each(|i| assert_eq!(extract_product_licences(i), output));
    }
    #[test]
    fn extract_multiple_product_licences() {
        let input = "00 PL123451234 01 pl123451235__ 02 123451236-03 PL 12345 1237";
        let output = "[\"PL123451234\",\"PL123451235\",\"PL123451236\",\"PL123451237\"]";

        assert_eq!(extract_product_licences(input), output);
    }
    #[test]
    fn extract_product_license_test_not_found() {
        assert_eq!(extract_product_licences("no pl number here"), "[]");
    }
}
