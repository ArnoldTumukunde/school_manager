use super::teacher::Teacher;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Classroom {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub year: i32,
    pub grade_id: i64,
    pub section: String,
    pub status: bool,
    pub remarks: String,
    pub teacher: Teacher,
}
