use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WindowConfig {
    pub window_name: Option<String>,
    pub command: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfig {
    pub session_dir: String,
    pub windows: Vec<WindowConfig>,
}

