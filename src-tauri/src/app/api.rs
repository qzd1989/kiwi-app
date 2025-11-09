use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::time::Duration;

const API_URL_DEV: &str = "http://localhost:9992";
const API_URL_PRO: &str = "https://kiwi.biexi.com";

pub struct Api {
    path: String,
    request_type: RequestType,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout_secs: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RequestType {
    Get,
    Post,
}

impl Api {
    fn new(path: impl Into<String>, request_type: RequestType) -> Self {
        Self {
            path: path.into(),
            request_type,
            params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
            timeout_secs: None,
        }
    }

    pub fn with_params<I, K, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.params = iter
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    pub async fn get<T: DeserializeOwned, I, K, V>(
        path: impl Into<String>, // /version.json
        params: I,
    ) -> Result<T>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let url = {
            if cfg!(debug_assertions) {
                API_URL_DEV
            } else {
                API_URL_PRO
            }
        };
        let full_path = format!("{}{}", url, path.into());
        let api = Self::new(full_path, RequestType::Get).with_params(params);

        api.with_timeout(10).send().await
    }

    fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    async fn send<T: DeserializeOwned>(&self) -> Result<T> {
        let client_builder = Client::builder();

        let client = if let Some(secs) = self.timeout_secs {
            client_builder
                .timeout(Duration::from_secs(secs))
                .build()
                .map_err(|e| anyhow!(e.to_string()))?
        } else {
            client_builder.build().map_err(|e| anyhow!(e.to_string()))?
        };

        let mut request = match self.request_type {
            RequestType::Get => client.get(&self.path).query(&self.params),
            RequestType::Post => {
                let req = client.post(&self.path);
                if let Some(ref body) = self.body {
                    req.body(body.clone())
                } else {
                    req.form(&self.params)
                }
            }
        };

        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let res = request.send().await.map_err(|e| anyhow!(e.to_string()))?;

        res.json::<T>().await.map_err(|e| anyhow!(e.to_string()))
    }
}
