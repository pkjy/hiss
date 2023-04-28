use serde::{Deserialize, Serialize};
#[derive(Deserialize, Clone, Debug)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

const DEFAULT_LIMIT: u64 = 20;
const DEFAULT_OFFSET: u64 = 0;

impl Pagination {
    pub fn limit(&self) -> u64 {
        // 限制limit不允许超过100,超过则按默认值处理
        if self.limit.is_none() || self.limit.unwrap_or_default() > 100 {
            DEFAULT_LIMIT
        } else {
            self.limit.unwrap_or_default()
        }
    }
    pub fn offset(&self) -> u64 {
        // 限制offset不允许超过1000,超过则按默认值处理
        if self.offset.is_none() || self.offset.unwrap_or_default() > 1000 {
            DEFAULT_OFFSET
        } else {
            self.offset.unwrap_or_default()
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: None,
            offset: None,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateCompany {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateShortUrl {
    pub short_domain: String,
    pub original_url: String,
    pub expired_at: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateShortDomain {
    pub domain: String,
}

#[derive(Deserialize, Serialize)]
pub struct ListTemplate<T> {
    pub results: Vec<T>,
    pub count: u64,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortUrlResponse {
    pub id: String,
    pub short_url: String,
    pub original_url: String,
}
