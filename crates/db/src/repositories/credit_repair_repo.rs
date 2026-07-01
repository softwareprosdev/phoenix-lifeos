use async_trait::async_trait;
use phoenix_core::entities::{
    BusinessCreditProfile, CreditAccount, CreditDerogatory, CreditInquiry,
    DisputeCase, DisputeLetter, DisputeLetterTemplate, DisputeTimelineEntry,
    NegativeItem, VendorCreditAccount,
};
use sqlx::Row;
use uuid::Uuid;

use crate::DbPool;

#[async_trait]
pub trait CreditRepairRepository: Send + Sync {
    // CreditAccount
    async fn create_credit_account(&self, entity: CreditAccount) -> anyhow::Result<CreditAccount>;
    async fn get_credit_account(&self, id: Uuid) -> anyhow::Result<Option<CreditAccount>>;
    async fn list_credit_accounts(&self) -> anyhow::Result<Vec<CreditAccount>>;
    async fn update_credit_account(&self, entity: CreditAccount) -> anyhow::Result<CreditAccount>;
    async fn delete_credit_account(&self, id: Uuid) -> anyhow::Result<()>;

    // CreditInquiry
    async fn create_credit_inquiry(&self, entity: CreditInquiry) -> anyhow::Result<CreditInquiry>;
    async fn list_credit_inquiries(&self) -> anyhow::Result<Vec<CreditInquiry>>;

    // CreditDerogatory
    async fn create_credit_derogatory(&self, entity: CreditDerogatory) -> anyhow::Result<CreditDerogatory>;
    async fn list_credit_derogatory(&self) -> anyhow::Result<Vec<CreditDerogatory>>;

    // NegativeItem
    async fn create_negative_item(&self, entity: NegativeItem) -> anyhow::Result<NegativeItem>;
    async fn get_negative_item(&self, id: Uuid) -> anyhow::Result<Option<NegativeItem>>;
    async fn list_negative_items(&self) -> anyhow::Result<Vec<NegativeItem>>;

    // DisputeCase
    async fn create_dispute_case(&self, entity: DisputeCase) -> anyhow::Result<DisputeCase>;
    async fn get_dispute_case(&self, id: Uuid) -> anyhow::Result<Option<DisputeCase>>;
    async fn update_dispute_case(&self, entity: DisputeCase) -> anyhow::Result<DisputeCase>;
    async fn list_dispute_cases(&self) -> anyhow::Result<Vec<DisputeCase>>;
    async fn list_dispute_cases_with_negative(&self) -> anyhow::Result<Vec<(DisputeCase, Option<String>)>>;

    // DisputeLetter
    async fn create_dispute_letter(&self, entity: DisputeLetter) -> anyhow::Result<DisputeLetter>;
    async fn get_dispute_letter(&self, id: Uuid) -> anyhow::Result<Option<DisputeLetter>>;
    async fn update_dispute_letter(&self, entity: DisputeLetter) -> anyhow::Result<DisputeLetter>;
    async fn list_dispute_letters_by_case(&self, case_id: Uuid) -> anyhow::Result<Vec<DisputeLetter>>;

    // DisputeLetterTemplate
    async fn get_template_by_code(&self, code: &str) -> anyhow::Result<Option<DisputeLetterTemplate>>;
    async fn get_template_by_id(&self, id: &str) -> anyhow::Result<Option<DisputeLetterTemplate>>;
    async fn list_templates(&self) -> anyhow::Result<Vec<DisputeLetterTemplate>>;

    // DisputeTimelineEntry
    async fn create_timeline_entry(&self, entity: DisputeTimelineEntry) -> anyhow::Result<DisputeTimelineEntry>;
    async fn list_timeline_by_case(&self, case_id: Uuid) -> anyhow::Result<Vec<DisputeTimelineEntry>>;

    // BusinessCreditProfile
    async fn create_business_profile(&self, entity: BusinessCreditProfile) -> anyhow::Result<BusinessCreditProfile>;
    async fn list_business_profiles(&self) -> anyhow::Result<Vec<BusinessCreditProfile>>;
    async fn get_business_profile(&self, id: Uuid) -> anyhow::Result<Option<BusinessCreditProfile>>;

    // VendorCreditAccount
    async fn create_vendor_account(&self, entity: VendorCreditAccount) -> anyhow::Result<VendorCreditAccount>;
    async fn list_vendor_accounts_by_profile(&self, profile_id: Uuid) -> anyhow::Result<Vec<VendorCreditAccount>>;
}

#[derive(Debug, Clone)]
pub struct SqlCreditRepairRepository {
    pool: DbPool,
}

