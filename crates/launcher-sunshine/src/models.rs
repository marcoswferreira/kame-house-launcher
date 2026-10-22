use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunshineAppPrepCommand {
    #[serde(rename = "do")]
    pub do_cmd: Option<String>,
    pub undo: Option<String>,
    #[serde(default)]
    pub elevated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunshineApp {
    pub name: String,
    pub output: Option<String>,
    pub cmd: Option<String>,
    pub index: Option<i32>,
    pub image_path: Option<String>,
    #[serde(rename = "auto-detach")]
    pub auto_detach: Option<String>,
    #[serde(rename = "wait-all")]
    pub wait_all: Option<String>,
    #[serde(rename = "exit-timeout")]
    pub exit_timeout: Option<String>,
    #[serde(rename = "prep-cmd")]
    pub prep_cmd: Option<Vec<SunshineAppPrepCommand>>,
    #[serde(rename = "working-dir")]
    pub working_dir: Option<String>,
    pub exclude_global_prep_cmd: Option<String>,
    pub elevated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunshineAppsList {
    pub env: Option<HashMap<String, String>>,
    pub apps: Vec<SunshineApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPinRequest {
    pub pin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunshineClientInfo {
    pub uuid: String,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunshineStatus {
    pub running: bool,
    pub streaming: bool,
    pub version: String,
    pub active_client: Option<String>,
}

pub fn attribution_notice() -> &'static str {
    "Streaming powered by Sunshine - Copyright (C) LizardByte & Sunshine Contributors (GPL-3.0). \
    Kame House Launcher extends Sunshine headless infrastructure and credits the upstream authors."
}

