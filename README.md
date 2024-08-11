<p>
    <a href="https://crates.io/crates/webdav-request">
    	<img alt="Crate Info" src="https://img.shields.io/crates/v/webdav-request.svg"/>
    </a>
</p>

# wevdav-request

`webdav-request` a lightweight webdav client library, based on [reqwest](https://crates.io/crates/reqwest).
# WARNING
This is a library under development and is not stable.


# Getting Started

```rust 
use webdav_request::WebDAVClient;

const WEBDAV_URL: &str = "https://your.webdav.com";
const USERNAME: &str = "name";
const PASSWORD: &str = "password";

#[tokio::main]
async fn main() -> webdav_request::Result<()> {
    let client = WebDAVClient::new(WEBDAV_URL, USERNAME, PASSWORD);
    let response = client.get("/path/file").await?;
    if response.status().is_success() {
        let _bytes = response.bytes().await?;
        // TODO
    }
    Ok(())
}

```
