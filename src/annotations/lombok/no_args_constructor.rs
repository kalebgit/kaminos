use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct NoArgsConstructor;

register_config!(NoArgsConstructor, "no_args", "@NoArgsConstructor", []);

