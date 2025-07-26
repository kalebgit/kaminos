use std::collections::{HashMap};

pub mod primary_key;
mod generated_value;

pub trait AnnotationProvider {
    fn get_annotations(&self, param_value_selected: String)->Vec<String>;
    fn get_key() -> &'static str where Self : Sized;

    //si necesitan parametrizacion
    // fn get_annotation_template(&self) -> &'static str;
    // fn get_default_params(&self) -> HashMap<String, String> {
    //     HashMap::new()
    // }
    // fn set_params(&mut self, param_opts)

}

//usamos registry y factory
pub struct ConfigRegistry {
    pub key: &'static str,
    pub factory:  fn() -> Box<dyn AnnotationProvider>
}

inventory::collect!(ConfigRegistry);

#[macro_export]
macro_rules! register_config {
    ($struct_name:ident, $key: literal, $annotation:literal, [$(($param_name:literal, $param_annotation:literal, [$(($param_opt_name:literal, $param_opt_value:literal)),*])),*]) => {
        impl AnnotationProvider for $struct_name {
            fn get_key() -> &'static str {
                $key
            }
            fn get_annotations(&self, mut param_value_selected: String) -> Vec<String> {

                //creacion del diccionario de todas las anotaciones de los params
                let params: HashMap<(String, String, HashMap<String, String>> = HashMap::new();

                $(
                    let opts_temporal: HashMap<String, String> = HashMap::new();
                    $(
                        opts_temporal.insert($param_annotation.to_string(), $param_opt_name.to_string())
                    )*
                    params.insert($param_name.to_string(), $param_annotation.to_string(), opts_temporal);
                )*




                //TODO: FALTA MODIFICAR TODO LO DE ABAJO PARA HACER ITERAR SOBRE CADA PARAM


                //annotation debe ser reemplazado por param annotation al crear el diccionario
                let mut param_annotation_result: String = $annotation.to_string();

                //creamos el map
                let mut param_opts: HashMap<String, String> = HashMap::new();
                $(
                    param_opts.insert($param.to_string(), $param_annotation.to_string());
                )*

                println!("[log] los param_opts de {} son: {:?}", $key, param_opts);
                // Si no hay parametros para esta anotacio, devolver annotation original
                if param_opts.is_empty() {
                    println!("[log] No hay parámetros definidos, usando anotación original");
                    return vec![param_annotation_result];
                }

                // si hay param_opts tomar el primero como default
                let params_vec: Vec<(String, String)> = vec![$((($param.to_string(), $param_annotation.to_string()))),*];
                // Manejo seguro del valor por defecto
                let default_option:String = if params_vec.is_empty() {
                    String::new() // O cualquier valor por defecto que prefieras
                } else {
                    params_vec.first().unwrap().0.clone()
                };

                // Si no hay parámetros definidos en yaml entonces usamos el valor por defecto
                if param_value_selected.is_empty() || !param_opts.contains_key(&param_value_selected) {
                    param_value_selected = default_option;
                    println!("[log] Usando valor por defecto: {}", param_value_selected);
                }

                //crea un breakpoint aqui por si hace panic
                // se crea el placeholder "{para_selected}"
                let param_value_extracted: &String = param_opts.get(&param_value_selected).unwrap();
                let placeholder: String = format!("{{{}}}", $key);
                println!("el placeholder: {}", placeholder);
                param_annotation_result = param_annotation_result.replace(&placeholder, param_value_extracted);
                println!("[log] param {} tiene parametrizaciones, asi quedo el annotation final: \n============\n{}\n============\n", param_value_selected, param_annotation_result);


                vec![param_annotation_result]
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



