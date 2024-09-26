pub mod method;
pub mod client;
mod url;
pub mod error;
pub mod reader;
pub mod res;
pub use client::{WebDAVClient, Auth, Range};
pub use reqwest::header;
pub use reqwest::{Body, Request, RequestBuilder, Response, StatusCode, Url};
pub use method::Method;

pub use quick_xml::DeError;
