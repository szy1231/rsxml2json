use crate::config::ConvertConfig;
use roxmltree::{Document, Node};
use serde_json::Value;
use std::{collections::HashMap, error::Error};

pub struct Convert {
    config: ConvertConfig,
}

type ChildrenMap<'a, 'input> = HashMap<String, Vec<Node<'a, 'input>>>;

struct ConversionError;

impl std::fmt::Debug for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conversion Error")
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conversion Error")
    }
}

impl Error for ConversionError {}

impl Convert {
    pub fn new(config: ConvertConfig) -> Self {
        Self { config }
    }

    pub fn execute(&self, xml: String) -> Result<String, Box<dyn Error>> {
        if xml.is_empty() {
            return Ok("".to_string());
        }
        let doc = Document::parse(xml.as_str())?;
        let root = doc.root_element();
        let mut json_string = String::new();
        convert_node_to_json(&mut json_string, root, &self.config);
        if json_string.is_empty() {
            return Err(Box::new(ConversionError));
        }
        json_string = format!("{{\"{}\":{}}}", root.tag_name().name(), json_string);

        is_valid_json(&json_string, self.config.validate_json_result)?;

        Ok(json_string)
    }
}

fn is_valid_json(s: &str, enable: bool) -> Result<(), serde_json::Error> {
    if enable {
        serde_json::from_str::<Value>(s)?;
    }
    Ok(())
}

fn convert_node_to_json(json_output: &mut String, current_node: Node, config: &ConvertConfig) {
    let mut element_count = 0;
    let mut child_elements_map: ChildrenMap = HashMap::new();
    let mut child_text_flag = false;

    for child in current_node.children() {
        if !child.tag_name().name().is_empty() {
            child_elements_map
                .entry(child.tag_name().name().to_string())
                .or_insert_with(Vec::new)
                .push(child);
            element_count += 1;
        }
        if child.is_text() && child.text().unwrap_or("").trim().len() > 0 {
            child_elements_map
                .entry("content".to_string())
                .or_insert_with(Vec::new)
                .push(child);
            child_text_flag = true;
        }
    }

    if element_count > 0 || current_node.attributes().count() > 0 {
        json_output.push_str("{");
    }

    for (i, attr) in current_node.attributes().enumerate() {
        json_output.push_str(&format!(
            r#""{}{}": "{}""#,
            config.attribute_prefix,
            attr.name(),
            attr.value()
        ));
        if element_count > 0 || i < current_node.attributes().count() - 1 || child_text_flag {
            json_output.push_str(", ");
        }
    }

    let mut i = 0;
    for (key, children) in &child_elements_map {
        if !key.eq("content") {
            json_output.push_str(&format!(r#""{}": "#, key));
        } else if element_count > 1 || current_node.attributes().count() > 0 {
            json_output.push_str(&format!(r#""{}content": "#, config.content_prefix));
        }

        if children.len() > 1 {
            json_output.push_str("[");
            for (j, child) in children.iter().enumerate() {
                convert_node_to_json(json_output, *child, config);
                if j < children.len() - 1 {
                    json_output.push_str(", ");
                }
            }
            json_output.push_str("]");
        } else {
            convert_node_to_json(json_output, children[0], config);
        }

        if i < child_elements_map.len() - 1 {
            json_output.push_str(", ");
        }
        i += 1;
    }

    if element_count > 0 || current_node.attributes().count() > 0 {
        json_output.push_str("}");
    } else if current_node.tag_name().name().is_empty()
        && current_node.text().unwrap_or("").trim().len() > 0
    {
        json_output.push_str(&format!(
            r#""{}""#,
            current_node.text().unwrap_or("").trim()
        ));
    }
}
