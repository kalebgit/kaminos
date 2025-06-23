
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_yaml_ng::Value;
use serde_yaml_ng::Value::Mapping;
use handlebars::Handlebars;

//mis modulos
use kaminos::{get_java_type, get_attribute_name};



#[derive(Debug, Serialize)]
pub struct JavaClass {
    class_name: String,
    attributes: Vec<Attribute>
}

#[derive(Debug, Serialize)]
pub struct Attribute {
    attribute_name: String,
    attribute_type: String,
    // annotations: &'static str,
    // code: &'static str,
}

impl Attribute {
    fn new(attribute_name: String, attribute_type: String)->Attribute{
        Attribute {
            attribute_name,
            attribute_type,
            // annotations:
        }

    }
}

impl JavaClass {
    fn new(entity: &Value, class_name: String) -> JavaClass{
        if let Mapping(map) = entity {
            let mut class: JavaClass = JavaClass {
                class_name,
                attributes: Vec::with_capacity(map.capacity()),
            };

            //iterate through each attribute, note there's missing antoher mapping for the configs
            for (attribute_key, attribute_value) in  map {
                let attribute_name: String= get_attribute_name!(attribute_key);
                //we iterate through value
                if let Mapping(configMap) = attribute_value {
                    //TODO
                }
                //it means it is of primitive type
                else {
                    let attribute_type: String = get_java_type!(attribute_value);
                    class.attributes.push( Attribute::new(attribute_name, attribute_type));
                }
            }
            class
        }else {
            panic!("error garrafal")
        }
    }
}


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

