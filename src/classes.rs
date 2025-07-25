use serde::Serialize;
use serde_yaml_ng::Value;
use serde_yaml_ng::Value::Mapping;
//mis modulos
use crate::{get_attribute_name, get_java_type};
use crate::annotations::{AnnotationProvider};

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
    pub fn new(attribute_name: String, attribute_type: String)->Attribute{
        Attribute {
            attribute_name,
            attribute_type,
            // annotations:
        }

    }
}

impl JavaClass {
    pub fn new(entity: &Value, class_name: String) -> JavaClass{
        if let Mapping(map) = entity {
            let mut class: JavaClass = JavaClass {
                class_name,
                attributes: Vec::with_capacity(map.capacity()),
            };

            //iterate through each attribute, note there's missing antoher mapping for the configs
            for (attribute_key, attribute_value) in  map {
                let attribute_name: String= get_attribute_name!(attribute_key);

                //we iterate through value, i.e. si tiene configuraciones
                if let Mapping(configMap) = attribute_value {
                    for (config_name, config_value) in configMap {

                    }
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
