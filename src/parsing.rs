use std::collections::HashMap;
use serde_yaml_ng::Value;
use std::io::{Error, ErrorKind};
use serde::Serialize;
use crate::{get_name, get_value_string, get_java_type};
use crate::annotations::create_config;

// configuracion para anotaciones
#[derive(Debug, Serialize)]
pub struct Config {
    config_name: String,
    config_value: String,
    annotation: String
}

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


impl Config {
    pub fn new(config_name: String, config_value: String, annotation:String)->Config {
        Config {
            config_name,
            config_value: config_value.clone(),
            annotation
        }
    }

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

// Enum para clasificar el tipo de nodo
#[derive(Debug, PartialEq)]
enum NodeType {
    EntityOptions,    // opts: lombok, jpa, etc.
    Attributes,       // atributos de la entidad
    Configuration,    // configuraciones específicas dentro de cualquier opt (puede ser de atributo o de configuracion como de lombokk
    Primitive,        // valores primitivos
}

// Estructura para mantener el contexto durante el recorrido
#[derive(Debug)]
struct ParseContext {
    current_path: Vec<String>,
    entity_name: String,
    lombok_configs: HashMap<String, Value>,
    jpa_configs: HashMap<String, Value>,
    attributes: Vec<Attribute>,
}

impl ParseContext {
    fn new(entity_name: String) -> Self {
        Self {
            current_path: Vec::new(),
            entity_name,
            lombok_configs: HashMap::new(),
            jpa_configs: HashMap::new(),
            attributes: Vec::new(),
        }
    }

    fn push_path(&mut self, segment: String) {
        self.current_path.push(segment);
    }

    fn pop_path(&mut self) {
        self.current_path.pop();
    }

    fn current_path_str(&self) -> String {
        self.current_path.join(".")
    }

    fn is_in_opts(&self) -> bool {
        //first ve el del indice 0, i.e. el primero que hizo push
        self.current_path.first().map(|s| s == "opts").unwrap_or(false)
    }

    fn is_lombok_config(&self) -> bool {
        self.current_path.len() >= 2 &&
            self.current_path[0] == "opts" &&
            self.current_path[1] == "lombok"
    }

    fn is_jpa_config(&self) -> bool {
        self.current_path.len() >= 2 &&
            self.current_path[0] == "opts" &&
            self.current_path[1] == "jpa"
    }

    fn is_attribute(&self) -> bool {
        !self.is_in_opts() && !self.current_path.is_empty()
    }
}

impl JavaClass {

    pub fn new()-> Self{
        JavaClass {
            class_name: String::new(),
            attributes: Vec::new()

        }
    }

    pub fn new_recursive(entity: &Value, class_name: String) -> Result<JavaClass, Error> {
        let mut context = ParseContext::new(class_name.clone());

        // Iniciar el recorrido recursivo
        Self::traverse_recursive(entity, &mut context)?;

        println!("[Log] los atributos finales de la clase fueron: \n{:?}", context.attributes);

        // Construir la clase con los datos recolectados
        Ok(JavaClass {
            class_name,
            attributes: context.attributes,
        })
    }

    fn traverse_recursive(value: &Value, context: &mut ParseContext) -> Result<(), Error> {
        match value {
            Value::Mapping(map) => {
                for (key, val) in map {
                    let key_name = get_name!(key);
                    //puede ser opts/attribute/relationships(proximamente)
                    context.push_path(key_name.clone());

                    println!("[DEBUG] Procesando: {} en ruta: {}",
                             key_name, context.current_path_str());

                    // Determinar que tipo de nodo estamos procesando
                    match Self::classify_node(&key_name, context) {
                        NodeType::EntityOptions => {
                            println!("[LOG] Procesando opciones de entidad");
                            //i.e. que tiene que ir un nivel mas abajo para empezar a procesar
                            Self::traverse_recursive(val, context)?;
                        },
                        NodeType::Attributes => {
                            println!("[LOG] Procesando atributo: {}", key_name);
                            Self::process_attribute(&key_name, val, context)?;
                        },
                        NodeType::Configuration => {
                            println!("[LOG] Procesando cofiguracion externa: {}", key_name);
                            Self::process_configuration(&key_name, val, context)?;
                        },
                        NodeType::Primitive => {
                            println!("[LOG] Procesando valor primitivo externa: {}", key_name);
                            Self::process_primitive_value(&key_name, val, context)?;
                        }
                    }

                    context.pop_path();
                }
            },
            Value::Sequence(seq) => {
                for (index, item) in seq.iter().enumerate() {
                    context.push_path(index.to_string());
                    Self::traverse_recursive(item, context)?;
                    context.pop_path();
                }
            },
            _ => {
                // Valor primitivo en contexto específico
                Self::handle_primitive_in_context(value, context)?;
            }
        }
        Ok(())
    }

    fn classify_node(key_name: &str, context: &ParseContext) -> NodeType {
        match key_name {
            "opts" => NodeType::EntityOptions,
            "lombok" | "jpa" if context.is_in_opts() => NodeType::Configuration,
            _ if context.is_in_opts() => NodeType::Configuration,
            _ if !context.is_in_opts() => NodeType::Attributes,
            _ => NodeType::Primitive,
        }
    }

