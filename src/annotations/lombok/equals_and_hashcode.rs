use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct EqualsAndHashCode;

register_config!(EqualsAndHashCode, "equals_and_hashcode", "@EqualsAndHashCode({opts})",
    [
        (
            "only_explicitly_included",
            (
                "onlyExplicitlyIncluded = {only_explicitly_included_param}",
                [("false", "false"), ("true", "true")]
            )
        ),
        (
            "call_super",
            (
                "callSuper = {call_super_param}",
                [("false", "false"), ("true", "true")]
            )
        )
    ]);
