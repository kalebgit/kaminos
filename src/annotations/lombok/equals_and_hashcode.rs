use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct EqualsAndHashCode;

register_config!(EqualsAndHashCode, "equals_and_hashcode", "@EqualsAndHashCode({opts})",
    [
        (
            "include",
            (
                "onlyExplicitlyIncluded = {include_param}",
                [("all", "false"), ("explicit", "true"), ("none", "true")]
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