impl SqlCreditRepairRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

// ─── Mapping functions ──────────────────────────────────────────────────────

fn map_credit_account(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<CreditAccount> {
    Ok(CreditAccount {
        id: row.get::<String, _>("id").parse()?,
        name: row.get("name"),
        issuer: row.get("issuer"),
        account_type: row.get("account_type"),
        credit_limit_cents: row.get("credit_limit_cents"),
        balance_cents: row.get("balance_cents"),
        apr: row.get("apr"),
        payment_status: row.get("payment_status"),
        opened_date: row
            .get::<Option<String>, _>("opened_date")
            .and_then(|d| d.parse().ok()),
        closed_date: row
            .get::<Option<String>, _>("closed_date")
            .and_then(|d| d.parse().ok()),
        is_active: row.get::<i64, _>("is_active") != 0,
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn map_credit_inquiry(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<CreditInquiry> {
    Ok(CreditInquiry {
        id: row.get::<String, _>("id").parse()?,
        bureau: row.get("bureau"),
        creditor_name: row.get("creditor_name"),
        inquiry_type: row.get("inquiry_type"),
        inquiry_date: row.get::<String, _>("inquiry_date").parse()?,
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

fn map_credit_derogatory(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<CreditDerogatory> {
    Ok(CreditDerogatory {
        id: row.get::<String, _>("id").parse()?,
        creditor_name: row.get("creditor_name"),
        account_number_encrypted: row.get("account_number_encrypted"),
        item_type: row.get("item_type"),
        amount_cents: row.get("amount_cents"),
        original_amount_cents: row.get("original_amount_cents"),
        status: row.get("status"),
        opened_date: row
            .get::<Option<String>, _>("opened_date")
            .and_then(|d| d.parse().ok()),
        expected_removal_date: row
            .get::<Option<String>, _>("expected_removal_date")
            .and_then(|d| d.parse().ok()),
        is_disputed: row.get::<i64, _>("is_disputed") != 0,
        bureau: row.get("bureau"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn map_dispute_letter_template(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<DisputeLetterTemplate> {
    let schema_str: String = row.get("variable_schema");
    let variable_schema: Vec<String> = serde_json::from_str(&schema_str)
        .unwrap_or_default();

    Ok(DisputeLetterTemplate {
        id: row.get("id"),
        template_code: row.get("template_code"),
        name: row.get("name"),
        description: row.get("description"),
        legal_basis: row.get("legal_basis"),
        recipient_type: row.get("recipient_type"),
        body_template: row.get("body_template"),
        variable_schema,
        is_active: row.get::<i64, _>("is_active") != 0,
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

fn map_negative_item(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<NegativeItem> {
    Ok(NegativeItem {
        id: row.get::<String, _>("id").parse()?,
        creditor_name: row.get("creditor_name"),
        item_type: row.get("item_type"),
        bureau: row.get("bureau"),
        account_number_encrypted: row.get("account_number_encrypted"),
        amount_cents: row.get("amount_cents"),
        expected_removal_date: row
            .get::<Option<String>, _>("expected_removal_date")
            .and_then(|d| d.parse().ok()),
        is_disputed: row.get::<i64, _>("is_disputed") != 0,
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn map_dispute_case(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<DisputeCase> {
    Ok(DisputeCase {
        id: row.get::<String, _>("id").parse()?,
        negative_item_id: row
            .get::<Option<String>, _>("negative_item_id")
            .and_then(|s| s.parse().ok()),
        dispute_type: row.get("dispute_type"),
        status: row.get("status"),
        status_basis: row.get("status_basis"),
        bureau: row.get("bureau"),
        statute_basis: row.get("statute_basis"),
        notes: row.get("notes"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn map_dispute_letter(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<DisputeLetter> {
    Ok(DisputeLetter {
        id: row.get::<String, _>("id").parse()?,
        dispute_case_id: row.get::<String, _>("dispute_case_id").parse()?,
        template_id: row.get("template_id"),
        recipient_type: row.get("recipient_type"),
        recipient_name: row.get("recipient_name"),
        recipient_address: row.get("recipient_address"),
        generated_content_encrypted: row.get("generated_content_encrypted"),
        tracking_number: row.get("tracking_number"),
        sent_date: row
            .get::<Option<String>, _>("sent_date")
            .and_then(|d| d.parse().ok()),
        response_received_date: row
            .get::<Option<String>, _>("response_received_date")
            .and_then(|d| d.parse().ok()),
        response_summary: row.get("response_summary"),
        status: row.get("status"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn map_timeline_entry(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<DisputeTimelineEntry> {
    Ok(DisputeTimelineEntry {
        id: row.get::<String, _>("id").parse()?,
        dispute_case_id: row.get::<String, _>("dispute_case_id").parse()?,
        event_type: row.get("event_type"),
        description: row.get("description"),
        metadata: row.get("metadata"),
        created_at: row.get::<String, _>("created_at").parse()?,
    })
}

fn map_business_profile(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<BusinessCreditProfile> {
    Ok(BusinessCreditProfile {
        id: row.get::<String, _>("id").parse()?,
        business_name: row.get("business_name"),
        ein_encrypted: row.get("ein_encrypted"),
        duns_number: row.get("duns_number"),
        paydex_score: row.get("paydex_score"),
        business_credit_score: row.get("business_credit_score"),
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

fn map_vendor_account(row: &sqlx::sqlite::SqliteRow) -> anyhow::Result<VendorCreditAccount> {
    Ok(VendorCreditAccount {
        id: row.get::<String, _>("id").parse()?,
        business_credit_profile_id: row.get::<String, _>("business_credit_profile_id").parse()?,
        vendor_name: row.get("vendor_name"),
        account_type: row.get("account_type"),
        credit_limit_cents: row.get("credit_limit_cents"),
        payment_status: row.get("payment_status"),
        reports_to_dnb: row.get::<i64, _>("reports_to_dnb") != 0,
        reports_to_experian: row.get::<i64, _>("reports_to_experian") != 0,
        created_at: row.get::<String, _>("created_at").parse()?,
        updated_at: row.get::<String, _>("updated_at").parse()?,
    })
}

// ─── Async trait implementation ─────────────────────────────────────────────

#[async_trait]
impl CreditRepairRepository for SqlCreditRepairRepository {
    // ── CreditAccount ──────────────────────────────────────────────────────────

    async fn create_credit_account(&self, entity: CreditAccount) -> anyhow::Result<CreditAccount> {
        sqlx::query(
            "INSERT INTO credit_accounts (id, name, issuer, account_type, credit_limit_cents, balance_cents, apr, payment_status, opened_date, closed_date, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(&entity.name)
        .bind(&entity.issuer)
        .bind(&entity.account_type)
        .bind(entity.credit_limit_cents)
        .bind(entity.balance_cents)
        .bind(&entity.apr)
        .bind(&entity.payment_status)
        .bind(entity.opened_date.map(|d| d.to_rfc3339()))
        .bind(entity.closed_date.map(|d| d.to_rfc3339()))
        .bind(entity.is_active as i64)
        .bind(entity.created_at.to_rfc3339())
        .bind(entity.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_credit_account(entity.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created credit account not found"))
    }

    async fn get_credit_account(&self, id: Uuid) -> anyhow::Result<Option<CreditAccount>> {
        let row = sqlx::query("SELECT * FROM credit_accounts WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_credit_account).transpose()
    }

    async fn list_credit_accounts(&self) -> anyhow::Result<Vec<CreditAccount>> {
        let rows = sqlx::query("SELECT * FROM credit_accounts ORDER BY name")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_credit_account).collect()
    }

    async fn update_credit_account(&self, entity: CreditAccount) -> anyhow::Result<CreditAccount> {
        sqlx::query(
            "UPDATE credit_accounts SET name = ?, issuer = ?, account_type = ?, credit_limit_cents = ?, balance_cents = ?, apr = ?, payment_status = ?, opened_date = ?, closed_date = ?, is_active = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&entity.name)
        .bind(&entity.issuer)
        .bind(&entity.account_type)
        .bind(entity.credit_limit_cents)
        .bind(entity.balance_cents)
        .bind(&entity.apr)
        .bind(&entity.payment_status)
        .bind(entity.opened_date.map(|d| d.to_rfc3339()))
        .bind(entity.closed_date.map(|d| d.to_rfc3339()))
        .bind(entity.is_active as i64)
        .bind(entity.updated_at.to_rfc3339())
        .bind(entity.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_credit_account(entity.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated credit account not found"))
    }

    async fn delete_credit_account(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM credit_accounts WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // ── CreditInquiry ──────────────────────────────────────────────────────────

    async fn create_credit_inquiry(&self, entity: CreditInquiry) -> anyhow::Result<CreditInquiry> {
        sqlx::query(
            "INSERT INTO credit_inquiries (id, bureau, creditor_name, inquiry_type, inquiry_date, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(&entity.bureau)
        .bind(&entity.creditor_name)
        .bind(&entity.inquiry_type)
        .bind(entity.inquiry_date.to_rfc3339())
        .bind(entity.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        let row = sqlx::query("SELECT * FROM credit_inquiries WHERE id = ?")
            .bind(entity.id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_credit_inquiry).transpose()
            .map(|o| o.ok_or_else(|| anyhow::anyhow!("created credit inquiry not found")))?
    }

    async fn list_credit_inquiries(&self) -> anyhow::Result<Vec<CreditInquiry>> {
        let rows = sqlx::query("SELECT * FROM credit_inquiries ORDER BY inquiry_date DESC")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_credit_inquiry).collect()
    }

    // ── CreditDerogatory ───────────────────────────────────────────────────────

    async fn create_credit_derogatory(&self, entity: CreditDerogatory) -> anyhow::Result<CreditDerogatory> {
        sqlx::query(
            "INSERT INTO credit_derogatory (id, creditor_name, account_number_encrypted, item_type, amount_cents, original_amount_cents, status, opened_date, expected_removal_date, is_disputed, bureau, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(&entity.creditor_name)
        .bind(&entity.account_number_encrypted)
        .bind(&entity.item_type)
        .bind(entity.amount_cents)
        .bind(entity.original_amount_cents)
        .bind(&entity.status)
        .bind(entity.opened_date.map(|d| d.to_rfc3339()))
        .bind(entity.expected_removal_date.map(|d| d.to_rfc3339()))
        .bind(entity.is_disputed as i64)
        .bind(&entity.bureau)
        .bind(entity.created_at.to_rfc3339())
        .bind(entity.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        let row = sqlx::query("SELECT * FROM credit_derogatory WHERE id = ?")
            .bind(entity.id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_credit_derogatory).transpose()
            .map(|o| o.ok_or_else(|| anyhow::anyhow!("created derogatory item not found")))?
    }

    async fn list_credit_derogatory(&self) -> anyhow::Result<Vec<CreditDerogatory>> {
        let rows = sqlx::query("SELECT * FROM credit_derogatory ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_credit_derogatory).collect()
    }

    // ── NegativeItem ───────────────────────────────────────────────────────────

    async fn create_negative_item(&self, entity: NegativeItem) -> anyhow::Result<NegativeItem> {
        sqlx::query(
            "INSERT INTO negative_items (id, creditor_name, item_type, bureau, account_number_encrypted, amount_cents, expected_removal_date, is_disputed, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(&entity.creditor_name)
        .bind(&entity.item_type)
        .bind(&entity.bureau)
        .bind(&entity.account_number_encrypted)
        .bind(entity.amount_cents)
        .bind(entity.expected_removal_date.map(|d| d.to_rfc3339()))
        .bind(entity.is_disputed as i64)
        .bind(entity.created_at.to_rfc3339())
        .bind(entity.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        let row = sqlx::query("SELECT * FROM negative_items WHERE id = ?")
            .bind(entity.id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_negative_item).transpose()
            .map(|o| o.ok_or_else(|| anyhow::anyhow!("created negative item not found")))?
    }

    async fn get_negative_item(&self, id: Uuid) -> anyhow::Result<Option<NegativeItem>> {
        let row = sqlx::query("SELECT * FROM negative_items WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_negative_item).transpose()
    }

    async fn list_negative_items(&self) -> anyhow::Result<Vec<NegativeItem>> {
        let rows = sqlx::query("SELECT * FROM negative_items ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_negative_item).collect()
    }

    // ── DisputeCase ────────────────────────────────────────────────────────────

    async fn create_dispute_case(&self, entity: DisputeCase) -> anyhow::Result<DisputeCase> {
        sqlx::query(
            "INSERT INTO dispute_cases (id, negative_item_id, dispute_type, status, status_basis, bureau, statute_basis, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(entity.negative_item_id.map(|id| id.to_string()))
        .bind(&entity.dispute_type)
        .bind(&entity.status)
        .bind(&entity.status_basis)
        .bind(&entity.bureau)
        .bind(&entity.statute_basis)
        .bind(&entity.notes)
        .bind(entity.created_at.to_rfc3339())
        .bind(entity.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_dispute_case(entity.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created dispute case not found"))
    }

    async fn get_dispute_case(&self, id: Uuid) -> anyhow::Result<Option<DisputeCase>> {
        let row = sqlx::query("SELECT * FROM dispute_cases WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_dispute_case).transpose()
    }

    async fn update_dispute_case(&self, entity: DisputeCase) -> anyhow::Result<DisputeCase> {
        sqlx::query(
            "UPDATE dispute_cases SET negative_item_id = ?, dispute_type = ?, status = ?, status_basis = ?, bureau = ?, statute_basis = ?, notes = ?, updated_at = ? WHERE id = ?",
        )
        .bind(entity.negative_item_id.map(|id| id.to_string()))
        .bind(&entity.dispute_type)
        .bind(&entity.status)
        .bind(&entity.status_basis)
        .bind(&entity.bureau)
        .bind(&entity.statute_basis)
        .bind(&entity.notes)
        .bind(entity.updated_at.to_rfc3339())
        .bind(entity.id.to_string())
        .execute(&self.pool)
        .await?;
        self.get_dispute_case(entity.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("updated dispute case not found"))
    }

    async fn list_dispute_cases(&self) -> anyhow::Result<Vec<DisputeCase>> {
        let rows = sqlx::query("SELECT * FROM dispute_cases ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_dispute_case).collect()
    }

    async fn list_dispute_cases_with_negative(&self) -> anyhow::Result<Vec<(DisputeCase, Option<String>)>> {
        let rows = sqlx::query(
            "SELECT dc.*, ni.creditor_name FROM dispute_cases dc LEFT JOIN negative_items ni ON dc.negative_item_id = ni.id ORDER BY dc.created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|row| {
                let dc = map_dispute_case(row)?;
                let creditor_name: Option<String> = row.get("creditor_name");
                Ok((dc, creditor_name))
            })
            .collect()
    }

    // ── DisputeLetter ──────────────────────────────────────────────────────────

    async fn create_dispute_letter(&self, entity: DisputeLetter) -> anyhow::Result<DisputeLetter> {
        sqlx::query(
            "INSERT INTO dispute_letters (id, dispute_case_id, template_id, recipient_type, recipient_name, recipient_address, generated_content_encrypted, tracking_number, sent_date, response_received_date, response_summary, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(entity.dispute_case_id.to_string())
        .bind(&entity.template_id)
        .bind(&entity.recipient_type)
        .bind(&entity.recipient_name)
        .bind(&entity.recipient_address)
        .bind(&entity.generated_content_encrypted)
        .bind(&entity.tracking_number)
        .bind(entity.sent_date.map(|d| d.to_rfc3339()))
        .bind(entity.response_received_date.map(|d| d.to_rfc3339()))
        .bind(&entity.response_summary)
        .bind(&entity.status)
        .bind(entity.created_at.to_rfc3339())
        .bind(entity.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        let row = sqlx::query("SELECT * FROM dispute_letters WHERE id = ?")
            .bind(entity.id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_dispute_letter).transpose()
            .map(|o| o.ok_or_else(|| anyhow::anyhow!("created dispute letter not found")))?
    }

    async fn get_dispute_letter(&self, id: Uuid) -> anyhow::Result<Option<DisputeLetter>> {
        let row = sqlx::query("SELECT * FROM dispute_letters WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_dispute_letter).transpose()
    }

    async fn update_dispute_letter(&self, entity: DisputeLetter) -> anyhow::Result<DisputeLetter> {
        sqlx::query(
            "UPDATE dispute_letters SET template_id = ?, recipient_type = ?, recipient_name = ?, recipient_address = ?, generated_content_encrypted = ?, tracking_number = ?, sent_date = ?, response_received_date = ?, response_summary = ?, status = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&entity.template_id)
        .bind(&entity.recipient_type)
        .bind(&entity.recipient_name)
        .bind(&entity.recipient_address)
        .bind(&entity.generated_content_encrypted)
        .bind(&entity.tracking_number)
        .bind(entity.sent_date.map(|d| d.to_rfc3339()))
        .bind(entity.response_received_date.map(|d| d.to_rfc3339()))
        .bind(&entity.response_summary)
        .bind(&entity.status)
        .bind(entity.updated_at.to_rfc3339())
        .bind(entity.id.to_string())
        .execute(&self.pool)
        .await?;
        let row = sqlx::query("SELECT * FROM dispute_letters WHERE id = ?")
            .bind(entity.id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_dispute_letter).transpose()
            .map(|o| o.ok_or_else(|| anyhow::anyhow!("updated dispute letter not found")))?
    }

    async fn list_dispute_letters_by_case(&self, case_id: Uuid) -> anyhow::Result<Vec<DisputeLetter>> {
        let rows = sqlx::query("SELECT * FROM dispute_letters WHERE dispute_case_id = ? ORDER BY created_at DESC")
            .bind(case_id.to_string())
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_dispute_letter).collect()
    }

    // ── DisputeLetterTemplate ──────────────────────────────────────────────────

    async fn get_template_by_code(&self, code: &str) -> anyhow::Result<Option<DisputeLetterTemplate>> {
        let row = sqlx::query("SELECT * FROM dispute_letter_templates WHERE template_code = ?")
            .bind(code)
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_dispute_letter_template).transpose()
    }

    async fn get_template_by_id(&self, id: &str) -> anyhow::Result<Option<DisputeLetterTemplate>> {
        let row = sqlx::query("SELECT * FROM dispute_letter_templates WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_dispute_letter_template).transpose()
    }

    async fn list_templates(&self) -> anyhow::Result<Vec<DisputeLetterTemplate>> {
        let rows = sqlx::query("SELECT * FROM dispute_letter_templates WHERE is_active = 1 ORDER BY template_code")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_dispute_letter_template).collect()
    }

    // ── DisputeTimelineEntry ───────────────────────────────────────────────────

    async fn create_timeline_entry(&self, entity: DisputeTimelineEntry) -> anyhow::Result<DisputeTimelineEntry> {
        sqlx::query(
            "INSERT INTO dispute_timeline (id, dispute_case_id, event_type, description, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(entity.dispute_case_id.to_string())
        .bind(&entity.event_type)
        .bind(&entity.description)
        .bind(&entity.metadata)
        .bind(entity.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        let row = sqlx::query("SELECT * FROM dispute_timeline WHERE id = ?")
            .bind(entity.id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_timeline_entry).transpose()
            .map(|o| o.ok_or_else(|| anyhow::anyhow!("created timeline entry not found")))?
    }

    async fn list_timeline_by_case(&self, case_id: Uuid) -> anyhow::Result<Vec<DisputeTimelineEntry>> {
        let rows = sqlx::query("SELECT * FROM dispute_timeline WHERE dispute_case_id = ? ORDER BY created_at")
            .bind(case_id.to_string())
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_timeline_entry).collect()
    }

    // ── BusinessCreditProfile ──────────────────────────────────────────────────

    async fn create_business_profile(&self, entity: BusinessCreditProfile) -> anyhow::Result<BusinessCreditProfile> {
        sqlx::query(
            "INSERT INTO business_credit_profiles (id, business_name, ein_encrypted, duns_number, paydex_score, business_credit_score, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(&entity.business_name)
        .bind(&entity.ein_encrypted)
        .bind(&entity.duns_number)
        .bind(entity.paydex_score)
        .bind(entity.business_credit_score)
        .bind(entity.created_at.to_rfc3339())
        .bind(entity.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        self.get_business_profile(entity.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created business profile not found"))
    }

    async fn list_business_profiles(&self) -> anyhow::Result<Vec<BusinessCreditProfile>> {
        let rows = sqlx::query("SELECT * FROM business_credit_profiles ORDER BY business_name")
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_business_profile).collect()
    }

    async fn get_business_profile(&self, id: Uuid) -> anyhow::Result<Option<BusinessCreditProfile>> {
        let row = sqlx::query("SELECT * FROM business_credit_profiles WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_business_profile).transpose()
    }

    // ── VendorCreditAccount ────────────────────────────────────────────────────

    async fn create_vendor_account(&self, entity: VendorCreditAccount) -> anyhow::Result<VendorCreditAccount> {
        sqlx::query(
            "INSERT INTO vendor_credit_accounts (id, business_credit_profile_id, vendor_name, account_type, credit_limit_cents, payment_status, reports_to_dnb, reports_to_experian, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(entity.id.to_string())
        .bind(entity.business_credit_profile_id.to_string())
        .bind(&entity.vendor_name)
        .bind(&entity.account_type)
        .bind(entity.credit_limit_cents)
        .bind(&entity.payment_status)
        .bind(entity.reports_to_dnb as i64)
        .bind(entity.reports_to_experian as i64)
        .bind(entity.created_at.to_rfc3339())
        .bind(entity.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        let row = sqlx::query("SELECT * FROM vendor_credit_accounts WHERE id = ?")
            .bind(entity.id.to_string())
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(map_vendor_account).transpose()
            .map(|o| o.ok_or_else(|| anyhow::anyhow!("created vendor account not found")))?
    }

    async fn list_vendor_accounts_by_profile(&self, profile_id: Uuid) -> anyhow::Result<Vec<VendorCreditAccount>> {
        let rows = sqlx::query("SELECT * FROM vendor_credit_accounts WHERE business_credit_profile_id = ? ORDER BY vendor_name")
            .bind(profile_id.to_string())
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(map_vendor_account).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::SqlitePool;

    async fn create_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS credit_accounts (
                id TEXT PRIMARY KEY, name TEXT NOT NULL, issuer TEXT NOT NULL,
                account_type TEXT NOT NULL DEFAULT 'revolving',
                credit_limit_cents INTEGER NOT NULL DEFAULT 0,
                balance_cents INTEGER NOT NULL DEFAULT 0, apr TEXT NOT NULL DEFAULT '0',
                payment_status TEXT NOT NULL DEFAULT 'current', opened_date TEXT,
                closed_date TEXT, is_active INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS credit_inquiries (
                id TEXT PRIMARY KEY, bureau TEXT NOT NULL, creditor_name TEXT NOT NULL,
                inquiry_type TEXT NOT NULL DEFAULT 'hard', inquiry_date TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS credit_derogatory (
                id TEXT PRIMARY KEY, creditor_name TEXT NOT NULL,
                account_number_encrypted TEXT, item_type TEXT NOT NULL,
                amount_cents INTEGER NOT NULL DEFAULT 0,
                original_amount_cents INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'open', opened_date TEXT,
                expected_removal_date TEXT, is_disputed INTEGER NOT NULL DEFAULT 0,
                bureau TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS dispute_letter_templates (
                id TEXT PRIMARY KEY, template_code TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL, description TEXT NOT NULL, legal_basis TEXT,
                recipient_type TEXT NOT NULL DEFAULT 'bureau',
                body_template TEXT NOT NULL, variable_schema TEXT NOT NULL DEFAULT '[]',
                is_active INTEGER NOT NULL DEFAULT 1, created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS negative_items (
                id TEXT PRIMARY KEY, creditor_name TEXT NOT NULL,
                item_type TEXT NOT NULL, bureau TEXT NOT NULL,
                account_number_encrypted TEXT, amount_cents INTEGER NOT NULL DEFAULT 0,
                expected_removal_date TEXT, is_disputed INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS dispute_cases (
                id TEXT PRIMARY KEY, negative_item_id TEXT REFERENCES negative_items(id),
                dispute_type TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'draft',
                status_basis TEXT, bureau TEXT NOT NULL, statute_basis TEXT, notes TEXT,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS dispute_letters (
                id TEXT PRIMARY KEY, dispute_case_id TEXT NOT NULL REFERENCES dispute_cases(id),
                template_id TEXT REFERENCES dispute_letter_templates(id),
                recipient_type TEXT NOT NULL, recipient_name TEXT NOT NULL,
                recipient_address TEXT, generated_content_encrypted TEXT,
                tracking_number TEXT, sent_date TEXT, response_received_date TEXT,
                response_summary TEXT, status TEXT NOT NULL DEFAULT 'draft',
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS dispute_timeline (
                id TEXT PRIMARY KEY, dispute_case_id TEXT NOT NULL REFERENCES dispute_cases(id),
                event_type TEXT NOT NULL, description TEXT NOT NULL, metadata TEXT,
                created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS business_credit_profiles (
                id TEXT PRIMARY KEY, business_name TEXT NOT NULL, ein_encrypted TEXT,
                duns_number TEXT, paydex_score INTEGER, business_credit_score INTEGER,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS vendor_credit_accounts (
                id TEXT PRIMARY KEY, business_credit_profile_id TEXT NOT NULL REFERENCES business_credit_profiles(id),
                vendor_name TEXT NOT NULL, account_type TEXT NOT NULL DEFAULT 'net30',
                credit_limit_cents INTEGER NOT NULL DEFAULT 0,
                payment_status TEXT NOT NULL DEFAULT 'current',
                reports_to_dnb INTEGER NOT NULL DEFAULT 0,
                reports_to_experian INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        // Seed some templates
        sqlx::query(
            "INSERT INTO dispute_letter_templates (id, template_code, name, description, recipient_type, body_template, variable_schema, is_active, created_at) VALUES ('t01', 'T01', 'Section 609 Request', 'Request credit file disclosure', 'bureau', 'Hello {{consumer_name}}, this is a test.', '[\"consumer_name\"]', 1, ?)",
        )
        .bind(Utc::now().to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_dispute_case() {
        let pool = create_test_db().await;
        let repo = SqlCreditRepairRepository::new(pool);
        let case = DisputeCase {
            id: Uuid::new_v4(),
            negative_item_id: None,
            dispute_type: "inaccuracy".into(),
            status: "draft".into(),
            status_basis: None,
            bureau: "Experian".into(),
            statute_basis: None,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let created = repo.create_dispute_case(case).await.unwrap();
        assert_eq!(created.status, "draft");
        assert_eq!(created.bureau, "Experian");

        let list = repo.list_dispute_cases().await.unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_list_templates() {
        let pool = create_test_db().await;
        let repo = SqlCreditRepairRepository::new(pool);
        let templates = repo.list_templates().await.unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].template_code, "T01");
        assert_eq!(templates[0].variable_schema, vec!["consumer_name".to_string()]);
    }

    #[tokio::test]
    async fn test_create_and_generate_letter() {
        let pool = create_test_db().await;
        let repo = SqlCreditRepairRepository::new(pool);

        // Create a dispute case
        let case = DisputeCase {
            id: Uuid::new_v4(),
            negative_item_id: None,
            dispute_type: "inaccuracy".into(),
            status: "draft".into(),
            status_basis: None,
            bureau: "TransUnion".into(),
            statute_basis: None,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let case = repo.create_dispute_case(case).await.unwrap();

        // Create a dispute letter
        let letter = DisputeLetter {
            id: Uuid::new_v4(),
            dispute_case_id: case.id,
            template_id: Some("t01".into()),
            recipient_type: "bureau".into(),
            recipient_name: "Experian".into(),
            recipient_address: None,
            generated_content_encrypted: Some("Test letter content".into()),
            tracking_number: None,
            sent_date: None,
            response_received_date: None,
            response_summary: None,
            status: "draft".into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let created = repo.create_dispute_letter(letter).await.unwrap();
        assert_eq!(created.status, "draft");

        let letters = repo.list_dispute_letters_by_case(case.id).await.unwrap();
        assert_eq!(letters.len(), 1);
    }

    #[tokio::test]
    async fn test_update_dispute_case() {
        let pool = create_test_db().await;
        let repo = SqlCreditRepairRepository::new(pool);

        let case = DisputeCase {
            id: Uuid::new_v4(),
            negative_item_id: None,
            dispute_type: "inaccuracy".into(),
            status: "draft".into(),
            status_basis: None,
            bureau: "Equifax".into(),
            statute_basis: None,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let mut case = repo.create_dispute_case(case).await.unwrap();
        case.status = "sent".into();
        case.notes = Some("Dispute filed".into());

        let updated = repo.update_dispute_case(case).await.unwrap();
        assert_eq!(updated.status, "sent");
        assert_eq!(updated.notes, Some("Dispute filed".into()));
    }

    #[tokio::test]
    async fn test_create_credit_account_and_inquiry() {
        let pool = create_test_db().await;
        let repo = SqlCreditRepairRepository::new(pool);

        let account = CreditAccount {
            id: Uuid::new_v4(),
            name: "Test Card".into(),
            issuer: "Visa".into(),
            account_type: "revolving".into(),
            credit_limit_cents: 1000000,
            balance_cents: 250000,
            apr: "18.99".into(),
            payment_status: "current".into(),
            opened_date: None,
            closed_date: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let created = repo.create_credit_account(account).await.unwrap();
        assert_eq!(created.name, "Test Card");
        assert!(created.is_active);

        let accounts = repo.list_credit_accounts().await.unwrap();
        assert_eq!(accounts.len(), 1);

        // Create inquiry
        let inquiry = CreditInquiry {
            id: Uuid::new_v4(),
            bureau: "Experian".into(),
            creditor_name: "Test Bank".into(),
            inquiry_type: "hard".into(),
            inquiry_date: Utc::now(),
            created_at: Utc::now(),
        };
        let created_inq = repo.create_credit_inquiry(inquiry).await.unwrap();
        assert_eq!(created_inq.creditor_name, "Test Bank");

        let inquiries = repo.list_credit_inquiries().await.unwrap();
        assert_eq!(inquiries.len(), 1);
    }

    #[tokio::test]
    async fn test_timeline_entries() {
        let pool = create_test_db().await;
        let repo = SqlCreditRepairRepository::new(pool);

        let case = DisputeCase {
            id: Uuid::new_v4(),
            negative_item_id: None,
            dispute_type: "inaccuracy".into(),
            status: "draft".into(),
            status_basis: None,
            bureau: "Experian".into(),
            statute_basis: None,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let case = repo.create_dispute_case(case).await.unwrap();

        let entry = DisputeTimelineEntry {
            id: Uuid::new_v4(),
            dispute_case_id: case.id,
            event_type: "letter_sent".into(),
            description: "Initial dispute letter sent".into(),
            metadata: None,
            created_at: Utc::now(),
        };
        let created = repo.create_timeline_entry(entry).await.unwrap();
        assert_eq!(created.event_type, "letter_sent");

        let timeline = repo.list_timeline_by_case(case.id).await.unwrap();
        assert_eq!(timeline.len(), 1);
    }
}
