use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum CabinetStatus {
    Vacant,
    Hold,
    Occupied,
}

impl std::fmt::Display for CabinetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CabinetStatus::Vacant => write!(f, "vacant"),
            CabinetStatus::Hold => write!(f, "hold"),
            CabinetStatus::Occupied => write!(f, "occupied"),
        }
    }
}

impl TryFrom<i32> for CabinetStatus {
    type Error = crate::error::DomainError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CabinetStatus::Vacant),
            2 => Ok(CabinetStatus::Hold),
            3 => Ok(CabinetStatus::Occupied),
            _ => Err(crate::error::cabinet::CabinetError::StatusNotSupport(value))?,
        }
    }
}

impl CabinetStatus {
    pub fn code(&self) -> i32 {
        match self {
            CabinetStatus::Vacant => 1,
            CabinetStatus::Hold => 2,
            CabinetStatus::Occupied => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Cabinet {
    pub code: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub password: Option<String>,
    pub status: CabinetStatus,
    pub hold_token: Option<String>,
    pub expire_at: Option<DateTime<Local>>,
    pub create_at: Option<DateTime<Local>>,
    pub update_at: Option<DateTime<Local>>,
    pub version: Option<i32>,
}

impl Cabinet {
    pub fn new(
        code: i64,
        name: Option<String>,
        description: Option<String>,
        status: CabinetStatus,
        hold_token: Option<String>,
        expire_at: Option<DateTime<Local>>,
    ) -> Self {
        Cabinet {
            code,
            name,
            description,
            password: None,
            status,
            hold_token,
            expire_at,
            create_at: None,
            update_at: None,
            version: None,
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
            _ => Err(crate::error::cabinet::CabinetError::InvalidItemCategory(
                s.to_string(),
            ))?,
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

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CabinetUsage {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

impl CabinetUsage {
    pub fn new(total: u64, used: u64) -> Self {
        CabinetUsage {
            total,
            used,
            free: total - used,
        }
    }
}
