use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub name: String,
    pub version: String,
    pub description: String,
    pub path: String,
}

impl Info {
    pub fn new(name: String, version: String, description: String, path: String) -> Self {
        Self {
            name,
            version,
            description,
            path,
        }
    }
}
