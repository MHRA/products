use regex::Regex;
use std::str;
use tantivy::tokenizer::*;

pub fn sanitize(s: &str) -> String {
    s.replace(|c: char| !c.is_ascii(), "")
        .replace("\n", " ")
        .trim()
        .to_string()
}

pub fn tokenize(s: &str) -> String {
    let s1 = s.replace(|c: char| !c.is_ascii(), "");
    let tokenizer = SimpleTokenizer
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(StopWordFilter::default());
    let mut tokens: Vec<Token> = vec![];
    {
        let mut add_token = |token: &Token| {
            tokens.push(token.clone());
        };
        tokenizer.token_stream(&s1).process(&mut add_token);
    }
    tokens
        .iter()
        .map(|t| t.text.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn to_array(s: &str) -> Vec<String> {
    let re = Regex::new(r"(,|\s+AND\s+)").unwrap();
    let pattern_spaces = Regex::new(r"(\s+)").unwrap();
    re.split(s)
        .map(|s| s.trim())
        .map(|s| pattern_spaces.replace_all(s, " "))
        .map(|s| s.replace("\n", " "))
        .map(|s| s.replace(|c: char| !c.is_ascii(), ""))
        .collect()
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
    fn tokenize_remove_newline() {
        assert_eq!(tokenize("newline\ntest"), "newline test");
    }
    #[test]
    fn tokenize_remove_unicode() {
        assert_eq!(tokenize("emoji🙂 ∫test"), "emoji test");
    }
    #[test]
    fn tokenize_sample_keywords1() {
        let s1 = "ukpar, public assessment report, par, national procedure,Ibuprofen, Phenylephrine Hydrochloride, Ibuprofen and Phenylephrine Hydrochloride 200 mg/6.1 mg Tablets, 200 mg, 6.1 mg, cold, flu, congestion, aches, pains, headache, fever, sore throat, blocked nose, sinuses";
        let s2 = "ukpar public assessment report par national procedure ibuprofen phenylephrine hydrochloride ibuprofen phenylephrine hydrochloride 200 mg 6 1 mg tablets 200 mg 6 1 mg cold flu congestion aches pains headache fever sore throat blocked nose sinuses";
        assert_eq!(tokenize(s1), s2);
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
}
