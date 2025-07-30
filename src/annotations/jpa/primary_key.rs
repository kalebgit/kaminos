use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry,AnnotationProvider};

//primary key
struct PrimaryKey;

register_config!(PrimaryKey, "primary_key", "@Id", []);
