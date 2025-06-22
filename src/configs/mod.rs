use inventory;
//modules
pub mod primary_key_config;
pub mod type_config;

//para prop
pub trait AttributeConfig {
    fn get_attribute_name() -> String;
}
//para obtener las anotaciones
pub trait AnnotationProvider {
    fn get_annotations(&self) -> String;
}

//para inyectar codigo
pub trait CodeProvider {
    fn get_code(&self) ->String;
}

//aqui es donde se gestiona la coleccion de configuraciones

//macro donde puedas devolver una instancia del config de acuerdo a su get_attribute_name
macro_rules! get_instance_of {
    ($attribute_name:expr) => {
        
    };
}




pub use primary_key_config::PrimaryKeyConfig;

