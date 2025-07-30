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
                [("free", "")]
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
        ),
        (
            "max_length",
            (
                "length = {max_length_param}",
                [("free", "")]
            )
        ),
        (
            "min_length",
            (
                "columnDefinition = \"VARCHAR({min_length_param}) CHECK (LENGTH(column_name) >= {min_length_param})\"",
                [("free", "")]
            )
        )
    ]);