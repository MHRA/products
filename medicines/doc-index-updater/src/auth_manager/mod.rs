use regex::Regex;
use warp::{Filter, Rejection};

#[derive(Debug)]
struct NoAuthenticationPassed;
#[derive(Debug)]
struct CorruptAuthenticationPassed;
#[derive(Debug)]
struct AuthenticationFailed;
impl warp::reject::Reject for NoAuthenticationPassed {}
impl warp::reject::Reject for CorruptAuthenticationPassed {}
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

fn extract_auth_from_header(auth_header: Option<String>) -> Option<String> {
    if let Some(auth_string) = auth_header {
        let re = Regex::new(r"^Basic\s(?P<encoded_credentials>[-A-Za-z0-9+/]*={0,3})$").unwrap();

        if let Some(caps) = re.captures(&auth_string) {
            match caps.name("encoded_credentials") {
                Some(creds) => Some(creds.as_str().to_string()),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn extract_credentials_from_base64_string(encoded_credentials: String) -> Option<(String, String)> {
    if let Ok(credentials) = base64::decode(encoded_credentials) {
        let re = Regex::new(r"^(?P<username>\w+)\:(?P<password>\w+)$").unwrap();
        match re.captures(&encoded_credentials) {
            Some(caps) => {
                if let (Some(username), Some(password)) =
                    (caps.name("username"), caps.name("password"))
                {
                    Some((username.as_str().to_string(), password.as_str().to_string()))
                } else {
                    None
                }
            }
            None => None,
        }
    } else {
        None
    }
}

pub fn basic_login(
    auth_header: Option<String>,
) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    if let Some(encoded_credentials) = extract_auth_from_header(auth_header) {
        match extract_credentials_from_base64_string(encoded_credentials) {
            Some((username, password)) => match auth_is_correct(username, password) {
                true => (),
                false => warp::reject::custom(AuthenticationFailed),
            },
            None => warp::reject::custom(CorruptAuthenticationPassed),
        }
    } else {
        warp::reject::custom(NoAuthenticationPassed)
    }
}
