use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct Data;

register_config!(Data, "data", "@Data", []);
