use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct ExcludeFromEqualsAndHashCode;

register_config!(ExcludeFromEqualsAndHashCode, "exclude_from_equals_and_hashcode", "@EqualsAndHashCode.Exclude", []);