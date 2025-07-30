use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct IncludeInToString;

register_config!(IncludeInToString, "include_in_toString", "@ToString.Include", []);
