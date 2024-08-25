use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Task {
    #[serde(default)]
    pub efforts: Vec<Effort>,
    pub status: String,
    #[serde(default)]
    pub project: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
pub struct Effort {
    pub project: String,
    pub amount: f32,
    #[serde(with = "serde_with::chrono::NaiveDate")]
    pub when: NaiveDate,
}