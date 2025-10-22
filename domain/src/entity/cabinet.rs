use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Cabinet {
    pub code: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub password: Option<String>,
    pub used: bool,
    pub pending_destruction: bool,
}

impl Cabinet {
    pub fn new(code: i64, name: Option<String>, description: Option<String>) -> Self {
        Cabinet {
            code,
            name,
            description,
            password: None,
            used: false,
            pending_destruction: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CabinetItemCategory {
    Text,
    File,
}

impl FromStr for CabinetItemCategory {
    type Err = crate::error::DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(CabinetItemCategory::Text),
            "file" => Ok(CabinetItemCategory::File),
            _ => Err(crate::error::DomainError::InvalidCabinetItemCategory(
                s.to_string(),
            )),
        }
    }
}

impl std::fmt::Display for CabinetItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CabinetItemCategory::File => write!(f, "file"),
            CabinetItemCategory::Text => write!(f, "text"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CabinetItem {
    pub id: i64,
    pub cabinet_code: i64,
    pub category: CabinetItemCategory,
    pub name: String,
    pub content: Option<Vec<u8>>,
    pub sort_order: i32,
}

impl CabinetItem {
    pub fn new(
        id: i64,
        cabinet_code: i64,
        category: CabinetItemCategory,
        name: String,
        content: Option<Vec<u8>>,
        sort_order: i32,
    ) -> Self {
        CabinetItem {
            id,
            cabinet_code,
            category,
            name,
            content,
            sort_order,
        }
    }
}
