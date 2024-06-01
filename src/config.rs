const ATTR_PREFIX: &str = "-";
const CONTENT_PREFIX: &str = "#";

#[derive(Debug)]
pub struct ConvertConfig {
    pub attribute_prefix: String,
    pub content_prefix: String,
    pub validate_json_result: bool,
}

impl ConvertConfig {
    pub fn init(attribute_prefix: Option<String>, content_prefix: Option<String>,validate_json_result: Option<bool>) -> Self {
        Self {
            attribute_prefix: attribute_prefix.unwrap_or_else(|| ATTR_PREFIX.into()),
            content_prefix: content_prefix.unwrap_or_else(|| CONTENT_PREFIX.into()),
            validate_json_result:validate_json_result.unwrap_or_else(|| true),
        }
    }
    
}

impl Default for ConvertConfig {
    fn default() -> Self {
        ConvertConfig {
            attribute_prefix: ATTR_PREFIX.to_string(),
            content_prefix: CONTENT_PREFIX.to_string(),
            validate_json_result:false,
        }
    }
}
