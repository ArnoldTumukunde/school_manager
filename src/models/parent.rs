use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub fname: String,
    pub lname: String,
    pub dob: DateTime,
    pub phone: String,
    pub mobile: String,
    pub status: bool,
    pub last_login_date: DateTime,
    pub last_login_ip: String,
}
