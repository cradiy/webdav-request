use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct MultiStatus {
    #[serde(default)]
    pub response: Vec<DResponse>,
}

impl MultiStatus {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(s)
    }
}

#[derive(Debug, Deserialize)]
pub struct DResponse {
    #[serde(rename = "href")]
    pub href: String,
    #[serde(rename = "propstat")]
    pub prop_stat: PropStat,
}

#[derive(Debug, Deserialize, Default)]
pub struct PropStat {
    pub prop: Prop,
    pub status: String,
}

#[derive(Debug, Deserialize, Default)]
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

#[derive(Debug, Deserialize, Default)]
pub struct ResourceType {
    #[serde(default)]
    collection: Option<String>,
}
