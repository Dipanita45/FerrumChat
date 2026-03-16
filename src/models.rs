use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatWithParticipant {
    pub id: Uuid,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub participant_id: Uuid,
    pub participant_name: String,
    pub participant_username: String,
}
