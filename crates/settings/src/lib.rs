use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: Theme,
    pub language: String,
    pub currency: String,
    pub ai_provider: AiProvider,
    pub sync_enabled: bool,
    pub auto_lock_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AiProvider {
    #[default]
    Ollama,
    OpenAi,
    Anthropic,
    Gemini,
    None,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            language: "en".into(),
            currency: "USD".into(),
            ai_provider: AiProvider::Ollama,
            sync_enabled: false,
            auto_lock_minutes: 15,
        }
    }
}
