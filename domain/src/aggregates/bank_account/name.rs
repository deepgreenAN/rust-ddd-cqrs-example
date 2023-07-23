use crate::error::DomainError;

use serde::{Deserialize, Serialize};

/// 口座名を表す型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountName {
    // 名
    first_name: String,
    // 姓
    last_name: String,
}

impl AccountName {
    pub fn from_primitives(first_name: String, last_name: String) -> Result<Self, DomainError> {
        Ok(Self {
            first_name,
            last_name,
        })
    }
    pub fn first_name(&self) -> &str {
        &self.first_name
    }
    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}
