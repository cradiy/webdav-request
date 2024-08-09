pub use reqwest as request;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Deserialize, Debug, Serialize)]
pub struct Multistatus {
    #[serde(rename = "response", default)]
    pub responses: Vec<DResponse>,
}

impl Multistatus {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(s)
    }
    pub async fn from_response(res: Response) -> Result<Self, String> {
        if res.status().is_success() {
            let text = res.text().await.map_err(|err| err.to_string())?;
            Self::from_str(&text).map_err(|err| err.to_string())
        } else {
            Err(res.status().to_string())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DResponse {
    #[serde(rename = "href")]
    pub href: String,
    pub propstat: Propstat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Propstat {
    prop: Prop,
    pub status: String,
}
impl Deref for Propstat {
    type Target = Prop;

    fn deref(&self) -> &Self::Target {
        &self.prop
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Prop {
    #[serde(rename = "displayname")]
    pub display_name: String,
    #[serde(default, rename = "getcontenttype")]
    pub content_type: String,
    #[serde(default, rename = "getlastmodified")]
    pub last_modified: String,
    #[serde(rename = "getcontentlength", default)]
    pub content_length: u64,
    #[serde(alias = "iscollection", default)]
    pub collection: bool,
    #[serde(rename = "resourcetype", default)]
    pub resource_type: Option<ResourceType>,
}
impl Prop {
    pub fn is_collection(&self) -> bool {
        self.collection
            || self
                .resource_type
                .as_ref()
                .is_some_and(|ty| ty.collection.is_some())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceType {
    #[serde(default)]
    collection: Option<String>,
}

#[derive(Default, Debug)]
pub struct FileTree {
    pub href: String,
    pub filename: String,
    pub children: Vec<FileNode>,
}
impl From<Multistatus> for FileTree {
    fn from(value: Multistatus) -> Self {
        if value.responses.is_empty() {
            return Default::default();
        }
        let mut iter = value.responses.into_iter();
        let header_node = iter.next().expect("This will not be empty!");
        FileTree {
            href: header_node.href,
            filename: header_node.propstat.prop.display_name,
            children: iter
                .map(|node| FileNode {
                    is_dir: node.propstat.is_collection(),
                    href: percent_encoding::percent_decode_str(&node.href)
                        .decode_utf8()
                        .map(|s| s.to_string())
                        .unwrap_or(node.href),
                    filename: node.propstat.prop.display_name,
                    last_modified: node.propstat.prop.last_modified,
                    len: node.propstat.prop.content_length,
                    content_type: if !node.propstat.prop.collection {
                        Some(node.propstat.prop.content_type)
                    } else {
                        None
                    },
                })
                .collect(),
        }
    }
}
#[derive(Default, Debug, Clone)]
pub struct FileNode {
    pub is_dir: bool,
    pub href: String,
    pub filename: String,
    pub last_modified: String,
    pub len: u64,
    pub content_type: Option<String>,
}
