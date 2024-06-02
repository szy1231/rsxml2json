const ATTR_PREFIX: &str = "-";
const CONTENT_PREFIX: &str = "#";

#[derive(Debug)]
pub struct ConvertConfig {
    // Prefix for attribute nodes
    pub attribute_prefix: String,
    // Prefix for content text
    pub content_prefix: String,
}

impl ConvertConfig {
    pub fn init(attribute_prefix: String, content_prefix: String) -> Self {
        Self {
            attribute_prefix,
            content_prefix,
        }
    }
}

impl Default for ConvertConfig {
    fn default() -> Self {
        ConvertConfig {
            attribute_prefix: ATTR_PREFIX.to_string(),
            content_prefix: CONTENT_PREFIX.to_string(),
        }
    }
}
