use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
    pub has_next: bool,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: u64, page: u32, per_page: u32) -> Self {
        let has_next = (page as u64 * per_page as u64) < total;
        Self { items, total, page, per_page, has_next }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: u32,
    pub per_page: u32,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self { page: 1, per_page: 50 }
    }
}
