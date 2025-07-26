use std::error::Error;

#[derive(Debug)]
pub enum ClassError {
    NotAMapping,
    NoStringKeys,
    ConversionError,
}

impl std::fmt::Display for ClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClassError::NotAMapping => write!(f, "the value is not a mapping/object"),
            ClassError::NoStringKeys => write!(f, "there were no string keys"),
            ClassError::ConversionError => write!(f, "error when parsing key to string"),
        }
    }
}

impl Error for ClassError {}


#[macro_export]
macro_rules! get_java_type {
    ($value:expr) => {
        match $value {
            Value::String(s) => {
                match s.as_str() {
                    "string" =>  "String".to_string() ,
                    "integer" =>  "int".to_string() ,
                    "float" =>  "double".to_string() ,
                    "boolean" =>  "boolean".to_string() ,
                    "time" =>  "LocalDateTime".to_string() ,
                    _ => panic!("Datatype in yml invalid")
                }
            }
            _ => panic!("Datatype in yml invalid")
        }
    };
}

#[macro_export]
macro_rules! get_name {
    ($value:expr) => {
        if let Value::String(attribute_name) = $value{
            attribute_name.clone()
        }else{
            panic!("there was no attribute name")
        }
    };
}


/*

DEBUGGING METHODS

 */


// pub fn get_class_name(class: &Value) -> Result<String, Box<dyn Error>> {
//     if let Value::Mapping(map)= class {
//         for (key, value) in map {
//             if let Value::String(string_key) = key{
//                 return Ok(String::from(string_key))
//             }
//         }
//         Err(Box::new(ClassError::NoStringKeys))
//     }else {
//         Err(Box::new(ClassError::NotAMapping))
//     }
// }
//
// pub fn get_mapping_as_hashmap(map: &Mapping) -> Result<HashMap<String, Value>, Box<dyn Error>> {
//     let mut hashmap: HashMap<String, Value> = HashMap::new();
//     for (key, value) in map {
//         if let Value::String(string_key) = key {
//             hashmap.insert(String::from(string_key), value.clone());
//         }
//     }
//     Ok(hashmap)
// }

// pub fn traverse_attribute(value: &Value) {
//     match value {
//         Value::Mapping(map) => {
//             println!("\n[debug] es un map: {:?}", value);
//             for (key, val) in map {
//                 println!("{:?}: {{", key);
//                 traverse_attribute(val);
//             }
//             println!("}}");
//         }
//         Value::Sequence(seq) => {
//             println!("[debug] es un array: {:?}", value);
//             for (index, item) in seq.iter().enumerate() {
//                 println!("[{:?}]: ", index);
//                 traverse_attribute(item);
//             }
//         }
//         _ => {
//             println!("[debug] es un atributo normal: {:?}", value);
//             println!("{}", get_value_string(value));
//         }
//     }
// }


