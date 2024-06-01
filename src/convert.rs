use crate::config::ConvertConfig;
use roxmltree::{Document, Node};
use std::collections::HashMap;
use serde_json::Value;

pub struct Convert {
    config: ConvertConfig,
}

type ChildrenMap<'a, 'input> = HashMap<String, Vec<Node<'a, 'input>>>;

impl Convert {
    pub fn new(config: ConvertConfig) -> Self {
        Self {
            config,
        }
    }

    pub fn execute(&self, xml: String) -> Option<String> {
        let doc_res = Document::parse(xml.as_str());
        let doc = match doc_res {
            Ok(data) => {data}
            Err(err) => {
                println!("xml format err = {}",err);
                return None
            }
        };
        let root = doc.root_element();
        let mut json_string = String::new();
        convert_node_to_json(&mut json_string, root, &self.config);
        json_string = format!("{{\"{}\":{}}}",root.tag_name().name(),json_string);

        if self.config.validate_json_result {
            if is_valid_json(&json_string) {
                Some(json_string)
            }else {
                None
            }
        }else {
            if json_string.is_empty() {
                None
            }else {
                Some(json_string)
            }
        }

    }
}

fn is_valid_json(s: &str) -> bool {
    serde_json::from_str::<Value>(s).is_ok()
}

fn convert_node_to_json(json_output: &mut String, current_node: Node, config: &ConvertConfig) {
    let mut element_count = 0;
    let mut child_elements_map: ChildrenMap = HashMap::new();

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
        }
    }

    if element_count > 0 || current_node.attributes().count() > 0 {
        json_output.push_str("{");
    }

    for (i, attr) in current_node.attributes().enumerate() {
        json_output.push_str(&format!(r#""{}{}": "{}""#, config.attribute_prefix,attr.name(), attr.value()));
        if element_count > 0 || i < current_node.attributes().count() - 1 {
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
    } else if !current_node.tag_name().name().is_empty()
        && current_node.text().unwrap_or("").trim().len() > 0
    {
        json_output.push_str(&format!(
            r#""{}""#,
            current_node.text().unwrap_or("").trim()
        ));
    }
}
