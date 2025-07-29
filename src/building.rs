use std::collections::HashMap;
use serde::Serialize;
use serde_yaml_ng::Value;
use serde_yaml_ng::Value::Mapping;

use std::io::{Error, ErrorKind};

//mis modulos
use crate::{get_name, get_java_type, get_value_string};
use crate::annotations::{AnnotationProvider, create_config};

#[derive(Debug, Serialize)]
pub struct JavaClass {
    class_name: String,
    attributes: Vec<Attribute>
}

#[derive(Debug, Serialize)]
pub struct Attribute {
    attribute_name: String,
    attribute_type: String,
    configs: Vec<Config>
}

#[derive(Debug, Serialize)]
pub struct Config {
    config_name: String,
    config_value: String,
    annotation: String
}

impl Attribute {
    pub fn new(attribute_name: String, attribute_type: String, configs: Vec<Config>)->Attribute{
        Attribute {
            attribute_name,
            attribute_type,
            configs
        }

    }
}

impl Config {
    //TODO neceistas pasar una lista de opts del config
    pub fn new(config_name: String, config_value: String, annotation:String)->Config {
        Config {
            config_name,
            config_value: config_value.clone(),
            annotation
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
                    for (config_name_raw, config_value_raw) in configMap {
                        //obtener el tipo de dato
                        let config_name:String = get_name!(config_name_raw);

                        println!("** config_name: {} del atributo {}", config_name, attribute_name);

                        //obtener el tipo de dato
                        if config_name == "type"{
                            attribute_type = get_java_type!(config_value_raw);
                            continue;
                        }


                        //se crea el proveedor de anotaciones de acuerdo la configuracion (GeneratedValue, PrimaryKey, etc)
                        let provider: Box<dyn AnnotationProvider> = create_config(&config_name).unwrap();


                        let mut opts: Vec<(String, String)>= Vec::new();
                        let mut config_value: String;
                        //si tiene mas opciones dentro de esa configuracion
                        if let Mapping(optsMap) = attribute_value {
                            for (opt_name_raw, param_value_raw) in optsMap {
                                opts.push((get_name!(opt_name_raw), get_value_string!(param_value_raw)));
                            }
                            config_value = String::new();

                        }else{
                            config_value = get_value_string!(config_value_raw);
                            opts.push(("single_value".to_string(), config_value.clone()));
                        }


                        let annotation: String = provider.get_annotations(opts);

                        //se crean las configuraciones
                        configs.push(Config::new(
                            config_name,
                            config_value,
                            annotation
                            ))
                    }
                    if attribute_type.is_empty() {
                        //no puede haber algun atributo sin tipo de dato
                        return Err(
                            Error::new(
                                ErrorKind::InvalidData,
                                format!("No se especifico un tipo para clase: {}, en atributo: {}", &class.class_name, attribute_name
                                )
                            ));
                    }
                    println!("[log] los configs de {} son: {:?}",attribute_name, configs);
                    //creamos el atributo con las configs
                    class.attributes.push(Attribute::new(attribute_name, attribute_type, configs))
                }
                    // significa que solo es tipo primitivo
                else {
                    let attribute_type: String = get_java_type!(attribute_value);
                    class.attributes.push( Attribute::new(attribute_name, attribute_type, vec![]));
                }
            }
            Ok(class)
        }else {
            panic!("error garrafal")
        }
    }
}
