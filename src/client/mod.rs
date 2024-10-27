mod inner;
use std::future::Future;
use std::sync::Arc;

use crate::method::Method;
use crate::reader::LazyResponseReader;
use crate::res::Collection;
use crate::res::MultiStatus;
use crate::{header::HeaderMap, Body};
pub use inner::InnerClient;
use reqwest::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::IntoUrl;
use reqwest::Response;
use reqwest::Url;

macro_rules! header_value {
    ($arg:expr) => {
        reqwest::header::HeaderValue::from_bytes($arg.as_bytes()).unwrap()
    };
}
macro_rules! header_name {
    ($arg:expr) => {
        reqwest::header::HeaderName::from_bytes($arg.as_bytes()).unwrap()
    };
}

const ALL_DROP: &str = r#"<?xml version="1.0" encoding="utf-8" ?>
    <D:propfind xmlns:D="DAV:">
        <D:allprop/>
    </D:propfind>
"#;
#[derive(Default, Clone)]
pub struct WebDAVClient {
    inner: Arc<InnerClient>,
}
unsafe impl Send for WebDAVClient {}

unsafe impl Sync for WebDAVClient {}

macro_rules! into_url {
    ($url:expr) => {
        match $url.into_url() {
            Ok(url) => url,
            Err(e) => panic!("{e}")
        }
    };
}

impl WebDAVClient {
    pub fn new(username: &str, password: &str) -> Result<Self, reqwest::Error> {
        Ok(Self {
            inner: Arc::new(InnerClient::new(username, password)?),
        })
    }
    pub fn request(&self, method: Method, url: impl IntoUrl) -> WevDAVRequestBuilder {
        WevDAVRequestBuilder::new(self.inner.clone(), into_url!(url), method)
    }

    #[inline(always)]
    pub fn get(&self, url: impl IntoUrl) -> WevDAVRequestBuilder {
        self.request(Method::GET, url)
    }

    pub fn put(&self, url: impl IntoUrl) -> WevDAVRequestBuilder {
        self.request(Method::PUT, url)
    }

    pub async fn list(&self, url: impl IntoUrl) -> Result<Collection, crate::error::Error> {
        let response = self.all_propfind(url).await?;
        if response.status().is_success() {
            let xml = response.text().await?;
            let multi_status = MultiStatus::parse(&xml)?;
            Ok(Collection::from(multi_status))
        } else {
            Err(crate::error::Error::ResponseError(response.status()))
        }
    }
    #[inline(always)]
    pub async fn all_propfind(
        &self,
        url: impl IntoUrl,
    ) -> Result<Response, crate::error::Error> {
        self.request(Method::PROPFIND, url.into_url()?)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/xml"))
            .header(header_name!("depth"), header_value!("1"))
            .body(ALL_DROP)
            .send().await.map_err(Into::into)
    }
}

pub struct WevDAVRequestBuilder {
    client: Arc<InnerClient>,
    basic_auth: Option<(String, String)>,
    url: Url,
    headers: HeaderMap,
    body: Option<Body>,
    method: Method,
}

impl WevDAVRequestBuilder {
    pub fn new(client: Arc<InnerClient>, url: Url, method: Method) -> Self {
        Self {
            client,
            basic_auth: None,
            headers: HeaderMap::new(),
            url,
            method,
            body: None,
        }
    }
    pub fn basic_auth(self, username: &str, password: &str) -> Self {
        Self {
            basic_auth: Some((username.to_owned(), password.to_owned())),
            ..self
        }
    }
    pub fn body(self, body: impl Into<Body>) -> Self {
        Self {
            body: Some(body.into()),
            ..self
        }
    }
    #[inline(always)]
    pub fn range(self, start: usize, end: usize) -> Self {
        self.header(
            header_name!("range"),
            header_value!(format!("bytes={}-{}", start, end)),
        )
    }
    pub fn header(mut self, key: HeaderName, val: HeaderValue) -> Self {
        self.headers.insert(key, val);
        self
    }
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn build(self) -> crate::RequestBuilder {
        let builder = self.client.inner.request(self.method.convert(), self.url);
        let builder = if let Some(body) = self.body {
            builder.body(body)
        } else {
            builder
        };
        if let Some((usr, pass)) = &self.basic_auth {
            builder.basic_auth(usr, Some(pass))
        } else if let Some((usr, psw)) = &self.client.auth {
            builder.basic_auth(usr, Some(psw))
        } else {
            panic!("Missing basic auth!")
        }
        .headers(self.headers)
    }
    pub fn into_lazy_reader(self) -> LazyResponseReader {
        LazyResponseReader::new(self.build())
    }
    pub fn send(self) -> impl Future<Output = Result<Response, reqwest::Error>> {
        self.build().send()
    }
}
