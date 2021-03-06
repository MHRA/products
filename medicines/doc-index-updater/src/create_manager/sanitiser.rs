#[derive(Clone, Debug, PartialEq)]
pub struct SanitisedString {
    inner: String,
}

impl From<String> for SanitisedString {
    fn from(inner: String) -> Self {
        Self { inner }
    }
}

impl From<&String> for SanitisedString {
    fn from(inner: &String) -> Self {
        Self::from(inner.to_owned())
    }
}

impl From<&str> for SanitisedString {
    fn from(inner: &str) -> Self {
        Self::from(inner.to_owned())
    }
}

impl ToString for SanitisedString {
    fn to_string(&self) -> String {
        self.inner
            .replace(|c: char| !c.is_ascii(), "")
            .replace("\n", " ")
            .trim()
            .to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VecSanitisedString {
    inner: Vec<String>,
}

impl From<Vec<String>> for VecSanitisedString {
    fn from(inner: Vec<String>) -> Self {
        Self { inner }
    }
}

impl VecSanitisedString {
    pub fn to_vec_string(&self) -> Vec<String> {
        self.inner
            .iter()
            .map(|s: &String| SanitisedString::from(s).to_string())
            .collect::<Vec<String>>()
    }

    pub fn join(&self, sep: &str) -> String {
        self.to_vec_string().join(sep)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.to_vec_string()).expect("Couldn't create JSON array.")
    }
}

impl Default for VecSanitisedString {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanitise_remove_newline() {
        assert_eq!(
            SanitisedString::from("newline\ntest").to_string(),
            "newline test"
        );
    }
    #[test]
    fn sanitise_remove_accented_character() {
        assert_eq!(
            SanitisedString::from("un café, s'il vous plait").to_string(),
            "un caf, s'il vous plait"
        );
    }

    #[test]
    fn sanitise_remove_emoji() {
        assert_eq!(
            SanitisedString::from("emoji🙂 ∫test").to_string(),
            "emoji test"
        );
    }
    #[test]
    fn sanitise_the_cure() {
        // disintegration - get it?
        assert_eq!(SanitisedString::from("∫x").to_string(), "x");
    }
    #[test]
    fn sanitise_trim() {
        assert_eq!(SanitisedString::from(" test ").to_string(), "test");
    }
}
