use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct ToString;

register_config!(ToString, "to_string", "@ToString({opts})",
    [
        (
            "include",
            (
                "includeFieldNames = {include_param}",
                [("field_names", "true"), ("none", "false"), ("all", "true")]
            )
        )
    ]);