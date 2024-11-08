use crate::utils::minors::custom_serde::*;
use serde::{Deserialize, Serialize};

// Request structure for sending a message
#[derive(Deserialize)]
pub struct SendMessageRequest {
  pub user_id: i32,
  pub group_id: i32,
  pub content: String,
  pub message_type: String, // Example values: "text"
}

// Response structure for sending a message
#[derive(Serialize)]
pub struct SendMessageResponse {
  pub message_id: i32,
  pub content: String,
  pub message_type: String,
  pub created_at: String,
}

// for get list message content by click at any joined gr (gr id)
use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Serialize)]
pub struct MessageResponse {
  pub id: i32,
  pub content: Option<String>,
  pub message_type: String,
  #[serde(serialize_with = "serialize_naive_datetime")]
  pub created_at: NaiveDateTime,
  pub user_id: i32,
  pub user_name: String,
}

// Full response structure containing a list of messages
#[derive(Serialize)]
pub struct GetMessagesResponse {
  pub messages: Vec<MessageResponse>,
}

#[derive(Queryable, Serialize, Debug)]
pub struct MessageWithUser {
  pub id: i32,
  pub content: Option<String>,
  pub message_type: String,
  #[serde(serialize_with = "serialize_naive_datetime")]
  pub created_at: NaiveDateTime,
  pub user_id: i32,
  pub user_name: String,
}

#[derive(Serialize)]
pub struct GroupDetailResponse {
  pub group_name: String,
  pub user_id: i32,
  pub max_member: i32,
  pub joined_member: i32,
  pub waiting_member: i32,
  pub created_at: String,
  pub expired_at: String,
  pub messages: Vec<MessageWithUser>,
}
