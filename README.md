# rsxml2json
A Rust library for converting XML to JSON format

### Dependencies:
```rust
use rsxml2json::{Convert, ConvertConfig};
```
### Usage
**Example Code 1: Returning String**
```rust
use rsxml2json::{Convert, ConvertConfig};

fn main() {
    //init
    let convert = Convert::new(ConvertConfig::default());
    //xml data
    let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?><hello>world</hello>"#.to_string();
    
    let data_option = convert.execute(xml_str);
    let json_str = match data_option {
        Ok(value) => value,
        Err(_) => return,
    };
    
    println!("json_str = {}",json_str);
}
```
**Example Code 2: Returning serde_json::Value**
```rust
use rsxml2json::{Convert, ConvertConfig};
use serde_json;

fn main() {
    // Initialize
    let convert = Convert::new(ConvertConfig::default());
    // XML data
    let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?><hello>world</hello>"#.to_string();
    
    let data_option = convert.execute_json(xml_str);
    let json_value = match data_option {
        Ok(value) => value,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };
    
    let pretty_json = serde_json::to_string_pretty(&json_value).expect("Unable to convert to pretty JSON");
    println!("pretty_json = {}", pretty_json);
}
```

**Input**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<osm version="0.6" generator="CGImap 0.0.2">
<bounds minlat="54.0889580" minlon="12.2487570" maxlat="54.0913900" maxlon="12.2524800"/>
<foo>bar</foo>
</osm>
```

**Output**

```json
{
  "osm": {
    "-generator": "CGImap 0.0.2",
    "-version": "0.6",
    "bounds": {
      "-maxlat": "54.0913900",
      "-maxlon": "12.2524800",
      "-minlat": "54.0889580",
      "-minlon": "12.2487570"
    },
    "foo": "bar"
  }
}
```

### Contributing
Feel free to contribute to this project if you want to fix/extend/improve it.

