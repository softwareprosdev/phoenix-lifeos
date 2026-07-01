use tauri::Manager;
use tracing::info;

mod commands;
mod state;

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
            let app_handle = app.handle();

            // Initialize database
            let db_path = app_handle
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir")
                .join("phoenix.db");

            let data_dir = db_path.parent().unwrap();
            // 0o700: only the owner can list/enter the app data directory
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                std::fs::DirBuilder::new().recursive(true).mode(0o700).create(data_dir)?;
            }
            #[cfg(not(unix))]
            std::fs::create_dir_all(data_dir)?;

            let db_path_str = db_path.to_string_lossy().to_string();
            let pool = tauri::async_runtime::block_on(async {
                phoenix_db::connection::open(&db_path_str).await
            })?;

            // Run migrations
            tauri::async_runtime::block_on(async {
                phoenix_db::connection::run_migrations(&pool).await
            })?;

            // Load or generate the vault encryption key
            let key_path = data_dir.join("vault.key");
            let vault_key: [u8; 32] = if key_path.exists() {
                let bytes = std::fs::read(&key_path)?;
                bytes.try_into().map_err(|_| "vault.key has wrong length — delete it to regenerate")?
            } else {
                use rand::RngCore;
                let mut key = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut key);
                write_secret_file(&key_path, &key)?;
                key
            };

            // Store state
            app_handle.manage(state::AppState::new(pool, vault_key));

            info!("Application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            // Finance
            commands::finance::list_accounts,
            commands::finance::get_account,
            commands::finance::create_account,
            commands::finance::update_account,
            commands::finance::deactivate_account,
            commands::finance::list_transactions,
            commands::finance::get_transaction,
            commands::finance::create_transaction,
            commands::finance::delete_transaction,
            commands::finance::search_transactions,
            commands::finance::list_transactions_by_date_range,
            commands::finance::get_net_worth,
            commands::finance::get_cash_flow,
            commands::finance::get_spending_by_category,
            commands::finance::list_categories,
            commands::finance::create_category,
            // Budget
            commands::budget::list_budgets,
            commands::budget::get_budget,
            commands::budget::create_budget,
            commands::budget::update_budget,
            commands::budget::get_budget_progress,
            commands::budget::list_budget_items,
            commands::budget::create_budget_item,
            // Debt
            commands::debt::list_debts,
            commands::debt::get_debt,
            commands::debt::create_debt,
            commands::debt::update_debt,
            commands::debt::delete_debt,
            commands::debt::make_debt_payment,
            commands::debt::get_payoff_schedule,
            commands::debt::get_debt_summary,
            commands::debt::list_debts_by_type,
            // Credit
            commands::credit::list_credit_reports,
            commands::credit::create_credit_report,
            commands::credit::get_latest_credit_report,
            commands::credit::get_credit_score_history,
            // Credit Repair
            commands::credit::list_dispute_cases,
            commands::credit::create_dispute_case,
            commands::credit::get_dispute_case,
            commands::credit::update_dispute_case,
            commands::credit::list_dispute_letters,
            commands::credit::generate_letter,
            commands::credit::list_letter_templates,
            commands::credit::get_letter_template_by_code,
            commands::credit::mark_letter_sent,
            commands::credit::add_timeline_entry,
            commands::credit::get_credit_accounts,
            commands::credit::create_credit_account,
            commands::credit::delete_credit_account,
            commands::credit::list_credit_inquiries,
            commands::credit::create_credit_inquiry,
            commands::credit::list_derogatory_items,
            commands::credit::create_derogatory_item,
            commands::credit::get_business_credit_profiles,
            commands::credit::create_business_credit_profile,
            commands::credit::delete_credit_report,
            // Goals
            commands::goals::list_goals,
            commands::goals::get_goal,
            commands::goals::create_goal,
            commands::goals::update_goal,
            commands::goals::delete_goal,
            commands::goals::get_goal_progress,
            commands::goals::list_goals_by_type,
            commands::goals::list_active_goals,
            commands::goals::list_goals_by_account,
            // Investments
            commands::investments::list_investments,
            commands::investments::get_investment,
            commands::investments::create_investment,
            commands::investments::update_investment,
            commands::investments::delete_investment,
            commands::investments::list_investments_by_type,
            // Vault
            commands::vault::list_vault_items,
            commands::vault::get_vault_item,
            commands::vault::create_vault_item,
            commands::vault::update_vault_item,
            commands::vault::delete_vault_item,
            commands::vault::search_vault_items,
            commands::vault::get_vault_access_log,
            commands::vault::list_recent_vault_access,
            // Documents
            commands::documents::list_documents,
            commands::documents::get_document,
            commands::documents::create_document,
            commands::documents::update_document,
            commands::documents::delete_document,
            commands::documents::search_documents,
            // CRM
            commands::crm::list_contacts,
            commands::crm::create_contact,
            commands::crm::update_contact,
            commands::crm::delete_contact,
            commands::crm::search_contacts,
            commands::crm::list_contact_interactions,
            // Settings
            commands::settings::get_settings,
            commands::settings::update_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Phoenix LifeOS");
}

/// Write a secret file with owner-only permissions (0o600 on Unix).
/// Uses `create_new` so it fails rather than silently overwrites an existing key.
fn write_secret_file(path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut f = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(path)?;
        f.write_all(data)?;
        f.flush()
    }
    #[cfg(not(unix))]
    {
        // On Windows the data directory ACLs inherited from %APPDATA% already
        // restrict access to the current user, so a plain write is acceptable.
        // A production hardening would use DPAPI or the Windows Credential Store.
        std::fs::write(path, data)
    }
}
