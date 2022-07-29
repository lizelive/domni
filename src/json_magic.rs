use std::collections::HashMap;

use serde_json::{Map, Value, Value::*};

pub fn cleanup(value: serde_json::Value) -> Option<serde_json::Value> {
    match value {
        Null => None,
        Bool(v) => Some(Bool(v)),
        Number(v) => Some(Number(v)),
        String(v) => Some(String(v)),
        Array(v) => {
            let values: Vec<Value> = v
                .into_iter()
                //.take_while(|x| !matches!(x, Null))
                .filter_map(cleanup)
                .collect();
            if values.len() > 1 {
                Some(Array(values))
                
            } else {
                let mut values = values;
                values.pop()
            }
        }
        Object(v) => {
            let v: Map<_, _> = v
                .into_iter()
                .filter_map(|(k, v)| {
                    let v = cleanup(v);
                    if let Some(v) = v {
                        Some((k, v))
                    } else {
                        None
                    }
                })
                .collect();
            if v.is_empty() {
                None
            } else {
                Some(Object(v))
            }
        }
    }
}
