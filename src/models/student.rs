use crate::models::parent::Parent;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub fname: String,
    pub lname: String,
    pub dob: DateTime,
    pub phone: String,
    pub mobile: String,
    pub parent: Parent,
    pub date_of_join: DateTime,
    pub status: bool,
    pub last_login_date: DateTime,
    pub last_login_ip: String,
}
