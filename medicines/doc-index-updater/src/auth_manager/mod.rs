use futures::future;
use percent_encoding::percent_decode_str;
use regex::Regex;
use url::Url;
use warp::{Filter, Rejection};

#[derive(Clone, Debug)]
pub struct AuthenticationFailed;
impl warp::reject::Reject for AuthenticationFailed {}

fn get_basic_username() -> String {
    std::env::var("BASIC_AUTH_USERNAME").expect("Set env variable BASIC_AUTH_USERNAME first!")
}

fn get_basic_password() -> String {
    std::env::var("BASIC_AUTH_PASSWORD").expect("Set env variable BASIC_AUTH_PASSWORD first!")
}

fn auth_is_correct(username: String, password: String) -> bool {
    username == get_basic_username() && password == get_basic_password()
}

fn extract_encoded_credentials(auth_header: String) -> Option<String> {
    let re = Regex::new(r"^Basic\s(?P<encoded_credentials>[-A-Za-z0-9+/]*={0,3})$")
        .expect("Regex failed to compile");

    if let Some(caps) = re.captures(&auth_header) {
        if let Some(creds) = caps.name("encoded_credentials") {
            return Some(creds.as_str().to_string());
        }
    }
    None
}

fn decode_credentials(encoded_credentials: String) -> Option<(String, String)> {
    if let Ok(credentials) = base64::decode(&encoded_credentials) {
        extract_username_and_password(std::str::from_utf8(&credentials).unwrap_or("").to_string())
    } else {
        None
    }
}

fn extract_username_and_password(decoded_creds: String) -> Option<(String, String)> {
    if let Ok(url) = Url::parse(&format!("http://{}@example.com", decoded_creds)) {
        if let Some(pwd) = url.password() {
            if let Ok(pwd) = percent_decode_str(pwd).decode_utf8() {
                return Some((url.username().to_string(), pwd.into()));
            }
        }
    }
    None
}

fn attempt_basic_auth(auth_header: String) -> bool {
    if let Some(encoded_creds) = extract_encoded_credentials(auth_header) {
        if let Some((username, password)) = decode_credentials(encoded_creds) {
            return auth_is_correct(username, password);
        }
    }
    false
}

pub fn with_basic_auth() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::optional::<String>("Authorization")
        .and_then(|header: Option<String>| match header {
            Some(auth_header) => {
                if attempt_basic_auth(auth_header) {
                    future::ok(())
                } else {
                    future::err(warp::reject::custom(AuthenticationFailed))
                }
            }
            None => future::err(warp::reject::custom(AuthenticationFailed)),
        })
        .untuple_one()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn get_basic_username_works() {
        std::env::set_var("BASIC_AUTH_USERNAME", "username");
        assert_eq!(get_basic_username(), "username".to_string());
    }

    #[test]
    fn get_basic_password_works() {
        std::env::set_var("BASIC_AUTH_PASSWORD", "password");
        assert_eq!(get_basic_password(), "password".to_string());
    }

    #[test]
    fn check_auth_works() {
        std::env::set_var("BASIC_AUTH_USERNAME", "username");
        std::env::set_var("BASIC_AUTH_PASSWORD", "password");
        assert_eq!(
            auth_is_correct("username".to_owned(), "password".to_owned()),
            true
        );
        assert_eq!(
            auth_is_correct("not_username".to_owned(), "password".to_owned()),
            false
        );
        assert_eq!(
            auth_is_correct("username".to_owned(), "not_password".to_owned()),
            false
        );
    }

    #[test_case("Weird String", None)]
    #[test_case("dXNlcm5hbWU6cGFzc3dvcmQ=", Some(("username".to_string(), "password".to_string())))]
    fn extracting_credentials_works(input: &str, output: Option<(String, String)>) {
        assert_eq!(decode_credentials(input.to_string()), output);
    }

    #[test_case("Basic abcdef", Some("abcdef".to_string()))]
    #[test_case("Basic abcde===", Some("abcde===".to_string()))]
    #[test_case("Not an auth string", None)]
    fn extracting_base64_creds_works(input: &str, output: Option<String>) {
        assert_eq!(extract_encoded_credentials(input.to_string()), output);
    }

    #[test_case("Basic dXNlcm5hbWU6cGFzc3dvcmQ=".to_string(), true)]
    #[test_case("Bearer dXNlcm5hbWU6cGFzc3dvcmQ=".to_string(), false)]
    #[test_case("".to_string(), false)]
    fn attempt_basic_auth_works(input: String, output: bool) {
        std::env::set_var("BASIC_AUTH_USERNAME", "username");
        std::env::set_var("BASIC_AUTH_PASSWORD", "password");
        assert_eq!(attempt_basic_auth(input), output);
    }

    #[test] // can't use `test_case` as the generated function names clash
    fn test_extract_username_and_password() {
        for v in vec![
            (
                "username:password".to_string(),
                Some(("username".to_string(), "password".to_string())),
            ),
            (
                "user-name:pass-word".to_string(),
                Some(("user-name".to_string(), "pass-word".to_string())),
            ),
            (
                "user_name:pass word".to_string(),
                Some(("user_name".to_string(), "pass word".to_string())),
            ),
            (
                "user_%ame:@£$%^&*()".to_string(),
                Some(("user_%ame".to_string(), "@£$%^&*()".to_string())),
            ),
            (
                "\x01:\x01".to_string(),
                Some(("%01".to_string(), "\u{1}".to_string())),
            ),
            (
                "username:pass:word".to_string(),
                Some(("username".to_string(), "pass:word".to_string())),
            ),
            ("".to_string(), None),
        ]
        .iter()
        {
            assert_eq!(extract_username_and_password(v.0.clone()), v.1);
        }
    }
}
