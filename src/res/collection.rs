use super::{multistatus::MultiStatus, privilege::Privilege};

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
                .map(|node| {
                    let href = node.href;
                    let prop = node.prop_stat.prop;
                    Resource {
                        is_collection: prop.is_collection(),
                        #[cfg(feature = "decode_url")]
                        href: percent_encoding::percent_decode_str(&href)
                            .decode_utf8()
                            .map(|s| s.to_string())
                            .unwrap_or(href),
                        #[cfg(not(feature = "decode_url"))]
                        href,
                        display_name: prop.display_name,
                        last_modified: prop.last_modified,
                        len: prop.content_length,
                        content_type: prop.content_type,
                        privilege: prop
                            .current_user_privilege_set
                            .unwrap_or_default()
                            .privilege(),
                    }
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
    pub privilege: Privilege,
}
