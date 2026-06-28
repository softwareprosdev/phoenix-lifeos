/// Placeholder command — remove after real commands are implemented.
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Phoenix LifeOS is alive, {}!", name)
}
