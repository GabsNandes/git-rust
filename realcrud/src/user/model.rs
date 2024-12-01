use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserDate {
    CreationDate(String),
    LastLogin(String),
    Unknown,
}

impl UserDate {
    pub fn to_db_string(&self) -> String {
        match self {
            UserDate::CreationDate(date) | UserDate::LastLogin(date) => date.clone(),
            UserDate::Unknown => "Unknown".to_string(),
        }
    }

    pub fn from_db_string(date: &str) -> Self {
        if date.is_empty() || date == "Unknown" {
            UserDate::Unknown
        } else {
            UserDate::CreationDate(date.to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub date: UserDate,
}
