use chrono::{DateTime, Utc};
use phoenix_core::event_bus::EventBus;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    Idle,
    Syncing,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub last_sync_at: Option<DateTime<Utc>>,
    pub status: SyncStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncProfile {
    pub id: Uuid,
    pub name: String,
    pub provider: String,
    pub config_json: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Sync provider not configured: {0}")]
    NotConfigured(String),
    #[error("Sync conflict: {0}")]
    Conflict(String),
    #[error("Network error: {0}")]
    Network(String),
}

pub struct SyncService {
    #[allow(dead_code)]
    event_bus: Option<EventBus>,
}

impl SyncService {
    pub fn new() -> Self {
        Self { event_bus: None }
    }

    pub fn with_event_bus(event_bus: EventBus) -> Self {
        Self {
            event_bus: Some(event_bus),
        }
    }

    pub fn state(&self) -> SyncState {
        SyncState {
            last_sync_at: None,
            status: SyncStatus::Idle,
            error_message: None,
        }
    }

    pub async fn trigger_sync(&self, _profile_id: Option<Uuid>) -> Result<SyncState, SyncError> {
        Ok(SyncState {
            last_sync_at: Some(Utc::now()),
            status: SyncStatus::Idle,
            error_message: None,
        })
    }
}

impl Default for SyncService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_state() {
        let svc = SyncService::new();
        let state = svc.state();
        assert_eq!(state.status, SyncStatus::Idle);
        assert!(state.last_sync_at.is_none());
    }

    #[tokio::test]
    async fn test_trigger_sync() {
        let svc = SyncService::new();
        let result = svc.trigger_sync(None).await.unwrap();
        assert!(result.last_sync_at.is_some());
        assert_eq!(result.status, SyncStatus::Idle);
    }
}
