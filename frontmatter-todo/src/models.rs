use chrono::DateTime;
use chrono::Utc;
pub struct Task {
    pub efforts: Vec<Effort>,
    pub status: String,
    pub project: String,
    pub tags: Vec<String>,
}

pub struct Effort {
    pub project: String,
    pub amount: u32,
    pub when: DateTime<Utc>,
}