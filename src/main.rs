use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_yaml_ng::Value;
use serde_yaml_ng::Value::Mapping;
use handlebars::Handlebars;
use kaminos::classes::{JavaClass};





fn main() -> Result<(), Box<dyn std::error::Error>>{
    let content: String = std::fs::read_to_string("assets/products.yml")?;
    let domain: Value = serde_yaml_ng::from_str(&content)?;
    let mut classes: Vec<JavaClass> = Vec::new();
    if let Mapping(map) = domain {
        for (entity, attributes_mapping) in map{
            if let Value::String(class_name) = entity {
                classes.push(JavaClass::new(&attributes_mapping, class_name))
            }
        }
    }
    // println!("el java class final: {:?}", classes[0]);
    let mut handlerbars: Handlebars = Handlebars::new();
    handlerbars.register_template_file("java_class", "templates/java_class.hbs").unwrap();
    let output = handlerbars.render("java_class", &classes[0]).unwrap();
    println!("{}", output);
    Ok(())
}

