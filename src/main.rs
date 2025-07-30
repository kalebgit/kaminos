use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_yaml_ng::Value;
use serde_yaml_ng::Value::Mapping;
use handlebars::Handlebars;
use kaminos::parsing::{JavaClass};





fn main() -> Result<(), Box<dyn std::error::Error>>{
    let content: String = std::fs::read_to_string("assets/products.yml")?;
    let domain: Value = serde_yaml_ng::from_str(&content)?;
    let mut class: JavaClass = JavaClass::new();
    if let Mapping(map) = domain {
        for (entity, attributes_mapping) in map{
            if let Value::String(class_name) = &entity {
                class = JavaClass::new_recursive(&attributes_mapping, class_name.clone())?;
            }
        }
    }


    println!("[Log] la clase final quedo con estos datos: \n{:?}", class);
    // println!("el java class final: {:?}", classes[0]);
    let mut handlebars: Handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);
    handlebars.register_template_file("java_class", "templates/java_class.hbs").unwrap();
    let output = handlebars.render("java_class", &class)?;
    println!("{}", output);


    Ok(())
}

