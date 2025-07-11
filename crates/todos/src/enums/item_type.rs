use std::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumString;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, EnumString)]
#[strum(serialize_all = "camelCase")] // 自动处理连字符格式
pub enum ItemType {
    #[default]
    TASK,
    NOTE,
}
impl ItemType {
    pub fn parse(value: Option<&str>) -> ItemType {
        match value {
            Some("task") => ItemType::TASK,
            Some("note") => ItemType::NOTE,
            _ => ItemType::TASK,
        }
    }
    pub fn to_lowercase(&self) -> String {
        match self {
            ItemType::TASK => "task".to_string(),
            ItemType::NOTE => "note".to_string(),
        }
    }
}
impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_lowercase())
    }
}
