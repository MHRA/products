extern crate rand;

use std::env;
use self::rand::Rng;

pub const API_ADMIN_KEY: &str = "API_ADMIN_KEY";
pub const DATASOURCE_NAME: &str = "DATASOURCE_NAME";
pub const INDEX_NAME: &str = "INDEX_NAME";
pub const INDEXER_NAME: &str = "INDEXER_NAME";
pub const SEARCH_SERVICE: &str = "SEARCH_SERVICE";
pub const STORAGE_ACCOUNT: &str = "STORAGE_ACCOUNT";
pub const STORAGE_CONTAINER: &str = "STORAGE_CONTAINER";
pub const STORAGE_MASTER_KEY: &str = "STORAGE_MASTER_KEY";

pub fn get_from_env(environment_variable: &str) -> String {
    env::var(environment_variable)
        .expect(format!("Set env variable {} first!", environment_variable).as_ref())
}

#[test]
fn test_get_env_var() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const RANDOM_LEN: u8 = 20;

    let mut rng = rand::thread_rng();

    let random_env_var: String = (0..RANDOM_LEN)
        .map(|_| {
            let i = rng.gen_range(0, CHARSET.len());
            CHARSET[i] as char
        })
        .collect();

    env::set_var(&random_env_var, "found");
    assert_eq!(get_from_env(&random_env_var), "found");
}