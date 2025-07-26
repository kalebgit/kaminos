use serde::Serialize;
use serde_yaml_ng::Value;
use serde_yaml_ng::Value::Mapping;

use std::io::{Error, ErrorKind};

//mis modulos
use crate::{get_name, get_java_type};
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
    configs: Option<Vec<Config>>
    // annotations: &'static str,
    // code: &'static str,
}

#[derive(Debug, Serialize)]
pub struct Config {
    config_name: String,
    config_value: String,
}

impl Attribute {
    pub fn new(attribute_name: String, attribute_type: String, configs: Option<Vec<Config>>)->Attribute{
        Attribute {
            attribute_name,
            attribute_type,
            configs
            // annotations:
        }

    }
}

impl Config {
    pub fn new(config_name: String, config_value: String)->Config {
        Config {
            config_name,
            config_value,
        }
    }
}

impl JavaClass {
    pub fn new(entity: &Value, class_name: String) -> Result<JavaClass, Error> {
        if let Mapping(map) = entity {
            let mut class: JavaClass = JavaClass {
                class_name,
                attributes: Vec::with_capacity(map.capacity()),
            };

            //iterate through each attribute, note there's missing antoher mapping for the configs
            for (attribute_key, attribute_value) in  map {
                let attribute_name: String= get_name!(attribute_key);


                //we iterate through value, i.e. si tiene configuraciones
                if let Mapping(configMap) = attribute_value {
                    let mut configs: Vec<Config> = Vec::with_capacity(configMap.capacity());
                    let mut attribute_type:String = String::from("");
                    for (config_name, config_value) in configMap {
                        //obtener el tipo de dato
                        if config_name == "type"{
                            attribute_type = get_java_type!(config_value);
                        }else{
                            //no puede haber algun atributo sin tipo de dato
                            return Err(
                                Error::new(
                                    ErrorKind::InvalidData,
                                    format!("No se especifico un tipo para clase: {}, en atributo: {}", &class.class_name, attribute_name
                                    )
                                ));
                        }
                        configs.push(Config::new(get_name!(config_name), get_name!(config_value)))
                    }
                    class.attributes.push(Attribute::new(attribute_name, attribute_type, Some(configs)))
                }
                    // significa que solo es tipo primitivo
                else {
                    let attribute_type: String = get_java_type!(attribute_value);
                    class.attributes.push( Attribute::new(attribute_name, attribute_type, None));
                }
            }
            Ok(class)
        }else {
            panic!("error garrafal")
        }
    }
}
