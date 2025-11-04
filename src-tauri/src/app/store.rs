use serde_json::json;
use std::sync::Arc;
use tauri::Wry;
use tauri_plugin_store::Store as TauriStore;

pub struct Store {
    instance: Arc<TauriStore<Wry>>,
}

impl Store {
    pub fn new(instance: Arc<TauriStore<Wry>>) -> Store {
        Self { instance }
    }

    pub fn set_string(&self, key: &str, value: &str) {
        let json_value = json!({"value": value});
        self.instance.set(key, json_value);
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.instance
            .get(key)
            .and_then(|value| value.get("value")?.as_str().map(|s| s.to_string()))
    }
}
