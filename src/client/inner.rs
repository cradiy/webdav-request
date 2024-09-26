use crate::url::WebDavURL;

#[derive(Debug, Clone)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

impl Auth {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}
#[derive(Clone, Default)]
pub struct InnerClient {
    pub(crate) base_url: Option<WebDavURL>,
    pub(crate) auth: Option<Auth>,
    pub(crate) inner: reqwest::Client,
}
unsafe impl Send for InnerClient {
    
}

unsafe impl Sync for InnerClient {
    
}