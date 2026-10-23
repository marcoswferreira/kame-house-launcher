use crate::models::{PairPinRequest, SunshineAppsList};
use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct SunshineClient {
    client: Client,
    pub base_url: String,
    pub csrf_token: Option<String>,
}

impl SunshineClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_default();

        Self {
            client,
            base_url: base_url.into(),
            csrf_token: None,
        }
    }

    pub fn with_client(client: Client, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            csrf_token: None,
        }
    }

    pub async fn fetch_csrf_token(&mut self) -> anyhow::Result<String> {
        let url = format!("{}/api/csrf-token", self.base_url);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let json: Value = resp.json().await?;
        let token = json
            .get("token")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();
        self.csrf_token = Some(token.clone());
        Ok(token)
    }

    pub async fn pair_pin(&self, pin: &str, name: Option<&str>) -> anyhow::Result<()> {
        let url = format!("{}/api/pin", self.base_url);
        let payload = PairPinRequest {
            pin: pin.to_string(),
            name: name.map(|n| n.to_string()),
        };
        let mut req = self.client.post(&url).json(&payload);
        if let Some(token) = &self.csrf_token {
            req = req.header("X-CSRF-Token", token);
        }
        req.send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn get_config(&self) -> anyhow::Result<Value> {
        let url = format!("{}/api/config", self.base_url);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let json: Value = resp.json().await?;
        Ok(json)
    }

    pub async fn update_config(&self, patch: Value) -> anyhow::Result<()> {
        let url = format!("{}/api/config", self.base_url);
        let mut req = self.client.post(&url).json(&patch);
        if let Some(token) = &self.csrf_token {
            req = req.header("X-CSRF-Token", token);
        }
        req.send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn get_apps(&self) -> anyhow::Result<SunshineAppsList> {
        let url = format!("{}/api/apps", self.base_url);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let apps: SunshineAppsList = resp.json().await?;
        Ok(apps)
    }

    pub async fn restart_service(&self) -> anyhow::Result<()> {
        let url = format!("{}/api/restart", self.base_url);
        let mut req = self.client.post(&url);
        if let Some(token) = &self.csrf_token {
            req = req.header("X-CSRF-Token", token);
        }
        let _ = req.send().await;
        Ok(())
    }
}
