use chrono::{DateTime, Utc};
use phoenix_core::entities::Goal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    Scheduled,
    EventDriven,
    Threshold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub trigger_type: TriggerType,
    pub trigger_config: String,
    pub action_config: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationSummary {
    pub total_rules: usize,
    pub enabled_rules: usize,
    pub last_triggered: Option<DateTime<Utc>>,
}

#[derive(Error, Debug)]
pub enum AutomationError {
    #[error("Rule not found: {0}")]
    NotFound(Uuid),
    #[error("Rule execution failed: {0}")]
    ExecutionFailed(String),
}

pub struct AutomationService;

impl AutomationService {
    pub async fn list_rules() -> Vec<AutomationRule> {
        Vec::new()
    }

    pub async fn get_summary() -> AutomationSummary {
        AutomationSummary {
            total_rules: 0,
            enabled_rules: 0,
            last_triggered: None,
        }
    }

    pub async fn evaluate_rules(_goal: &Goal) -> Result<Vec<Uuid>, AutomationError> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_automation_empty() {
        let rules = AutomationService::list_rules().await;
        assert!(rules.is_empty());
        let summary = AutomationService::get_summary().await;
        assert_eq!(summary.total_rules, 0);
    }
}
