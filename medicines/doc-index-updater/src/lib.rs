#[macro_use]
extern crate lazy_static;

use anyhow::anyhow;
use core::fmt::Display;
use std::{env, str::FromStr};

pub mod auth_manager;
pub mod create_manager;
pub mod delete_manager;
pub mod document_manager;
pub mod health;
pub mod models;
pub mod search_client;
pub mod service_bus_client;
pub mod state_manager;
pub mod storage_client;

pub fn get_env_or_default<T>(key: &str, default: T) -> T
where
    T: FromStr + Display,
{
    get_env(key).unwrap_or_else(|e| {
        tracing::warn!(r#"defaulting {} to "{}" ({})"#, key, &default, e);
        default
    })
}

pub fn get_env<T>(key: &str) -> anyhow::Result<T>
where
    T: FromStr,
{
    env::var(key)?
        .parse::<T>()
        .map_err(|_| anyhow!("failed to parse for {}", key))
}
