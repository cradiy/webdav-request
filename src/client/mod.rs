mod inner;
use std::future::Future;
use std::sync::Arc;

use crate::method::Method;
use crate::reader::LazyResponseReader;
use crate::res::Collection;
use crate::res::MultiStatus;
use crate::{header::HeaderMap, Body};
pub use inner::Auth;
pub use inner::InnerClient;
pub use inner::Range;
use reqwest::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::Response;

use crate::url::WebDavURL;

const ALL_DROP: &str = r#"
<?xml version="1.0"?>
<d:propfind xmlns:d="DAV:">
<d:allprop/>
</d:propfind>
"#;

#[derive(Default, Clone)]
pub struct WebDAVClient {
    inner: Arc<InnerClient>,
}
unsafe impl Send for WebDAVClient {}

unsafe impl Sync for WebDAVClient {}

impl WebDAVClient {
    pub fn new(auth: Option<Auth>, base_url: Option<&str>) -> Result<Self, reqwest::Error> {
        Ok(Self {
            inner: Arc::new(InnerClient {
                auth,
                base_url: base_url.map(WebDavURL::new),
                inner: reqwest::Client::builder().build()?,
            }),
        })
    }
    pub fn request(&self, method: Method, url: &str) -> WevDAVRequestBuilder {
        WevDAVRequestBuilder::new(self.inner.clone(), url, method)
    }

    #[inline(always)]
    pub fn get(&self, url: &str) -> impl Future<Output = Result<Response, reqwest::Error>> {
        self.request(Method::GET, url).send()
    }

    #[inline(always)]
    pub async fn list_collection<U: AsRef<str>>(
        &self,
        url: U,
    ) -> Result<Collection, crate::error::Error> {
        let response = self.all_propfind(url.as_ref()).await?;
        if response.status().is_success() {
            let xml = response.text().await?;
            let multi_status = MultiStatus::from_str(&xml)?;
            Ok(Collection::from(multi_status))
        } else {
            Err(crate::error::Error::ResponseError(response.status()))
        }
    }
    #[inline(always)]
    pub fn all_propfind(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Response, reqwest::Error>> {
        self.request(Method::PROPFIND, url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/xml"))
            .body(ALL_DROP)
            .send()
    }
}

pub struct WevDAVRequestBuilder {
    client: Arc<InnerClient>,
    basic_auth: Option<(String, String)>,
    url: String,
    headers: HeaderMap,
    body: Option<Body>,
    method: Method,
}

impl<'c> WevDAVRequestBuilder {
    pub fn new(client: Arc<InnerClient>, url: &str, method: Method) -> Self {
        Self {
            client,
            basic_auth: None,
            headers: HeaderMap::new(),
            url: String::from(url),
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
    pub fn range(self, range: &Range) -> Self {
        self.header(
            HeaderName::from_static("range"),
            HeaderValue::from_bytes(format!("bytes={}-{}", range.start, range.end).as_bytes())
                .unwrap(),
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
        let builder = self.client.inner.request(
            self.method.convert(),
            self.client
                .base_url
                .as_ref()
                .map_or(self.url.clone(), |url| url.url_join(&self.url)),
        );
        let builder = if let Some(body) = self.body {
            builder.body(body)
        } else {
            builder
        };
        if let Some((usr, pass)) = &self.basic_auth {
            builder.basic_auth(usr, Some(pass))
        } else if let Some(auth) = &self.client.auth {
            builder.basic_auth(&auth.username, Some(&auth.password))
        } else {
            builder
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
