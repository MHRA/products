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

pub fn to_json_array(s: &str) -> String {
    let re = Regex::new(r"(,|\s+AND\s+)").unwrap();
    let words = re
        .split(s)
        .map(|s| s.trim())
        .map(|s| s.replace("\n", " "))
        .map(|s| s.replace(|c: char| !c.is_ascii(), ""))
        .collect::<Vec<String>>();

    serde_json::to_string(&words).expect("Couldn't create JSON array.")
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
        assert_eq!(to_json_array(s), json);
    }
    #[test]
    fn jsonify_single_term() {
        let s = "Phenylephrine Hydrochloride";
        let json = "[\"Phenylephrine Hydrochloride\"]";
        assert_eq!(to_json_array(s), json);
    }
    #[test]
    fn jsonify_terms_joined_with_and() {
        let s = "THIOPENTAL SODIUM AND SODIUM CARBONATE";
        let json = "[\"THIOPENTAL SODIUM\",\"SODIUM CARBONATE\"]";
        assert_eq!(to_json_array(s), json);
    }
    #[test]
    fn jsonify_terms_joined_with_and_2() {
        let s = "THIOPENTAL SODIUMANDSODIUM CARBONATE";
        let json = "[\"THIOPENTAL SODIUMANDSODIUM CARBONATE\"]";
        assert_eq!(to_json_array(s), json);
    }
}
