use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub platforms: HashMap<String, PlatformInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformInfo {
    pub signature: String,
    pub version: String,
    pub pub_date: String, // 可改为 chrono::DateTime<FixedOffset> 做时间解析
    pub force_update: bool,
    pub notes: Vec<String>,
    pub url: String,
    pub size: u64,
}
