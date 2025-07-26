use crate::register_config;
use super::{ConfigRegistry,AnnotationProvider};

struct GeneratedValue{


}

register_config!(GeneratedValue, "generated_value", ["@GeneratedValue(strategy = GenerationType.IDENTITY)"]);