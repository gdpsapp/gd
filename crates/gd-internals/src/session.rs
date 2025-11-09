use crate::http::Http;

#[derive(Debug, Clone, Default)]
pub struct Session {
    pub http: Http,
}

impl Session {
    pub const fn new(http: Http) -> Self {
        Self { http }
    }
}
