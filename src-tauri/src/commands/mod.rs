pub mod budget;
pub mod credit;
pub mod crm;
pub mod debt;
pub mod finance;
pub mod goals;
pub mod investments;
pub mod settings;
pub mod vault;

/// Placeholder command — remove after real commands are implemented.
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Phoenix LifeOS is alive, {}!", name)
}
