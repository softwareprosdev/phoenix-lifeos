use phoenix_core::entities::Transaction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiInsight {
    pub id: Uuid,
    pub insight_type: String,
    pub title: String,
    pub description: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySuggestion {
    pub transaction_id: Uuid,
    pub suggested_category_id: Uuid,
    pub confidence: f64,
}

pub struct AiService;

impl AiService {
    pub async fn categorize_transaction(
        _transaction: &Transaction,
    ) -> Vec<CategorySuggestion> {
        Vec::new()
    }

    pub async fn generate_insights(
        _transactions: &[Transaction],
    ) -> Vec<AiInsight> {
        Vec::new()
    }

    pub async fn chat_query(
        _query: &str,
        _context: &str,
    ) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_empty() {
        let insights = AiService::generate_insights(&[]).await;
        assert!(insights.is_empty());
    }
}
