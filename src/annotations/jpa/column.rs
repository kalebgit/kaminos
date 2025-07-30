use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct Column;

register_config!(Column, "column", "@Column({opts})",
    [
        (
            "name",
            (
                "name = \"{name_param}\"",
                []
            )
        ),
        (
            "nullable",
            (
                "nullable = {nullable_param}",
                [("true", "true"), ("false", "false")]
            )
        ),
        (
            "unique",
            (
                "unique = {unique_param}",
                [("false", "false"), ("true", "true")]
            )
        ),
        (
            "updatable",
            (
                "updatable = {updatable_param}",
                [("true", "true"), ("false", "false")]
            )
        )
    ]);
