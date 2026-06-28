// Seeds the database with sample financial data for development/testing
use phoenix_db::connection;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

type Id = Uuid;

fn now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn today() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

fn days_ago(n: i64) -> String {
    (Utc::now() - chrono::Duration::days(n))
        .format("%Y-%m-%d")
        .to_string()
}

fn id() -> Id {
    Uuid::new_v4()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_path = std::env::var("PHOENIX_DB_PATH")
        .unwrap_or_else(|_| "phoenix.db".to_string());

    println!("Seeding database: {db_path}");
    let pool = connection::open(&db_path).await?;

    seed(&pool).await?;
    println!("Seeding complete.");
    Ok(())
}

async fn seed(pool: &SqlitePool) -> anyhow::Result<()> {
    let _ts = now();
    let td = today();

    // ── Categories ──────────────────────────────────────────────────────────
    let cat_salary = id();
    let cat_freelance = id();
    let cat_groceries = id();
    let cat_rent = id();
    let cat_utilities = id();
    let cat_dining = id();
    let cat_transport = id();
    let cat_entertainment = id();
    let cat_health = id();
    let cat_shopping = id();

    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_salary).bind("Salary").bind("income").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_freelance).bind("Freelance").bind("income").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_groceries).bind("Groceries").bind("expense").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_rent).bind("Rent").bind("expense").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_utilities).bind("Utilities").bind("expense").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_dining).bind("Dining Out").bind("expense").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_transport).bind("Transport").bind("expense").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_entertainment).bind("Entertainment").bind("expense").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_health).bind("Health").bind("expense").execute(pool).await?;
    sqlx::query("INSERT OR IGNORE INTO categories (id, name, type) VALUES (?1, ?2, ?3)")
        .bind(&cat_shopping).bind("Shopping").bind("expense").execute(pool).await?;

    // ── Accounts ────────────────────────────────────────────────────────────
    let acct_checking = id();
    let acct_savings = id();
    let acct_credit = id();
    let acct_investment = id();
    let acct_cash = id();

    sqlx::query(
        "INSERT OR IGNORE INTO accounts (id, name, account_type, institution, balance_cents, currency) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
        .bind(&acct_checking).bind("Main Checking").bind("checking").bind("First National Bank")
        .bind(425000).bind("USD")
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO accounts (id, name, account_type, institution, balance_cents, currency) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
        .bind(&acct_savings).bind("High-Yield Savings").bind("savings").bind("Online Savings Co")
        .bind(1500000).bind("USD")
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO accounts (id, name, account_type, institution, balance_cents, currency) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
        .bind(&acct_credit).bind("Platinum Rewards Card").bind("credit").bind("Chase")
        .bind(-34500).bind("USD")
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO accounts (id, name, account_type, institution, balance_cents, currency) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
        .bind(&acct_investment).bind("Brokerage Account").bind("investment").bind("Vanguard")
        .bind(5000000).bind("USD")
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO accounts (id, name, account_type, institution, balance_cents, currency) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
        .bind(&acct_cash).bind("Petty Cash").bind("cash").bind("Home")
        .bind(50000).bind("USD")
        .execute(pool).await?;

    // ── Transactions (22) ───────────────────────────────────────────────────
    let d01 = days_ago(1);
    let d02 = days_ago(2);
    let d03 = days_ago(3);
    let d04 = days_ago(4);
    let d05 = days_ago(5);
    let d06 = days_ago(6);
    let d07 = days_ago(7);
    let d08 = days_ago(8);
    let d10 = days_ago(10);

    for (desc, amount, acct, ttype, cat, merchant, date) in [
        ("Salary deposit", 850000, &acct_checking, "income", &cat_salary, "Employer Inc", &d02),
        ("Freelance project", 120000, &acct_checking, "income", &cat_freelance, "Client XYZ", &d03),
        ("Grocery run", -12500, &acct_checking, "expense", &cat_groceries, "Whole Foods", &d04),
        ("Monthly rent", -250000, &acct_checking, "expense", &cat_rent, "Landlord", &d05),
        ("Electric bill", -8900, &acct_checking, "expense", &cat_utilities, "Power Co", &d06),
        ("Dinner out", -6500, &acct_credit, "expense", &cat_dining, "Italian Place", &d01),
        ("Gas station", -4500, &acct_credit, "expense", &cat_transport, "Shell", &d02),
        ("Netflix subscription", -1599, &acct_credit, "expense", &cat_entertainment, "Netflix", &d03),
        ("Pharmacy", -3200, &acct_checking, "expense", &cat_health, "CVS", &d04),
        ("New shoes", -8900, &acct_credit, "expense", &cat_shopping, "Nike", &d05),
        ("Transfer to savings", -200000, &acct_checking, "transfer", &cat_salary, "Internal", &d02),
        ("Transfer from checking", 200000, &acct_savings, "transfer", &cat_salary, "Internal", &d02),
        ("Water bill", -4500, &acct_checking, "expense", &cat_utilities, "Water Utility", &d07),
        ("Internet bill", -7999, &acct_checking, "expense", &cat_utilities, "Comcast", &d07),
        ("Lunch with client", -3500, &acct_credit, "expense", &cat_dining, "Cafe Misto", &d01),
        ("Movie tickets", -2500, &acct_credit, "expense", &cat_entertainment, "AMC", &d03),
        ("Interest payment", 1200, &acct_savings, "income", &cat_salary, "Bank Interest", &d01),
        ("Gym membership", -4999, &acct_checking, "expense", &cat_health, "Planet Fitness", &d06),
        ("Uber ride", -1800, &acct_credit, "expense", &cat_transport, "Uber", &d04),
        ("Amazon order", -6700, &acct_credit, "expense", &cat_shopping, "Amazon", &d02),
        ("Bonus deposit", 250000, &acct_checking, "income", &cat_salary, "Employer Inc", &d10),
        ("Car payment", -45000, &acct_checking, "expense", &cat_transport, "Auto Loan Co", &d08),
    ] {
        sqlx::query(
            "INSERT OR IGNORE INTO transactions \
             (id, account_id, amount_cents, currency, transaction_type, category_id, merchant, description, date) \
             VALUES (?1, ?2, ?3, 'USD', ?4, ?5, ?6, ?7, ?8)"
        )
            .bind(id())
            .bind(acct)
            .bind(amount)
            .bind(ttype)
            .bind(cat)
            .bind(merchant)
            .bind(desc)
            .bind(date)
            .execute(pool).await?;
    }

    // ── Budgets ─────────────────────────────────────────────────────────────
    let budget_monthly = id();
    let budget_yearly = id();

    sqlx::query(
        "INSERT OR IGNORE INTO budgets (id, name, period_type, total_limit_cents, currency, start_date, end_date) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    )
        .bind(&budget_monthly).bind("Monthly Living").bind("monthly").bind(300000)
        .bind("USD").bind(&days_ago(30)).bind(&days_ago(-30))
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO budgets (id, name, period_type, total_limit_cents, currency, start_date, end_date) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    )
        .bind(&budget_yearly).bind("Annual Fun Fund").bind("yearly").bind(2400000)
        .bind("USD").bind(&days_ago(365)).bind(&days_ago(-365))
        .execute(pool).await?;

    // Budget items
    sqlx::query(
        "INSERT OR IGNORE INTO budget_items (id, budget_id, category_id, limit_cents) VALUES (?1, ?2, ?3, ?4)"
    )
        .bind(id()).bind(&budget_monthly).bind(&cat_groceries).bind(60000)
        .execute(pool).await?;
    sqlx::query(
        "INSERT OR IGNORE INTO budget_items (id, budget_id, category_id, limit_cents) VALUES (?1, ?2, ?3, ?4)"
    )
        .bind(id()).bind(&budget_monthly).bind(&cat_dining).bind(30000)
        .execute(pool).await?;
    sqlx::query(
        "INSERT OR IGNORE INTO budget_items (id, budget_id, category_id, limit_cents) VALUES (?1, ?2, ?3, ?4)"
    )
        .bind(id()).bind(&budget_yearly).bind(&cat_entertainment).bind(120000)
        .execute(pool).await?;

    // ── Debts ───────────────────────────────────────────────────────────────
    sqlx::query(
        "INSERT OR IGNORE INTO debts \
         (id, name, debt_type, principal_cents, remaining_balance_cents, interest_rate_pct, minimum_payment_cents, currency, due_date, creditor) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
    )
        .bind(id()).bind("Auto Loan").bind("auto").bind(3500000).bind(3150000)
        .bind("4.5").bind(45000).bind("USD").bind(&days_ago(10)).bind("Capital One Auto")
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO debts \
         (id, name, debt_type, principal_cents, remaining_balance_cents, interest_rate_pct, minimum_payment_cents, currency, due_date, creditor) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
    )
        .bind(id()).bind("Student Loan").bind("student").bind(15000000).bind(12000000)
        .bind("3.2").bind(125000).bind("USD").bind(&days_ago(15)).bind("FedLoan")
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO debts \
         (id, name, debt_type, principal_cents, remaining_balance_cents, interest_rate_pct, minimum_payment_cents, currency, due_date, creditor) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
    )
        .bind(id()).bind("Credit Card Balance").bind("credit_card").bind(50000).bind(34500)
        .bind("22.99").bind(5000).bind("USD").bind(&days_ago(7)).bind("Chase")
        .execute(pool).await?;

    // ── Goals ───────────────────────────────────────────────────────────────
    sqlx::query(
        "INSERT OR IGNORE INTO goals \
         (id, name, goal_type, target_amount_cents, current_amount_cents, monthly_contribution_cents, currency, deadline, linked_account_id) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    )
        .bind(id()).bind("Emergency Fund").bind("savings").bind(3000000).bind(1500000)
        .bind(200000).bind("USD").bind(&days_ago(-365)).bind(&acct_savings)
        .execute(pool).await?;

    sqlx::query(
        "INSERT OR IGNORE INTO goals \
         (id, name, goal_type, target_amount_cents, current_amount_cents, monthly_contribution_cents, currency, deadline, linked_account_id) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    )
        .bind(id()).bind("Vacation Fund").bind("savings").bind(500000).bind(120000)
        .bind(50000).bind("USD").bind(&days_ago(-180)).bind(&acct_savings)
        .execute(pool).await?;

    // ── Credit Report ───────────────────────────────────────────────────────
    sqlx::query(
        "INSERT OR IGNORE INTO credit_reports \
         (id, score, score_model, bureau, utilization_rate, on_time_rate, derogatory_marks, total_accounts, open_accounts, hard_inquiries_12m, report_date) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
    )
        .bind(id()).bind(742).bind("FICO_8").bind("Experian")
        .bind("12.5").bind("100").bind(0).bind(8).bind(5).bind(2).bind(&td)
        .execute(pool).await?;

    Ok(())
}
