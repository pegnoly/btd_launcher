extern crate google_drive3 as drive3;
use std::{default::Default, path::PathBuf, sync::{Arc, Mutex}};
use chrono::{DateTime, Utc};
use drive3::{DriveHub, oauth2, hyper, hyper_rustls::{HttpsConnector, HttpsConnectorBuilder}, chrono, FieldMask};
use oauth2::{hyper::client::HttpConnector, service_account_impersonator, ServiceAccountKey};
use tauri::{State, Manager};
use yup_oauth2::ServiceAccountAuthenticator;
use reqwest::Client;

use std::fs::File;
use std::io::Cursor;

/// Google drive connection module.

const BTD_FILES_FOLDER_ID: &str = "1V4r2zyRutMSEpzD7envq5nAXDhIZaLtH";

pub struct DriveManager {
    pub hub: Arc<DriveHub<HttpsConnector<HttpConnector>>>
}

impl DriveManager {
    /// Here i'm using service account auth cause its only way that work permanently(secret key requires re-auth sometimes)
    /// Don't really know this is a best way but i found it more preferable in this situation.
    pub async fn build(config_path: &PathBuf) -> Option<Self> {
        let s1 = yup_oauth2::read_service_account_key(config_path.join("auth\\btd-main-25fe7725174f.json"))
            .await;
        match s1 {
            Ok(key) => {
                let auth_result = yup_oauth2::ServiceAccountAuthenticator::builder(
                    key
                ).build().await;
                match auth_result {
                    Ok(auth) => {
                        let hub = DriveHub::new(
                            hyper::Client::builder().build(
                                HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()
                            ),
                            auth
                        );
                        println!("auth ok");
                        Some(DriveManager {
                            hub: Arc::new(hub)
                        })
                    },
                    Err(ee) => {
                        println!("Error with service account auth, {}", ee.to_string());
                        None
                    }
                }
            },
            Err(e) => {
                println!("Error with service account key, {}", e.to_string());
                None
            }
        }

        // let s = yup_oauth2::read_application_secret(
        //     config_path.
        //     join("auth\\client_secret_59722361664-ptmtbg21sef4fvpffk7djnt5ogd4rf2s.apps.googleusercontent.com.json")).
        //     await.unwrap(); 
        // let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        //     s, 
        //     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect
        // )
        //     .persist_tokens_to_disk(config_path.join("auth\\tokencashe.json"))
        //     .build()
        //     .await.unwrap();
        // let hub = DriveHub::new(
        //     hyper::Client::builder().build(
        //         HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()
        //     ),
        //     auth
        // );
        // Some(DriveManager {
        //     hub: Arc::new(tokio::sync::Mutex::new(hub))
        // })
    }

    pub async fn test(&mut self) {
        let res = self.hub.
            files().
            list().add_scope(drive3::api::Scope::MetadataReadonly).param("fields", "files(id, name, mimeType, parents, modifiedTime)").
            q(format!("'{}' in parents and mimeType = 'application/x-zip'", BTD_FILES_FOLDER_ID).as_str()).
            // supports_all_drives(true).
            // include_items_from_all_drives(true).
            doit().await;
        match res {
            Err(e) => match e {
                // The Error enum provides details about what exactly happened.
                // You can also just use its `Debug`, `Display` or `Error` traits
                drive3::Error::HttpError(_)
                | drive3::Error::Io(_)
                | drive3::Error::MissingAPIKey
                | drive3::Error::MissingToken(_)
                | drive3::Error::Cancelled
                | drive3::Error::UploadSizeLimitExceeded(_, _)
                | drive3::Error::Failure(_)
                | drive3::Error::BadRequest(_)
                | drive3::Error::FieldClash(_)
                | drive3::Error::JsonDecodeError(_, _) => println!("{}", e),
            },
            Ok(res) => {
                let files = res.1.files.unwrap();
                for file in files {
                    let name = &file.name.unwrap();
                    let modified: &DateTime<Utc> = &file.modified_time.unwrap().into();
                    println!("file: {:?}", name);
                    println!("modified: {:?}", modified);
                }
            },
        }
    }  
}