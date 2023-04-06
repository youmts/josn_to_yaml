use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("serde_json error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("serde_yaml error")]
    SerdeYaml(#[from] serde_yaml::Error),
}

pub fn json_to_yaml(json_string: String) -> Result<String, LibError> {
    let value_hash_map: HashMap<String, Value> = serde_json::from_str(&json_string)?;
    let yaml_string = serde_yaml::to_string(&value_hash_map)?;

    Ok(yaml_string)
}
