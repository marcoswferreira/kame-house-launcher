use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetKind {
    SharedJsContext,
    Library,
    BigPicture,
    Community,
    Other,
}

pub fn classify_target(title: &str, url: &str) -> TargetKind {
    if title == "SharedJSContext" || url.contains("SharedJSContext") {
        TargetKind::SharedJsContext
    } else if title == "Steam Big Picture Mode" || title.starts_with("MainMenu_") {
        TargetKind::BigPicture
    } else if title == "Steam" && url.contains("steamloopback.host") {
        TargetKind::Library
    } else if url.contains("steamcommunity.com") {
        TargetKind::Community
    } else {
        TargetKind::Other
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementToast {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon_url: String,
    pub app_id: u32,
}

impl AchievementToast {
    pub fn build_eval_script(&self) -> String {
        format!(
            r#"(() => {{
                try {{
                    if (window.SteamClient && window.SteamClient.Notifications) {{
                        window.SteamClient.Notifications.ShowCustomNotification(
                            "{}",
                            "{}",
                            "{}",
                            "{}"
                        );
                        return true;
                    }}
                }} catch (e) {{}}
                return false;
            }})()"#,
            self.title.replace('"', r#"\""#),
            self.description.replace('"', r#"\""#),
            self.icon_url,
            self.app_id
        )
    }
}

#[derive(Debug, Serialize)]
pub struct CdpRequest<'a> {
    pub id: u64,
    pub method: &'a str,
    pub params: serde_json::Value,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<&'a str>,
}

pub fn format_cdp_call(id: u64, method: &str, params: serde_json::Value, session_id: Option<&str>) -> String {
    let req = CdpRequest {
        id,
        method,
        params,
        session_id,
    };
    serde_json::to_string(&req).unwrap_or_default()
}

