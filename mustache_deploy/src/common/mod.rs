pub use std::{
    env,
    future::Future,
    io::Write,
    path::PathBuf,
    sync::Arc,
};

pub use tokio::time::Duration;

pub use log::{error, info};

pub use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, Record};

pub use serde_json::Value;

pub use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub use elasticsearch::{
    Elasticsearch,
    http::headers::HeaderMap,
    http::response::Response,
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    http::{Method, Url},
};

pub use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};

pub use chrono::{DateTime, NaiveDateTime, Utc};

pub use dotenv::dotenv;

pub use anyhow::{Result, anyhow};

pub use derive_new::new;
pub use getset::Getters;

pub use async_trait::async_trait;

pub use regex::Regex;

pub use urlencoding::encode;

pub use toml;

// Constants
pub const DEFAULT_ENV: &str = "dev";
pub const ENV_PROD_PATH: &str = "ES_PROD_PATH";
pub const ENV_DEV_PATH: &str = "ES_DEV_PATH";
pub const HTTP_PROTOCOL: &str = "http://";
pub const CONNECTION_TIMEOUT_SECS: u64 = 5;
pub const LOG_DIRECTORY: &str = "logs";
pub const LOG_FILES_TO_KEEP: usize = 10;