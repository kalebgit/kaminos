use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

// Bean Validation para required
struct NotNull;
register_config!(NotNull, "required", "@NotNull", []);