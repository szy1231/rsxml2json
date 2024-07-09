use rsxml2json::{Convert, ConvertConfig};

extern crate rsxml2json;
extern crate serde_json;

fn main() {
    let conf = ConvertConfig::default();
    let convert = Convert::new(conf);

    let xml_string = r#"<?xml version="1.0" encoding="UTF-8"?><hello>world</hello>"#;

    // Example 1: Using execute to return a String
    match convert.execute(xml_string.to_string()) {
        Ok(json_str) => {
            println!("JSON String: {}", json_str);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

    // Example 2: Using execute_json to return serde_json::Value
    match convert.execute_json(xml_string.to_string()) {
        Ok(json_value) => {
            let pretty_json = serde_json::to_string_pretty(&json_value)
                .expect("Unable to convert to pretty JSON");
            println!("Pretty JSON:\n{}", pretty_json);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
