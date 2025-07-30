use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct ToString;

register_config!(ToString, "to_string", "@ToString({opts})",
    [
        (
            "only_explicitly_included",
            (
                "onlyExplicitlyIncluded = {only_explicitly_included_param}",
                [("false", "false"), ("true", "true")]
            )
        ),
        (
            "include_field_names",
            (
                "includeFieldNames = {include_field_names_param}",
                [("true", "true"), ("false", "false")]
            )
        )
    ]);
