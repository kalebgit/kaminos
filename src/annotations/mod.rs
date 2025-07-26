use std::collections::{HashMap};

pub mod primary_key;
mod generated_value;

pub trait AnnotationProvider {
    fn get_annotations(&self)->Vec<String>;
    fn get_key() -> &'static str where Self : Sized;

    //si necesitan parametrizacion
    // fn get_annotation_template(&self) -> &'static str;
    // fn get_default_params(&self) -> HashMap<String, String> {
    //     HashMap::new()
    // }
    // fn set_params(&mut self, params)

}

//usamos registry y factory
pub struct ConfigRegistry {
    pub key: &'static str,
    pub factory:  fn() -> Box<dyn AnnotationProvider>
}

inventory::collect!(ConfigRegistry);

#[macro_export]
macro_rules! register_config {
    ($struct_name:ident, $key: literal, $annotation:literal, [$(($param:literal, $param_annotation:literal)),*]) => {
        impl AnnotationProvider for $struct_name {
            fn get_key() -> &'static str {
                $key
            }
            fn get_annotations(&self, param_selected: String) -> Vec<String> {
                let annotation_raw: String = $annotation.to_string();

                //creamos el map
                let mut params = HashMap::new();
                $(
                    params.insert($param, $param_type);
                )*

                //crea un breakpoint aqui por si hace panic
                if let Some(param_extracted) = params.get(param_selected) {

                }else {
                    panic!("No puede ser, no funciono la extraccion de params para {}", $key);
                }

                // vec![$(    ),*]
            }
        }

        inventory::submit! {
            ConfigRegistry {
                key: $key,
                //closure
                factory: | | Box::new($struct_name { })
            }
        }
    };
}

pub fn create_config(key: &String) -> Option<Box<dyn AnnotationProvider>> {
    println!("[log] create_config se recibio el key: {}", key);
    for config in inventory::iter::<ConfigRegistry>{
        if config.key == key.as_str() {
            println!("[log] create_config hemos encontrado la configuracion: {}", config.key);
            return Some((config.factory)()) // lo encerramos en parentesis porque el tipo de dato es una funcion
        }
    }
    None
}



