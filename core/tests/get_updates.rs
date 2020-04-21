extern crate lockbook_core;

use crate::utils::generate_account;

use lockbook_core::client;
use lockbook_core::client::CreateFileRequest;
use lockbook_core::client::NewAccountRequest;
use lockbook_core::client::{FileMetadata, GetUpdatesRequest};

#[macro_use]
pub mod utils;
use utils::{api_loc, generate_file_id, generate_username, TestError};
use lockbook_core::service::auth_service::{AuthServiceImpl, AuthService};
use lockbook_core::service::clock_service::ClockImpl;
use lockbook_core::service::crypto_service::RsaImpl;

fn get_updates(username: String, file_id: String) -> Result<(Vec<FileMetadata>, u64), TestError> {
    let account = generate_account();

    client::new_account(
        api_loc(),
        &NewAccountRequest {
            username: account.username.clone(),
            auth: AuthServiceImpl::<ClockImpl, RsaImpl>::generate_auth( &account.keys, &account.username.clone()).unwrap(),
            public_key: serde_json::to_string(&account.keys.to_public_key()).unwrap(),
        },
    )?;

    let file_version = client::create_file(
        api_loc(),
        &CreateFileRequest {
            username: account.username.clone(),
            auth: AuthServiceImpl::<ClockImpl, RsaImpl>::generate_auth( &account.keys, &account.username.clone()).unwrap(),
            file_id: file_id.to_string(),
            file_name: "file_name".to_string(),
            file_path: "file_path".to_string(),
            file_content: "file_content".to_string(),
        },
    )?;

    let updates_metadata = client::get_updates(
        api_loc(),
        &GetUpdatesRequest {
            username: account.username.clone(),
            auth: AuthServiceImpl::<ClockImpl, RsaImpl>::generate_auth( &account.keys, &account.username.clone()).unwrap(),
            since_version: 0,
        },
    )?;

    Ok((updates_metadata, file_version))
}

#[test]
fn test_get_updates() {
    let account = generate_account();
    let file_id = generate_file_id();

    let updates_metadata_and_file_version = get_updates(account.username.clone(), file_id.to_string());
    assert_matches!(&updates_metadata_and_file_version, &Ok(_));
    let (updates_metadata, file_version) = updates_metadata_and_file_version.unwrap();
    assert_eq!(
        updates_metadata[..],
        [FileMetadata {
            file_id: file_id.to_string(),
            file_name: "file_name".to_string(),
            file_path: "file_path".to_string(),
            file_content_version: file_version,
            file_metadata_version: file_version,
            deleted: false,
        }][..]
    );
}
