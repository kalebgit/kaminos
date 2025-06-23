
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_yaml_ng::Value;
use serde_yaml_ng::Value::Mapping;
use std::any::Any;

//mis modulos
use kaminos::util::{get_class_name, get_mapping_as_hashmap, traverse_attribute};


pub struct JavaClass {
    class_name: String,
    attributes: Vec<Attribute>
}
pub struct Attribute {
    annotations: String,
    code: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let content: String = std::fs::read_to_string("assets/products.yml")?;
    let entity: Value = serde_yaml_ng::from_str(&content)?;
    let class_name: String = get_class_name(&entity)?;
    println!("el nombre de la clase es: {}", class_name);
    traverse_attribute(&entity);
    Ok(())
}

