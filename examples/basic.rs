use rsxml2json::{Convert, ConvertConfig};

extern crate rsxml2json;
extern crate serde_json;

fn main() {
    let conf = ConvertConfig::default();
    let convert = Convert::new(conf);
    let json_option = convert
        .execute(r#"<?xml version="1.0" encoding="UTF-8"?><hello>world</hello>"#.to_string());
    let json_str = match json_option {
        Ok(value) => value,
        Err(err) => {
            println!("err = {:?}", err);
            return;
        }
    };

    let parsed_json: serde_json::Value =
        serde_json::from_str(json_str.as_str()).expect("Unable to parse JSON");
    let pretty_json =
        serde_json::to_string_pretty(&parsed_json).expect("Unable to convert to pretty JSON");
    println!("{}", pretty_json);
}
