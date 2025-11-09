use std::sync::Arc;

use bon::Builder;
use reqwest::{Client, Url};

#[derive(Debug, Clone, Builder)]
pub struct Http {
    pub client: Client,
    pub base_url: Arc<Url>,
}
