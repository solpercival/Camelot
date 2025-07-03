use core::str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{
    Validate,
    ValidationError,
};

use crate::models::{ReceiveFileDetails, SendFileDetails, User};

// DTO for sending file details
// This struct is used to encapsulate the details of a file being sent
#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate)]
pub struct RegisteredUserDto {
    #[validate(length(min = 1, message = "Username must not be empty"))]
    pub username: String,

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Invalid email format")
    )]
    pub email: String,

    #[validate(
        length(min = 8, 
        message = "Password must be at least 8 characters long"))
    ]
    pub password: String,

    #[validate(
        length (min = 1, message = "Confirm Password must not be empty"),
        must_match(
            other = "password",
            message = "Confirm Password must match Password"
        )
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

// Custom validation function to check if two strings match
#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
    pub struct LoginUserDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Invalid email format"))]
    pub email: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 8, message = "Password must be at least 8 characters long")
    )]
    pub password: String,
}

// DTO for sending file details
// This struct is used to encapsulate the details of a file being sent
#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<u32>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<u32>,
}

// DTO for sending file details
// This struct is used to encapsulate the details of a file being sent
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub public_key: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Implementation of methods for FilterUserDto
// This struct is used to filter user data for responses
impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            public_key: user.public_key.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// DTO for user registration
// This struct is used to encapsulate the user data for registration
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

// DTO for user responsible actions
// This struct is used to encapsulate the user data and status for responses
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponsibleDto {
    pub status: String,
    pub data: UserData,
}

// DTO for receiving file details
// This struct is used to encapsulate the details of a file being received
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileDto {
    pub file_id: String,
    pub file_name: String,
    pub recipient_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

// Implementation of methods for UserSendFileDto
// This struct is used to filter send file details for responses
impl UserSendFileDto {
    pub fn filter_send_user_file(file_data: &SendFileDetails) -> Self {
        Self {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.filename.to_owned(),
            recipient_email: file_data.recipient_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap_or_else(|| Utc::now()),
            created_at: file_data.created_at.unwrap_or_else(|| Utc::now()),
        }
    }

    pub fn filter_send_user_files(user: &[SendFileDetails]) -> Vec<Self> {
        user.iter()
            .map(Self::filter_send_user_file)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileListResponseDto {
    pub status: String,
    pub files: Vec<UserSendFileDto>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileDto {
    pub file_id: String,
    pub file_name: String,
    pub sender_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

// Implementation of methods for UserReceiveFileDto
// This struct is used to filter receive file details for responses
impl UserReceiveFileDto {
    pub fn filter_receive_user_file(file_data: &ReceiveFileDetails) -> Self {
        Self {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.filename.to_owned(),
            sender_email: file_data.sender_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap_or_else(|| Utc::now()),
            created_at: file_data.created_at.unwrap_or_else(|| Utc::now()),
        }
    }

    pub fn filter_receive_user_files(user: &[ReceiveFileDetails]) -> Vec<Self> {
        user.iter()
            .map(Self::filter_receive_user_file)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileListResponseDto {
    pub status: String,
    pub files: Vec<UserReceiveFileDto>,
    pub results: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String
}

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "New password is required"),
        length(min = 8, message = "New password must be at least 8 characters long"))]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "New confirm password is required"),
        length(min = 8, message = "New confirm password must be at least 8 characters long"),)]
    pub new_password_confirm: String,

    #[validate(
        length(min = 1, message = "Old password is required"),
        length(min = 8, message = "Old password must be at least 8 characters long"))]
    pub old_password: String
}

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct SearchQueryByEmailDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct FilterEmailDto {
    pub email: String
}

impl FilterEmailDto {
    pub fn filter_email(user: &User) -> Self {
        Self {
            email: user.email.to_owned(),
        }
    }

    pub fn filter_emails(user: &[User]) -> Vec<Self> {
        user.iter()
            .map(Self::filter_email)
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailListResponseDto {
    pub status: String,
    pub emails: Vec<FilterEmailDto>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Validate)]
pub struct FileUploadDtos {
    #[validate(email(message = "Invalid email format"))]
    pub recipient_email: String,

    #[validate(
        length(min = 1, message = "New password is required"),
        length(min = 8, message = "New password must be at least 8 characters long")
    )]
    pub password: String,

    #[validate(custom = "validate_expiration_date")]
    pub expiration_date: String
}

fn validate_expiration_date(expiration_date: &str) -> Result<(), ValidationError> {
    if expiration_date.is_empty() {
        let mut error = ValidationError::new("expiration_date_required");
        error.message = Some("Expiration date is required.".into());
        return Err(error);
    }

    let parsed_date = DateTime::parse_from_rfc3339(expiration_date)
    .map_err(|_| {
        let mut error = ValidationError::new("invalid_date_format");
        error.message = Some("Invalid date format. Expected format is YYYY-MM-DDTHH:MM:SS.ssssssZ.".into());
        error
    })?;

    let now = Utc::now();

    if parsed_date <= now {
        let mut error = ValidationError::new("expiration_date_future");
        error.message = Some("Expiration date must be in the future.".into());
        return Err(error);
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Validate, Clone, Default)]
pub struct RetrieverFileDto {
    #[validate(length(min = 1, message = "Shared id is required"))]
    pub shared_id: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 8, message = "Password must be at least 8 characters long")
    )]
    pub password: String
}