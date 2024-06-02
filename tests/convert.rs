extern crate rsxml2json;
use rsxml2json::{Convert, ConvertConfig};
use std::{fs::File, io::Read, path::Path};

pub fn load_xml(file_path: &str) -> String {
    let absolute_path = Path::new(file_path).canonicalize().unwrap();
    let mut file = File::open(absolute_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[test]
fn convert_success() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
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
    };
    let convert = Convert::new(config);

    let xml_input = r#"<?xml version="1.0" encoding="UTF-8"?><hello>world"#.to_string(); // Invalid XML

    assert!(convert.execute(xml_input).is_err());
}

#[test]
fn convert_readme_xml() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
    };
    let convert = Convert::new(config);

    let xml_input = r#"<?xml version="1.0" encoding="UTF-8"?>
<osm version="0.6" generator="CGImap 0.0.2">
<bounds minlat="54.0889580" minlon="12.2487570" maxlat="54.0913900" maxlon="12.2524800"/>
<foo>bar</foo>
</osm>"#
        .to_string();

    assert!(!convert.execute(xml_input).is_err());
}

#[test]
fn convert_test_data_cds() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
    };
    let convert = Convert::new(config);

    let xml_input = load_xml("tests/data/cds.xml"); // Invalid XML

    assert!(!convert.execute(xml_input).is_err());
}

#[test]
fn convert_test_data_def_namespace() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
    };
    let convert = Convert::new(config);

    let xml_input = load_xml("tests/data/def_namespace.xml"); // Invalid XML

    assert!(!convert.execute(xml_input).is_err());
}

#[test]
fn convert_test_data_numerical() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
    };
    let convert = Convert::new(config);

    let xml_input = load_xml("tests/data/numerical.xml"); // Invalid XML

    assert!(!convert.execute(xml_input).is_err());
}

#[test]
fn convert_test_data_xsd() {
    let config = ConvertConfig {
        attribute_prefix: "-".to_string(),
        content_prefix: "#".to_string(),
    };
    let convert = Convert::new(config);

    let xml_input = load_xml("tests/data/xsd.xml"); // Invalid XML

    assert!(!convert.execute(xml_input).is_err());
}
