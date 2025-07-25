use crate::register_config;
use super::{AnnotationProvider, ConfigRegistry};

//primary key
struct PrimaryKey;

register_config!(PrimaryKey, "primary_key", ["@Id"]);
