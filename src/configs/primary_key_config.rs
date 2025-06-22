use inventory;
use super::{AnnotationProvider, AttributeConfig};
pub struct PrimaryKeyConfig {

}


impl AttributeConfig for PrimaryKeyConfig {
    fn get_attribute_name() -> String{
        String::from("primary_key")
    }
}

impl AnnotationProvider for PrimaryKeyConfig {
    fn get_annotations(&self) -> String {
        String::from(
            "@Id\n@GeneratedValue(strategy = GenerationType.IDENTITY)"
        )
    }
}


inventory::submit! { PrimaryKeyConfig::get_attribute_name() }