use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub public_key: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct File {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub filename: String,
    pub filetype: String,
    pub filesize: i64,
    pub encrypted_aes_key: Vec<u8>,
    pub encrypted_file: Vec<u8>,
    pub iv: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct ShareLink {
    pub id: uuid::Uuid,
    pub file_id: uuid::Uuid,
    pub recipient_user_id: Option<uuid::Uuid>,
    pub password: Option<String>,
    pub link: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)] 
pub struct SendFileDetails {
    pub file_id: uuid::Uuid,
    pub filename: String,
    pub recipient_email: String,
    pub expiration_date: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}
#[derive(sqlx::FromRow)] 
pub struct ReceiveFileDetails {
    pub file_id: uuid::Uuid,
    pub filename: String,
    pub sender_email: String,
    pub expiration_date: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}