use std::sync::Arc;

use bon::Builder;
use reqwest::{
    Client, Url,
    header::{COOKIE, HeaderMap, HeaderValue},
};
use thiserror::Error;

pub const CREATE_ERROR: &str = "failed to create the client";

#[derive(Debug, Error)]
#[error("{CREATE_ERROR}")]
pub struct CreateError {
    #[source]
    #[from]
    pub error: reqwest::Error,
}

#[derive(Debug, Clone, Builder)]
pub struct Http {
    pub client: Client,
    pub base_url: Arc<Url>,
}

pub const DEFAULT_BASE_URL: &str = "https://www.boomlings.com/database";

pub fn default_base_url() -> Url {
    Url::parse(DEFAULT_BASE_URL).expect("default base url must be valid")
}

pub const GD: HeaderValue = HeaderValue::from_static("gd=1");

pub fn default_headers() -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();

    headers.append(COOKIE, GD);

    headers
}

impl Default for Http {
    fn default() -> Self {
        Self::try_default().expect(CREATE_ERROR)
    }
}

impl Http {
    pub fn default_with(client: Client) -> Self {
        Self::builder()
            .client(client)
            .base_url(Arc::new(default_base_url()))
            .build()
    }

    pub fn try_default() -> Result<Self, CreateError> {
        let client = Client::builder()
            .default_headers(default_headers())
            .build()?;

        Ok(Self::default_with(client))
    }
}
