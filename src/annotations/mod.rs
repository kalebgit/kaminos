pub mod primary_key;
mod generated_value;

pub trait AnnotationProvider {
    fn get_annotations(&self)->Vec<String>;
    fn get_key() -> &'static str where Self : Sized;
}

//usamos registry y factory
pub struct ConfigRegistry {
    pub key: &'static str,
    pub factory:  fn() -> Box<dyn AnnotationProvider>
}

inventory::collect!(ConfigRegistry);

#[macro_export]
macro_rules! register_config {
    ($struct_name:ident, $key: literal, [$($annotation:literal),*]) => {
        impl AnnotationProvider for $struct_name {
            fn get_key() -> &'static str {
                $key
            }
            fn get_annotations(&self) -> Vec<String> {
                vec![$($annotation.to_string()),*]
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

