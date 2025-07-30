use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

// Para Hibernate Natural ID (unique business key)
struct NaturalId;

register_config!(NaturalId, "unique", "@NaturalId", []);