use reqwest::blocking::Client;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum CreateFileError {
    SendFailed(ReqwestError),
    ReceiveFailed(ReqwestError),
    InvalidAuth,
    ExpiredAuth,
    FileIdTaken,
    FilePathTaken,
    Unspecified,
}

#[derive(Debug)]
pub struct CreateFileRequest {
    pub username: String,
    pub auth: String,    // SignedValue
    pub file_id: String, // UUID
    pub file_name: String,
    pub file_path: String,
    pub file_content: String, // EncryptedValue
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CreateFileResponse {
    pub error_code: String,
    pub current_version: u64,
}

pub fn create_file(
    api_location: String,
    params: &CreateFileRequest,
) -> Result<u64, CreateFileError> {
    let client = Client::new();
    let form_params = [
        ("username", params.username.as_str()),
        ("auth", params.auth.as_str()),
        ("file_id", params.file_id.as_str()),
        ("file_name", params.file_name.as_str()),
        ("file_path", params.file_path.as_str()),
        ("file_content", params.file_content.as_str()),
    ];
    let response = client
        .post(format!("{}/create-file", api_location).as_str())
        .form(&form_params)
        .send()
        .map_err(CreateFileError::SendFailed)?;

    let status = response.status();
    let response_body = response
        .json::<CreateFileResponse>()
        .map_err(CreateFileError::ReceiveFailed)?;

    match (status.as_u16(), response_body.error_code.as_str()) {
        (200..=299, _) => Ok(response_body.current_version),
        (401, "invalid_auth") => Err(CreateFileError::InvalidAuth),
        (401, "expired_auth") => Err(CreateFileError::ExpiredAuth),
        (422, "file_id_taken") => Err(CreateFileError::FileIdTaken),
        (422, "file_path_taken") => Err(CreateFileError::FilePathTaken),
        _ => Err(CreateFileError::Unspecified),
    }
}
