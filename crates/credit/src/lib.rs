use std::collections::HashMap;

use chrono::{DateTime, Utc};
use phoenix_core::entities::{
    BusinessCreditProfile, CreditAccount, CreditDerogatory, CreditInquiry, CreditReport,
    DisputeCase, DisputeLetter, DisputeLetterTemplate, DisputeTimelineEntry,
};
use phoenix_core::event_bus::{DomainEvent, EventBus};
use phoenix_db::repositories::credit_repair_repo::{
    CreditRepairRepository as _, SqlCreditRepairRepository,
};
use phoenix_db::repositories::credit_repo::{CreditReportRepository as _, SqlCreditReportRepository};
use phoenix_db::DbPool;
use rust_decimal::Decimal;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateCreditReportInput {
    pub score: i32,
    pub score_model: String,
    pub bureau: String,
    pub utilization_rate: Decimal,
    pub on_time_rate: Decimal,
    pub derogatory_marks: i32,
    pub total_accounts: i32,
    pub open_accounts: i32,
    pub hard_inquiries_12m: i32,
    pub report_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateDisputeCaseInput {
    pub negative_item_id: Option<Uuid>,
    pub dispute_type: String,
    pub bureau: String,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct UpdateDisputeCaseInput {
    pub status: Option<String>,
    pub status_basis: Option<String>,
    pub statute_basis: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct CreateDisputeLetterInput {
    pub case_id: Uuid,
    pub template_id: Option<String>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug)]
pub struct CreateCreditAccountInput {
    pub name: String,
    pub issuer: String,
    pub account_type: String,
    pub credit_limit_cents: i64,
    pub balance_cents: i64,
    pub apr: String,
    pub payment_status: String,
    pub opened_date: Option<DateTime<Utc>>,
    pub closed_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct CreateCreditInquiryInput {
    pub bureau: String,
    pub creditor_name: String,
    pub inquiry_type: String,
    pub inquiry_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateDerogatoryItemInput {
    pub creditor_name: String,
    pub account_number_encrypted: Option<String>,
    pub item_type: String,
    pub amount_cents: i64,
    pub original_amount_cents: i64,
    pub status: String,
    pub opened_date: Option<DateTime<Utc>>,
    pub expected_removal_date: Option<DateTime<Utc>>,
    pub bureau: String,
}

#[derive(Debug)]
pub struct CreateBusinessProfileInput {
    pub business_name: String,
    pub ein_encrypted: Option<String>,
    pub duns_number: Option<String>,
    pub paydex_score: Option<i32>,
    pub business_credit_score: Option<i32>,
}

pub struct CreditScorePoint {
    pub date: DateTime<Utc>,
    pub score: i32,
    pub bureau: String,
}

pub struct UtilizationBreakdown {
    pub bureau: String,
    pub utilization_rate: Decimal,
    pub total_accounts: i32,
    pub open_accounts: i32,
}

pub struct DisputeCaseWithCreditor {
    pub case: DisputeCase,
    pub creditor_name: Option<String>,
}

pub struct DisputeCaseDetail {
    pub case: DisputeCase,
    pub timeline: Vec<DisputeTimelineEntry>,
}

pub struct CreditService {
    report_repo: SqlCreditReportRepository,
    repair_repo: SqlCreditRepairRepository,
    event_bus: Option<EventBus>,
}

impl CreditService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            report_repo: SqlCreditReportRepository::new(pool.clone()),
            repair_repo: SqlCreditRepairRepository::new(pool),
            event_bus: None,
        }
    }

    pub fn with_event_bus(pool: DbPool, event_bus: EventBus) -> Self {
        Self {
            report_repo: SqlCreditReportRepository::new(pool.clone()),
            repair_repo: SqlCreditRepairRepository::new(pool),
            event_bus: Some(event_bus),
        }
    }

    #[instrument(skip(self))]
    pub async fn create_report(&self, input: CreateCreditReportInput) -> anyhow::Result<CreditReport> {
        if !(300..=850).contains(&input.score) {
            anyhow::bail!("credit score must be between 300 and 850");
        }
        let report = CreditReport {
            id: Uuid::new_v4(),
            score: input.score,
            score_model: input.score_model,
            bureau: input.bureau,
            utilization_rate: input.utilization_rate,
            on_time_rate: input.on_time_rate,
            derogatory_marks: input.derogatory_marks,
            total_accounts: input.total_accounts,
            open_accounts: input.open_accounts,
            hard_inquiries_12m: input.hard_inquiries_12m,
            report_date: input.report_date,
            created_at: Utc::now(),
        };
        let created = self.report_repo.create(report).await?;

        if let Some(ref bus) = self.event_bus {
            bus.publish(DomainEvent::CreditScoreUpdated {
                report_id: created.id,
                score: created.score,
                timestamp: Utc::now(),
            })
            .await;
        }

        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn get_latest_report(&self) -> anyhow::Result<Option<CreditReport>> {
        self.report_repo.latest().await
    }

    #[instrument(skip(self))]
    pub async fn list_reports(&self) -> anyhow::Result<Vec<CreditReport>> {
        self.report_repo
            .list_by_date_range(
                DateTime::UNIX_EPOCH,
                Utc::now() + chrono::Duration::days(1),
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_credit_score_history(&self) -> anyhow::Result<Vec<CreditScorePoint>> {
        let reports = self
            .report_repo
            .list_by_date_range(
                DateTime::UNIX_EPOCH,
                Utc::now() + chrono::Duration::days(1),
            )
            .await?;
        Ok(reports
            .into_iter()
            .map(|r| CreditScorePoint {
                date: r.report_date,
                score: r.score,
                bureau: r.bureau,
            })
            .collect())
    }

    #[instrument(skip(self))]
    pub async fn get_utilization_breakdown(&self) -> anyhow::Result<Vec<UtilizationBreakdown>> {
        let reports = self
            .report_repo
            .list_by_date_range(
                DateTime::UNIX_EPOCH,
                Utc::now() + chrono::Duration::days(1),
            )
            .await?;

        let mut latest_per_bureau: std::collections::HashMap<String, CreditReport> =
            std::collections::HashMap::new();
        for r in reports {
            let entry = latest_per_bureau.entry(r.bureau.clone());
            match entry {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    if r.report_date > o.get().report_date {
                        o.insert(r);
                    }
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(r);
                }
            }
        }

        Ok(latest_per_bureau
            .into_values()
            .map(|r| UtilizationBreakdown {
                bureau: r.bureau,
                utilization_rate: r.utilization_rate,
                total_accounts: r.total_accounts,
                open_accounts: r.open_accounts,
            })
            .collect())
    }

    // ── Dispute Cases ──────────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn create_dispute_case(&self, input: CreateDisputeCaseInput) -> anyhow::Result<DisputeCase> {
        let now = Utc::now();
        let case = DisputeCase {
            id: Uuid::new_v4(),
            negative_item_id: input.negative_item_id,
            dispute_type: input.dispute_type,
            status: "draft".into(),
            status_basis: None,
            bureau: input.bureau,
            statute_basis: None,
            notes: input.notes,
            created_at: now,
            updated_at: now,
        };
        let created = self.repair_repo.create_dispute_case(case).await?;

        self.repair_repo
            .create_timeline_entry(DisputeTimelineEntry {
                id: Uuid::new_v4(),
                dispute_case_id: created.id,
                event_type: "dispute_filed".into(),
                description: "Dispute case created".into(),
                metadata: None,
                created_at: Utc::now(),
            })
            .await?;

        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn list_dispute_cases(&self) -> anyhow::Result<Vec<DisputeCaseWithCreditor>> {
        let results = self.repair_repo.list_dispute_cases_with_negative().await?;
        Ok(results
            .into_iter()
            .map(|(case, creditor_name)| DisputeCaseWithCreditor {
                case,
                creditor_name,
            })
            .collect())
    }

    #[instrument(skip(self))]
    pub async fn get_dispute_case(&self, id: Uuid) -> anyhow::Result<Option<DisputeCaseDetail>> {
        let case = self.repair_repo.get_dispute_case(id).await?;
        match case {
            Some(case) => {
                let timeline = self.repair_repo.list_timeline_by_case(id).await?;
                Ok(Some(DisputeCaseDetail { case, timeline }))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    pub async fn update_dispute_case(
        &self,
        id: Uuid,
        input: UpdateDisputeCaseInput,
    ) -> anyhow::Result<DisputeCase> {
        let mut case = self
            .repair_repo
            .get_dispute_case(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("dispute case not found"))?;

        if let Some(status) = input.status {
            case.status = status;
        }
        if let Some(basis) = input.status_basis {
            case.status_basis = Some(basis);
        }
        if let Some(basis) = input.statute_basis {
            case.statute_basis = Some(basis);
        }
        if let Some(notes) = input.notes {
            case.notes = Some(notes);
        }
        case.updated_at = Utc::now();

        self.repair_repo.update_dispute_case(case).await
    }

    // ── Dispute Letters ────────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn create_dispute_letter(
        &self,
        input: CreateDisputeLetterInput,
    ) -> anyhow::Result<DisputeLetter> {
        let template = match &input.template_id {
            Some(tid) => Some(
                self.repair_repo
                    .get_template_by_id(tid)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("template not found: {}", tid))?,
            ),
            None => None,
        };

        let generated_content = template.as_ref().map(|t| {
            let mut body = t.body_template.clone();
            for (key, value) in &input.variables {
                body = body.replace(&format!("{{{{{}}}}}", key), value);
            }
            body
        });

        let case = self
            .repair_repo
            .get_dispute_case(input.case_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("dispute case not found"))?;

        let now = Utc::now();
        let recipient_type = template
            .as_ref()
            .map(|t| t.recipient_type.clone())
            .unwrap_or_else(|| "bureau".into());
        let recipient_name = template
            .as_ref()
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        let letter = DisputeLetter {
            id: Uuid::new_v4(),
            dispute_case_id: input.case_id,
            template_id: input.template_id,
            recipient_type,
            recipient_name,
            recipient_address: None,
            generated_content_encrypted: generated_content,
            tracking_number: None,
            sent_date: None,
            response_received_date: None,
            response_summary: None,
            status: "draft".into(),
            created_at: now,
            updated_at: now,
        };

        let created = self.repair_repo.create_dispute_letter(letter).await?;

        self.repair_repo
            .create_timeline_entry(DisputeTimelineEntry {
                id: Uuid::new_v4(),
                dispute_case_id: case.id,
                event_type: "letter_sent".into(),
                description: format!("Letter generated using template {}", template.as_ref().map_or("none", |t| &t.template_code)),
                metadata: None,
                created_at: Utc::now(),
            })
            .await?;

        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn list_dispute_letters(&self, case_id: Uuid) -> anyhow::Result<Vec<DisputeLetter>> {
        self.repair_repo.list_dispute_letters_by_case(case_id).await
    }

    #[instrument(skip(self))]
    pub async fn list_letter_templates(&self) -> anyhow::Result<Vec<DisputeLetterTemplate>> {
        self.repair_repo.list_templates().await
    }

    #[instrument(skip(self))]
    pub async fn get_letter_template(&self, code: &str) -> anyhow::Result<Option<DisputeLetterTemplate>> {
        self.repair_repo.get_template_by_code(code).await
    }

    #[instrument(skip(self))]
    pub async fn mark_letter_sent(
        &self,
        id: Uuid,
        tracking_number: Option<String>,
    ) -> anyhow::Result<DisputeLetter> {
        let mut letter = self
            .repair_repo
            .get_dispute_letter(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("dispute letter not found"))?;

        letter.status = "sent".into();
        letter.tracking_number = tracking_number;
        letter.sent_date = Some(Utc::now());
        letter.updated_at = Utc::now();

        self.repair_repo.update_dispute_letter(letter).await
    }

    // ── Timeline ───────────────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn add_timeline_entry(
        &self,
        case_id: Uuid,
        event_type: String,
        description: String,
    ) -> anyhow::Result<DisputeTimelineEntry> {
        let entry = DisputeTimelineEntry {
            id: Uuid::new_v4(),
            dispute_case_id: case_id,
            event_type,
            description,
            metadata: None,
            created_at: Utc::now(),
        };
        self.repair_repo.create_timeline_entry(entry).await
    }

    #[instrument(skip(self))]
    pub async fn get_timeline(&self, case_id: Uuid) -> anyhow::Result<Vec<DisputeTimelineEntry>> {
        self.repair_repo.list_timeline_by_case(case_id).await
    }

    // ── Credit Accounts ────────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn get_credit_accounts(&self) -> anyhow::Result<Vec<CreditAccount>> {
        self.repair_repo.list_credit_accounts().await
    }

    #[instrument(skip(self))]
    pub async fn create_credit_account(&self, input: CreateCreditAccountInput) -> anyhow::Result<CreditAccount> {
        let now = Utc::now();
        let account = CreditAccount {
            id: Uuid::new_v4(),
            name: input.name,
            issuer: input.issuer,
            account_type: input.account_type,
            credit_limit_cents: input.credit_limit_cents,
            balance_cents: input.balance_cents,
            apr: input.apr,
            payment_status: input.payment_status,
            opened_date: input.opened_date,
            closed_date: input.closed_date,
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        self.repair_repo.create_credit_account(account).await
    }

    #[instrument(skip(self))]
    pub async fn delete_credit_account(&self, id: Uuid) -> anyhow::Result<()> {
        self.repair_repo.delete_credit_account(id).await
    }

    // ── Credit Inquiries ───────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn list_credit_inquiries(&self) -> anyhow::Result<Vec<CreditInquiry>> {
        self.repair_repo.list_credit_inquiries().await
    }

    #[instrument(skip(self))]
    pub async fn create_credit_inquiry(&self, input: CreateCreditInquiryInput) -> anyhow::Result<CreditInquiry> {
        let inquiry = CreditInquiry {
            id: Uuid::new_v4(),
            bureau: input.bureau,
            creditor_name: input.creditor_name,
            inquiry_type: input.inquiry_type,
            inquiry_date: input.inquiry_date,
            created_at: Utc::now(),
        };
        self.repair_repo.create_credit_inquiry(inquiry).await
    }

    // ── Derogatory Items ───────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn list_derogatory_items(&self) -> anyhow::Result<Vec<CreditDerogatory>> {
        self.repair_repo.list_credit_derogatory().await
    }

    #[instrument(skip(self))]
    pub async fn create_derogatory_item(&self, input: CreateDerogatoryItemInput) -> anyhow::Result<CreditDerogatory> {
        let now = Utc::now();
        let item = CreditDerogatory {
            id: Uuid::new_v4(),
            creditor_name: input.creditor_name,
            account_number_encrypted: input.account_number_encrypted,
            item_type: input.item_type,
            amount_cents: input.amount_cents,
            original_amount_cents: input.original_amount_cents,
            status: input.status,
            opened_date: input.opened_date,
            expected_removal_date: input.expected_removal_date,
            is_disputed: false,
            bureau: input.bureau,
            created_at: now,
            updated_at: now,
        };
        self.repair_repo.create_credit_derogatory(item).await
    }

    // ── Business Credit ────────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn get_business_credit_profiles(&self) -> anyhow::Result<Vec<BusinessCreditProfile>> {
        self.repair_repo.list_business_profiles().await
    }

    #[instrument(skip(self))]
    pub async fn create_business_credit_profile(
        &self,
        input: CreateBusinessProfileInput,
    ) -> anyhow::Result<BusinessCreditProfile> {
        let now = Utc::now();
        let profile = BusinessCreditProfile {
            id: Uuid::new_v4(),
            business_name: input.business_name,
            ein_encrypted: input.ein_encrypted,
            duns_number: input.duns_number,
            paydex_score: input.paydex_score,
            business_credit_score: input.business_credit_score,
            created_at: now,
            updated_at: now,
        };
        self.repair_repo.create_business_profile(profile).await
    }

    // ── Delete helpers ─────────────────────────────────────────────────────────

    #[instrument(skip(self))]
    pub async fn delete_credit_report(&self, id: Uuid) -> anyhow::Result<()> {
        self.report_repo.delete(id).await
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
            "CREATE TABLE IF NOT EXISTS credit_reports (
                id TEXT PRIMARY KEY, score INTEGER NOT NULL,
                score_model TEXT NOT NULL DEFAULT 'FICO_8',
                bureau TEXT NOT NULL,
                utilization_rate TEXT NOT NULL DEFAULT '0',
                on_time_rate TEXT NOT NULL DEFAULT '100',
                derogatory_marks INTEGER NOT NULL DEFAULT 0,
                total_accounts INTEGER NOT NULL DEFAULT 0,
                open_accounts INTEGER NOT NULL DEFAULT 0,
                hard_inquiries_12m INTEGER NOT NULL DEFAULT 0,
                report_date TEXT NOT NULL, created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

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
    async fn test_create_dispute_case_via_service() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);

        let case = svc
            .create_dispute_case(CreateDisputeCaseInput {
                negative_item_id: None,
                dispute_type: "inaccuracy".into(),
                bureau: "Experian".into(),
                notes: Some("Test dispute".into()),
            })
            .await
            .unwrap();

        assert_eq!(case.status, "draft");
        assert_eq!(case.bureau, "Experian");

        let cases = svc.list_dispute_cases().await.unwrap();
        assert_eq!(cases.len(), 1);
    }

    #[tokio::test]
    async fn test_generate_letter_from_template() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);

        let case = svc
            .create_dispute_case(CreateDisputeCaseInput {
                negative_item_id: None,
                dispute_type: "inaccuracy".into(),
                bureau: "TransUnion".into(),
                notes: None,
            })
            .await
            .unwrap();

        let mut variables = HashMap::new();
        variables.insert("consumer_name".into(), "John Doe".into());

        let letter = svc
            .create_dispute_letter(CreateDisputeLetterInput {
                case_id: case.id,
                template_id: Some("t01".into()),
                variables,
            })
            .await
            .unwrap();

        assert_eq!(letter.status, "draft");
        assert!(letter
            .generated_content_encrypted
            .unwrap()
            .contains("John Doe"));

        let letters = svc.list_dispute_letters(case.id).await.unwrap();
        assert_eq!(letters.len(), 1);
    }

    #[tokio::test]
    async fn test_list_templates_via_service() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);

        let templates = svc.list_letter_templates().await.unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].template_code, "T01");

        let template = svc
            .get_letter_template("T01")
            .await
            .unwrap()
            .expect("template should exist");
        assert_eq!(template.name, "Section 609 Request");
    }

    #[tokio::test]
    async fn test_get_dispute_case_with_timeline() {
        let pool = create_test_db().await;
        let svc = CreditService::new(pool);

        let case = svc
            .create_dispute_case(CreateDisputeCaseInput {
                negative_item_id: None,
                dispute_type: "obsolete".into(),
                bureau: "Equifax".into(),
                notes: None,
            })
            .await
            .unwrap();

        let timeline_entry = svc
            .add_timeline_entry(case.id, "item_disputed".into(), "Item marked as disputed".into())
            .await
            .unwrap();
        assert_eq!(timeline_entry.event_type, "item_disputed");

        let detail = svc
            .get_dispute_case(case.id)
            .await
            .unwrap()
            .expect("case should exist");
        assert_eq!(detail.timeline.len(), 2); // auto-created + manual entry
    }
}
