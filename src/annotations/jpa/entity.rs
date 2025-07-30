use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct Entity;

register_config!(Entity, "entity", "@Entity", []);