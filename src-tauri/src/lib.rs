use tauri::Manager;
use tracing::info;

mod commands;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "phoenix_lifeos=info,sqlx=warn".into()),
        )
        .json()
        .init();

    info!(version = env!("CARGO_PKG_VERSION"), "Phoenix LifeOS starting");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let _app_handle = app.handle();
            info!("Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Phoenix LifeOS");
}
