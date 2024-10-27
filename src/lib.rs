pub mod client;
pub mod error;
pub mod method;
pub mod reader;
pub mod res;
pub mod url;
pub use client::WebDAVClient;
pub use method::Method;
pub use reqwest::header;
pub use reqwest::{Body, IntoUrl, Request, RequestBuilder, Response, StatusCode, Url};

pub use quick_xml::DeError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}
impl Range {
    pub fn new(start: usize, end: usize) -> Range {
        Self { start, end }
    }
}
