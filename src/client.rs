use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE}, Body, Response
};

use crate::{
    parse::{FileTree, Multistatus},
    webdav_method::PROPFIND, Error, ALL_DROP,
};

pub struct Method;

pub struct WebDAVRequestBuilder<'w> {
    client: &'w WebDAVClient,
    basic_auth: Option<(String, String)>,
    url: String,
    headers: HeaderMap,
    body: Option<Body>,
    method: String,
}

impl<'a> WebDAVRequestBuilder<'a> {
    pub fn new(client: &'a WebDAVClient) -> Self {
        Self {
            client,
            basic_auth: None,
            headers: HeaderMap::new(),
            url: String::new(),
            method: String::new(),
            body: None
        }
    }
    pub fn basic_auth(mut self, username: &str, password: &str) -> Self {
        self.basic_auth = Some((username.to_owned(), password.to_owned()));
        self
    }
    pub fn request(mut self, url: &str, method: &str) -> Self {
        self.url = url.to_owned();
        self.method = method.to_uppercase();
        self
    }
    #[inline(always)]
    pub fn get(self, url: &str) -> Self {
        self.request(url, "GET")
    }
    #[inline(always)]
    pub fn put(self, url: &str) -> Self {
        self.request(url, "PUT")
    }

    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers.extend(headers);
        self
    }
    #[inline(always)]
    pub fn range(self, start: u64, end: u64) -> Self {
        self.header(
            header::HeaderName::from_static("range"),
            HeaderValue::from_bytes(format!("bytes={start}-{end}").as_bytes()).unwrap(),
        )
    }
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn send(mut self) -> impl std::future::Future<Output = Result<Response, reqwest::Error>> {
        let builder = self
            .client
            .inner
            .request(
                reqwest::Method::from_bytes(self.method.as_bytes()).expect("Error Method"),
                if self.url.starts_with('/') {
                    format!("{}{}", self.client.webdav_url, self.url)
                } else {
                    self.url.clone()
                },
            )
            .headers(self.headers);
        let builder = if let Some(body) = self.body.take() {
            builder.body(body)
        } else {
            builder
        };
        if let Some((username, password)) = &self.basic_auth {
            builder.basic_auth(username, Some(password))
        } else {
            builder.basic_auth(&self.client.username, Some(&self.client.password))
        }
        .send()
    }
}
#[derive(Clone, Default)]
pub struct WebDAVClient {
    inner: reqwest::Client,
    webdav_url: String,
    username: String,
    password: String,
}
unsafe impl Sync for WebDAVClient {}
unsafe impl Send for WebDAVClient {}

impl WebDAVClient {
    pub fn new(webdav_url: &str, username: &str, password: &str) -> Self {
        Self {
            inner: reqwest::Client::new(),
            webdav_url: webdav_url.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
    pub fn basic_auth(&mut self, username: &str, password: &str) -> &mut WebDAVClient {
        self.username = username.to_owned();
        self.password = password.to_owned();
        self
    }
    pub fn webdav_url(&mut self, webdav_url: &str) -> &mut WebDAVClient {
        self.webdav_url = webdav_url.to_owned();
        self
    }
    pub fn builder(&self) -> WebDAVRequestBuilder<'_> {
        WebDAVRequestBuilder::new(self)
    }
    pub async fn get(&self, url: &str) -> reqwest::Result<Response> {
        let url = if url.starts_with('/') {
            format!("{}{}", self.webdav_url, url)
        } else {
            url.to_owned()
        };
        self.inner
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
    }
    /// List directory
    /// 
    /// ## Warning
    /// Not compatible with all situations, conversion may fail.
    pub async fn list_dir<S: AsRef<str>>(&self, url: S) -> crate::Result<FileTree> {
        let response = self.all_propfind(url.as_ref()).await?;
        if response.status().is_success() {
            let xml = response.text().await?;
            let multi_status = Multistatus::from_str(&xml)?;
            Ok(FileTree::from(multi_status))
        } else {
            Err(Error::ResponseError(response.status()))
        }
    }
    pub async fn all_propfind(&self, url: &str) -> crate::Result<Response> {
        let url = if self.webdav_url.is_empty() {
            url.to_owned()
        } else {
            format!("{}/{}", self.webdav_url, url)
        };
        self.inner
            .request(PROPFIND.clone(), url)
            .basic_auth(&self.username, Some(&self.password))
            .header(CONTENT_TYPE, "application/xml")
            .body(ALL_DROP)
            .send()
            .await
            .map_err(Into::into)
    }
}
