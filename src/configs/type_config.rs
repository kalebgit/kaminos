use std::fmt::format;
use inventory;
use serde::__private::de::TagOrContentField::Tag;
use crate::configs::{AttributeConfig, CodeProvider};

pub struct TypeConfig {
    name: &'static str,
    datatype: &'static str
}

impl TypeConfig {
    fn new(name: &str, datatype: &str) -> TypeConfig{
        TypeConfig {
            name,
            datatype
        }
    }
}

impl AttributeConfig for TypeConfig {
    fn get_attribute_name() -> String{
        String::from("type")
    }
}

impl CodeProvider for TypeConfig {
    fn get_code(&self) -> String {
        let dt: &str = match self.datatype {
            "integer" => "int",
            "string" => "String",
            "time" => "LocalDateTime", 
            _ => panic!("datatype not detected in yml configuration file")
        };
        
        format!("private {} {};", self.datatype, self.name)
        
    }
}