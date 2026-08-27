use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub reason: String,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PostJob {
    pub source_code: String,
    pub language: String,
    pub user_id: i32,
    pub contest_id: i32,
    pub problem_id: i32,
}

#[derive(Debug, Deserialize, Default)]
pub struct JobFilter {
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub contest_id: Option<i32>,
    pub problem_id: Option<i32>,
    pub language: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub state: Option<String>,
    pub result: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetJob {
    pub id: i32,
    pub created_time: String,
    pub updated_time: String,
    pub submission: PostJob,
    pub state: String,
    pub result: String,
    pub score: f32,
    pub cases: Vec<TestCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contest {
    pub id: i32,
    pub name: String,
    pub from: String,
    pub to: String,
    pub problem_ids: Vec<i32>,
    pub user_ids: Vec<i32>,
    pub submission_limit: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OJConfig {
    pub server: ServerConfig,
    pub problems: Vec<Problem>,
    pub languages: Vec<Language>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    pub bind_address: String,
    pub bind_port: u16,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Problem {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub problem_type: String,
    pub misc: Value,
    pub cases: Vec<TestCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub score: f64,
    pub input_file: String,
    pub answer_file: String,
    pub time_limit: u128,
    pub memory_limit: i32
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Language {
    pub name: String,
    pub file_name: String,
    pub command: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestJob {
    pub id: i32,
    pub created_time: String,
    pub updated_time: String,
    pub submission: PostJob,
    pub state: String,
    pub result: String,
    pub score: f64,
    pub cases: Vec<TestJobCase>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestJobCase {
    pub id: i32,
    pub result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JudgeResult {
    pub score: f64,
    pub result: String,
    pub cases: Vec<TestJobCase>
}
