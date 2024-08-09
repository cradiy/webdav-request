pub mod client;
mod error;
pub mod parse;
pub mod read;
pub use reqwest;
pub const ALL_DROP: &str = r#"
<?xml version="1.0"?>
<d:propfind xmlns:d="DAV:">
<d:allprop/>
</d:propfind>
"#;
pub use error::*;
pub mod webdav_method {
    use std::sync::LazyLock;
    pub static PROPFIND: LazyLock<reqwest::Method> =
        LazyLock::new(|| reqwest::Method::from_bytes(b"PROPFIND").unwrap());
}
