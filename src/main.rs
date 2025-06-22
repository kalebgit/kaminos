use serde::{Deserialize, Serialize};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::io::PipeReader;
use kaminos::configs::{PrimaryKeyConfig, AttributeConfig};

#[derive(Debug, Deserialize)]
pub struct Entity {
    //the first string si the entity name
    pub attributes: HashMap<String, HashMap<String, Attribute>>
}


#[derive(Debug, Deserialize)]
// just to know which type it is: "string", "integer", etc.
pub enum Attribute {
    Simple(String),
    Complex(Box<dyn AttributeConfig>)
}


fn main() -> Result<(), serde_yaml_ng::Error>{

    let mut handlebars = Handlebars::new();
    
    handlebars.register_template_file("java_class", )

}
