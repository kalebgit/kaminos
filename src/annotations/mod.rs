use std::collections::{HashMap};

pub mod primary_key;
mod generated_value;

pub trait AnnotationProvider {
    fn get_annotations(&self, param_selected: String)->Vec<String>;
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
                let mut annotation_result: String = $annotation.to_string();

                //creamos el map
                let mut params: HashMap<String, String> = HashMap::new();
                $(
                    params.insert($param.to_string(), $param_annotation.to_string());
                )*

                // Si no hay parametros para esta anotacio, devolver annotation original
                if params.is_empty() {
                    return vec![annotation_result];
                }

                // si hay params tomar el primero como default
                let default_value: String = params.get(&param_selected).unwrap().clone();

                // Si no hay parámetros definidos en yaml entonces usamos el valor por defecto
                if param_selected.is_empty() {
                    println!("[log] Usando valor por defecto: {}", default_value);
                }

                //crea un breakpoint aqui por si hace panic
                // se crea el placeholder "{para_selected}"
                let param_value_extracted = params.get(&param_selected).unwrap_or(&default_value);
                let placeholder = format!("{{{}}}", param_selected);
                annotation_result = annotation_result.replace(&placeholder, param_value_extracted);
                println!("[log] param {} tiene parametrizaciones, asi quedo el annotation final: \n============\n{}\n============\n", param_selected, annotation_result);


                vec![annotation_result]
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



