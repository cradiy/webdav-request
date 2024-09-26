use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct WebDavURL {
    scheme: String,
    domain: String,
    path: String,
}
unsafe impl Send for WebDavURL { }
unsafe impl Sync for WebDavURL { }
impl Unpin for WebDavURL { }


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
impl WebDavURL {
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
