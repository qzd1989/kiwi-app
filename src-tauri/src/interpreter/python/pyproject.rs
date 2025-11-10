use anyhow::Result;
use serde::Deserialize;
use std::any::Any;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

use crate::project::Config;

#[derive(Debug, Deserialize)]
pub struct PyProject {
    pub project: Project,
    pub tool: Tool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub description: String,
    readme: Option<String>,
    #[serde(rename = "requires-python")]
    requires_python: Option<String>,
    dependencies: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Tool {
    pub uv: Uv,
}

#[derive(Debug, Deserialize)]
pub struct Uv {
    // key 为包名（如 "kiwi"），value 为对应的 Source
    pub sources: HashMap<String, Source>,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub path: String,
}

impl PyProject {
    pub fn new_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let pyproject: Self = toml::from_str(&content)?;
        Ok(pyproject)
    }
}

impl Config for PyProject {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
