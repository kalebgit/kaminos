use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct IncludeInEqualsAndHashCode;

register_config!(IncludeInEqualsAndHashCode, "include_in_equals_and_hashcode", "@EqualsAndHashCode.Include", []);
