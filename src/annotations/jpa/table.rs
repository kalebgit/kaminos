use crate::register_config;
use std::collections::HashMap;
use crate::annotations::{ConfigRegistry, AnnotationProvider};

struct Table;

register_config!(Table, "table", "@Table({opts})",
    [
        (
            "name",
            (
                "name = \"{name_param}\"",
                [("free", "")]
            )
        )
    ]);
