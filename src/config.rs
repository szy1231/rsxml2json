const ATTR_PREFIX: &str = "-";
const CONTENT_PREFIX: &str = "#";

#[derive(Debug)]
pub struct ConvertConfig {
    pub attribute_prefix: String,
    pub content_prefix: String,
    pub validate_json_result: bool,
}

impl ConvertConfig {
    pub fn init(
        attribute_prefix: String,
        content_prefix: String,
        validate_json_result: bool,
    ) -> Self {
        Self {
            attribute_prefix,
            content_prefix,
            validate_json_result,
        }
    }
}

impl Default for ConvertConfig {
    fn default() -> Self {
        ConvertConfig {
            attribute_prefix: ATTR_PREFIX.to_string(),
            content_prefix: CONTENT_PREFIX.to_string(),
            validate_json_result: false,
        }
    }
}
