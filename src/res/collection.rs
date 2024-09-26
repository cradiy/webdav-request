use super::multistatus::MultiStatus;

#[derive(Default, Debug)]
pub struct Collection {
    pub href: String,
    pub display_name: String,
    pub children: Vec<Resource>,
}

impl From<MultiStatus> for Collection {
    fn from(value: MultiStatus) -> Self {
        if value.response.is_empty() {
            return Default::default();
        }
        let mut iter = value.response.into_iter();
        let collection = iter.next().expect("never panic!");
        Collection {
            #[cfg(feature = "decode_url")]
            href: percent_encoding::percent_decode_str(&collection.href)
                .decode_utf8()
                .map(|s| s.to_string())
                .unwrap_or(collection.href),
            #[cfg(not(feature = "decode_url"))]
            href: collection.href,
            display_name: collection.prop_stat.prop.display_name,
            children: iter
                .map(|node| Resource {
                    is_collection: node.prop_stat.prop.is_collection(),
                    #[cfg(feature = "decode_url")]
                    href: percent_encoding::percent_decode_str(&node.href)
                        .decode_utf8()
                        .map(|s| s.to_string())
                        .unwrap_or(node.href),
                    #[cfg(not(feature = "decode_url"))]
                    href: node.href,
                    display_name: node.prop_stat.prop.display_name,
                    last_modified: node.prop_stat.prop.last_modified,
                    len: node.prop_stat.prop.content_length,
                    content_type: node.prop_stat.prop.content_type,
                })
                .collect(),
        }
    }
}
#[derive(Default, Debug, Clone)]
pub struct Resource {
    pub is_collection: bool,
    pub href: String,
    pub display_name: String,
    pub last_modified: String,
    pub len: u64,
    pub content_type: String,
}
