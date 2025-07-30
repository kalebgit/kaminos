use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct GeneratedValue;

register_config!(GeneratedValue, "generated_value", "@GeneratedValue({opts})",
    [
        (
            "strategy",
            (
                "strategy = {strategy_param}",
                [("identity", "GenerationType.IDENTITY"),
                ("sequence", "GenerationType.SEQUENCE"),
                ("auto", "GenerationType.AUTO"),
                ("uuid", "GenerationType.UUID")]
            )
        )
    ]);