    fn process_attribute(
        attribute_name: &str,
        attribute_value: &Value,
        context: &mut ParseContext
    ) -> Result<(), Error> {
        match attribute_value {
            // Atributo con configuraciones
            Value::Mapping(config_map) => {
                let mut configs: Vec<Config> = Vec::new();
                let mut attribute_type = String::new();

                for (config_key, config_value) in config_map {
                    let config_name = get_name!(config_key);

                    if config_name == "type" {
                        attribute_type = get_java_type!(config_value);
                        continue;
                    }

                    // Procesar configuracion del atributo
                    let config = Self::process_attribute_config(
                        &config_name,
                        config_value,
                        context
                    )?;
                    configs.push(config);
                }

                println!("[Log] las configs finales del atributo fueron: \n{:?}", configs);

                if attribute_type.is_empty() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("No se especificó tipo para atributo: {}", attribute_name)
                    ));
                }

                context.attributes.push(Attribute::new(
                    attribute_name.to_string(),
                    attribute_type,
                    configs
                ));
            },
            // Atributo primitivo
            _ => {
                let attribute_type = get_java_type!(attribute_value);
                context.attributes.push(Attribute::new(
                    attribute_name.to_string(),
                    attribute_type,
                    vec![]
                ));
            }
        }
        Ok(())
    }

    fn process_attribute_config(
        config_name: &str,
        config_value: &Value,
        context: &ParseContext
    ) -> Result<Config, Error> {
        // Crear proveedor de anotaciones
        let provider = create_config(&config_name.to_string())
            .ok_or_else(|| Error::new(
                ErrorKind::InvalidData,
                format!("No se encontró proveedor para configuración: {}", config_name)
            ))?;


        let mut opts: Vec<(String, String)> = Vec::new();
        let config_value_str: String;

        match config_value {
            Value::Mapping(opts_map) => {
                // Tiene opciones específicas (como strategy en generated key)
                println!("[Log] procesando las opciones de {}", config_name);
                for (opt_key, opt_value) in opts_map {
                    opts.push((
                        get_name!(opt_key),
                        get_value_string!(&opt_value)
                    ));
                }
                config_value_str = String::new();
            },
            _ => {
                // Valor simple
                println!("[Log] no hay opciones para esta configuracion, solo un valor...");
                config_value_str = get_value_string!(&config_value);
                opts.push(("single_value".to_string(), config_value_str.clone()));
            }
        }

        let annotation = provider.get_annotations(opts);

        println!("[Log] se obtuve la equita despues de get_annotation: {}", annotation);

        Ok(Config::new(
            config_name.to_string(),
            config_value_str,
            annotation
        ))
    }

    fn process_configuration(
        config_name: &str,
        config_value: &Value,
        context: &mut ParseContext
    ) -> Result<(), Error> {
        if context.is_lombok_config() {
            println!("[LOG] Guardando configuración Lombok: {}", config_name);
            context.lombok_configs.insert(config_name.to_string(), config_value.clone());
        } else if context.is_jpa_config() {
            println!("[LOG] Guardando configuración JPA: {}", config_name);
            context.jpa_configs.insert(config_name.to_string(), config_value.clone());
        } else {
            // Otros tipos de configuración
            Self::traverse_recursive(config_value, context)?;
        }
        Ok(())
    }

    fn process_primitive_value(
        _key: &str,
        value: &Value,
        context: &ParseContext
    ) -> Result<(), Error> {
        let value_str = get_value_string!(&value);
        println!("[LOG] Valor primitivo en {}: {}",
                 context.current_path_str(), value_str);
        Ok(())
    }

    fn handle_primitive_in_context(
        value: &Value,
        context: &ParseContext
    ) -> Result<(), Error> {
        let value_str = get_value_string!(&value);
        println!("[LOG] Valor primitivo en contexto {}: {}",
                 context.current_path_str(), value_str);
        Ok(())
    }

    // Método para aplicar configuraciones de Lombok después del parsing
    pub fn apply_lombok_configurations(&mut self, context: &ParseContext) {
        if !context.lombok_configs.is_empty() {
            println!("[LOG] Aplicando configuraciones Lombok: {:?}", context.lombok_configs);
            // Aquí puedes procesar las configuraciones de Lombok
            // Por ejemplo, agregar anotaciones a nivel de clase
        }
    }

    // Método para aplicar configuraciones de JPA después del parsing
    pub fn apply_jpa_configurations(&mut self, context: &ParseContext) {
        if !context.jpa_configs.is_empty() {
            println!("[LOG] Aplicando configuraciones JPA: {:?}", context.jpa_configs);
            // Aquí puedes procesar las configuraciones de JPA
        }
    }
}

// Función helper para debugging
pub fn debug_traverse_structure(value: &Value, depth: usize) {
    let indent = "  ".repeat(depth);
    match value {
        Value::Mapping(map) => {
            println!("{}📁 Mapping ({} items):", indent, map.len());
            for (key, val) in map {
                println!("{}  🔑 {}: ", indent, get_name!(key));
                debug_traverse_structure(val, depth + 2);
            }
        },
        Value::Sequence(seq) => {
            println!("{}📚 Sequence ({} items):", indent, seq.len());
            for (i, item) in seq.iter().enumerate() {
                println!("{}  [{}]: ", indent, i);
                debug_traverse_structure(item, depth + 2);
            }
        },
        _ => {
            println!("{}📄 Value: {}", indent, get_value_string!(&value));
        }
    }
}