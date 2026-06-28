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

            std::fs::create_dir_all(db_path.parent().unwrap())?;

            let db_path_str = db_path.to_string_lossy().to_string();
            let pool = tauri::async_runtime::block_on(async {
                phoenix_db::connection::open(&db_path_str).await
            })?;

            // Run migrations
            tauri::async_runtime::block_on(async {
                phoenix_db::connection::run_migrations(&pool).await
            })?;

            // Store state
            app_handle.manage(state::AppState::new(pool));

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
            commands::debt::make_debt_payment,
            commands::debt::get_payoff_schedule,
            commands::debt::get_debt_summary,
            // Credit
            commands::credit::list_credit_reports,
            commands::credit::create_credit_report,
            commands::credit::get_latest_credit_report,
            commands::credit::get_credit_score_history,
            // Goals
            commands::goals::list_goals,
            commands::goals::get_goal,
            commands::goals::create_goal,
            commands::goals::update_goal,
            commands::goals::get_goal_progress,
            // Investments
            commands::investments::list_investments,
            commands::investments::create_investment,
            commands::investments::update_investment,
            // Vault
            commands::vault::list_vault_items,
            commands::vault::get_vault_item,
            commands::vault::create_vault_item,
            commands::vault::delete_vault_item,
            commands::vault::get_vault_access_log,
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
