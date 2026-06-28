use phoenix_db::repositories::*;
use phoenix_settings::AppSettings;
use serde::Deserialize;
use tauri::State;

use crate::state::AppState;

// ─── Payloads ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UpdateSettingPayload {
    pub key: String,
    pub value: String,
}

// ─── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let repo = SqlSettingsRepository::new(state.pool.clone());
    let raw = repo.get_all().await.map_err(|e| e.to_string())?;
    let settings = AppSettings {
        theme: raw
            .get("theme")
            .and_then(|v| match v.as_str() {
                "dark" => Some(phoenix_settings::Theme::Dark),
                "light" => Some(phoenix_settings::Theme::Light),
                "system" => Some(phoenix_settings::Theme::System),
                _ => None,
            })
            .unwrap_or_default(),
        language: raw.get("language").cloned().unwrap_or_else(|| "en".into()),
        currency: raw.get("currency").cloned().unwrap_or_else(|| "USD".into()),
        ai_provider: raw
            .get("ai_provider")
            .and_then(|v| match v.as_str() {
                "ollama" => Some(phoenix_settings::AiProvider::Ollama),
                "openai" => Some(phoenix_settings::AiProvider::OpenAi),
                "anthropic" => Some(phoenix_settings::AiProvider::Anthropic),
                "gemini" => Some(phoenix_settings::AiProvider::Gemini),
                "none" => Some(phoenix_settings::AiProvider::None),
                _ => None,
            })
            .unwrap_or_default(),
        sync_enabled: raw
            .get("sync_enabled")
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false),
        auto_lock_minutes: raw
            .get("auto_lock_minutes")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(15),
    };
    Ok(settings)
}

#[tauri::command]
pub async fn update_setting(
    state: State<'_, AppState>,
    payload: UpdateSettingPayload,
) -> Result<(), String> {
    let repo = SqlSettingsRepository::new(state.pool.clone());
    repo.set(&payload.key, &payload.value)
        .await
        .map_err(|e| e.to_string())
}
