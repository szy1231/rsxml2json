use rsxml2json::{Convert, ConvertConfig};

#[test]
fn convert_success() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
        validate_json_result: false,
    };
    let convert = Convert::new(config);

    let xml_input = r#"<?xml version="1.0" encoding="UTF-8"?><hello>world</hello>"#.to_string();
    let expected_output = r#"{"hello":"world"}"#.to_string();

    match convert.execute(xml_input) {
        Ok(result) => assert_eq!(result, expected_output),
        Err(e) => panic!("Test failed with error: {:?}", e),
    }
}

#[test]
fn convert_empty_xml() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
        validate_json_result: false,
    };
    let convert = Convert::new(config);

    let xml_input = "".to_string();
    let expected_output = "".to_string();

    match convert.execute(xml_input) {
        Ok(result) => assert_eq!(result, expected_output),
        Err(e) => panic!("Test failed with error: {:?}", e),
    }
}

#[test]
fn convert_invalid_xml() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
        validate_json_result: false,
    };
    let convert = Convert::new(config);

    let xml_input = r#"<?xml version="1.0" encoding="UTF-8"?><hello>world"#.to_string(); // Invalid XML

    assert!(convert.execute(xml_input).is_err());
}
