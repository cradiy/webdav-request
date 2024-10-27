
use std::{fmt::Display, path::PathBuf};

use reqwest::IntoUrl;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct WebDavUrl {
    scheme: String,
    domain: String,
    path: String,
}
unsafe impl Send for WebDavUrl { }
unsafe impl Sync for WebDavUrl { }
impl Unpin for WebDavUrl { }

impl Display for WebDavUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}{}", self.scheme, self.domain, self.path))
    }
}


fn parse_url(url: &str) -> (String, String, String) {
    let path = PathBuf::from(url);
    let mut iter = path.iter();
    let Some(first_path) = iter.next().and_then(|p| p.to_str()) else {
        panic!("invalid url: `{url}`")
    };
    let mut domain = String::new();
    let scheme;
    let mut path = PathBuf::from("/");
    if matches!(first_path, "http:" | "https:") {
        scheme = format!("{}//", first_path);
        let Some(domain_) = iter.next().and_then(|p| p.to_str()) else {
            panic!("error webdav endpoint: {url}")
        };
        domain.push_str(domain_)
    } else {
        scheme = "https://".to_owned();
        domain.push_str(first_path);
    }
    path.extend(iter);
    (scheme, domain, path.to_string_lossy().to_string())
}
impl WebDavUrl {
    pub fn new(url: impl AsRef<str>) -> Self {
        let (scheme, domain , path) = parse_url(url.as_ref());
        Self {
            scheme,
            domain,
            path,
        }
    }
    pub fn scheme(&self) -> &str {
        &self.scheme
    }
    pub fn domain(&self) -> &str {
        &self.domain
    }
    pub fn path(&self) -> &str {
        &self.path
    }

    ///
    /// # Usage
    /// 
    /// ```
    /// use webdav_request::url::WebDavUrl;
    /// 
    /// let url = WebDavUrl::new("https://example.com/dav");
    /// let url2 = WebDavUrl::new("https://example.com/dav/dav");
    /// 
    /// assert_eq!(url.join("dav"), url2);
    /// assert_eq!(url.join("/dav"), url2);
    /// ```
    pub fn join(&self, path: &str) -> Self {
        let mut url = self.clone();
        if path.starts_with("/") {
            url.path.push_str(path);
        } else {
            url.path.push('/');
            url.path.push_str(path);
        }
        url
    }

    pub(crate) fn smart_merge(&self, path: &str) -> String {
        if path.starts_with("/") {
            if path.starts_with(self.path()) {
                format!("{}{}{}", self.scheme, self.domain(), path)
            } else {
                format!("{}{}{}{}",self.scheme, self.domain(), self.path, path)
            } 
        } else {
            path.to_owned()
        }
    }
    pub fn url_join(&self, url: &str) -> String {
        if url.starts_with("/") {
            if url.starts_with(self.path()) {
                format!("{}{}{}", self.scheme, self.domain(), url)
            } else {
                format!("{}{}{}{}",self.scheme, self.domain(), self.path, url)
            } 
        } else {
            url.to_owned()
        }
    }
}

