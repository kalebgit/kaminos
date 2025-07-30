use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

// Constructors - maneja todos los tipos de constructores
struct Constructors;

register_config!(Constructors, "constructors", "",
    [
        (
            "no_args",
            (
                "@NoArgsConstructor",
                [("false", ""), ("true", "@NoArgsConstructor")]
            )
        ),
        (
            "all_args",
            (
                "@AllArgsConstructor",
                [("false", ""), ("true", "@AllArgsConstructor")]
            )
        ),
        (
            "required_args",
            (
                "@RequiredArgsConstructor",
                [("false", ""), ("true", "@RequiredArgsConstructor")]
            )
        )
    ]);