use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct AllArgsConstructor;

register_config!(AllArgsConstructor, "all_args", "@AllArgsConstructor", []);
