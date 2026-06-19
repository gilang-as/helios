use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockConfig {
    pub icon_size: u32,
    pub padding: u32,
    pub gap: u32,
    pub apps: Vec<AppConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub icon: String,
    pub exec: String,
}

impl Default for DockConfig {
    fn default() -> Self {
        Self {
            icon_size: 52,
            padding: 10,
            gap: 8,
            apps: vec![
                AppConfig {
                    name: "Files".into(),
                    icon: "folder".into(),
                    exec: "".into(),
                },
                AppConfig {
                    name: "Terminal".into(),
                    icon: "terminal".into(),
                    exec: "".into(),
                },
                AppConfig {
                    name: "Settings".into(),
                    icon: "settings".into(),
                    exec: "".into(),
                },
                AppConfig {
                    name: "Browser".into(),
                    icon: "browser".into(),
                    exec: "".into(),
                },
                AppConfig {
                    name: "Editor".into(),
                    icon: "editor".into(),
                    exec: "".into(),
                },
            ],
        }
    }
}
