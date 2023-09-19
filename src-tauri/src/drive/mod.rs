extern crate google_drive3 as drive3;
use std::{default::Default, path::PathBuf, sync::{Arc, Mutex}};
use chrono::{DateTime, Utc};
use drive3::{DriveHub, oauth2, hyper, hyper_rustls::{HttpsConnector, HttpsConnectorBuilder}, chrono, FieldMask};
use oauth2::{hyper::client::HttpConnector, service_account_impersonator};
use tauri::{State, Manager};
use yup_oauth2::ServiceAccountAuthenticator;
use reqwest::Client;

use std::fs::File;
use std::io::Cursor;

const BTD_FILES_FOLDER_ID: &str = "1V4r2zyRutMSEpzD7envq5nAXDhIZaLtH";

pub struct DriveManager {
    pub hub: Arc<tokio::sync::Mutex<DriveHub<HttpsConnector<HttpConnector>>>>
}

impl DriveManager {
    pub async fn build(config_path: &PathBuf) -> Self {
        let s = yup_oauth2::read_application_secret(
            config_path.
            join("auth\\client_secret_59722361664-ptmtbg21sef4fvpffk7djnt5ogd4rf2s.apps.googleusercontent.com.json")).
            await.unwrap(); 
        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            s, 
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect
        )
            .persist_tokens_to_disk(config_path.join("auth\\tokencashe.json"))
            .build()
            .await.unwrap();
        let hub = DriveHub::new(
            hyper::Client::builder().build(
                HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()
            ),
            auth
        );
        DriveManager {
            hub: Arc::new(tokio::sync::Mutex::new(hub))
        }
    }

    pub async fn test(&mut self) {
        let res = self.hub.lock().await.
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

#[derive(Clone, serde::Serialize)]
struct Payload {
  download_percent: f32,
}

// #[tauri::command]
// pub async fn check_for_update(app: tauri::AppHandle, drive_manager: State<'_, DriveManager>, file_manager: State<'_, FileManager>, file_type: FileType) -> Result<(), ()> {
//     let mut file_manager_locked = file_manager.files_info.lock().await;
//     let current_files_info = file_manager_locked.get_mut(&file_type).unwrap();
//     let res = 
//         drive_manager.hub.lock().await
//         .files()
//         .list()
//         .add_scope(drive3::api::Scope::MetadataReadonly)
//         .param("fields", "files(id, name, mimeType, parents, modifiedTime)")
//         .q(format!("'{}' in parents and mimeType = 'application/x-zip'", BTD_FILES_FOLDER_ID).as_str())
//         .doit().await;
//     match res {
//         Err(e) => println!("error when checking for update"),
//         Ok(response) => {
//             println!("Checking for updated files");
//             let stored_files = response.1.files.unwrap();
//             // select all files those are not exists on client side or have not actual modified time
//             let updated_files: Vec<_> = stored_files.iter()
//                 .filter(|f| {
//                     current_files_info.is_actual_file(f.name.as_ref().unwrap()) == false
//                 }).collect();
//             println!("updated files: {:?}", updated_files);
//             // download new or updated files
//             for updated_file in updated_files {
//                 let target = format!("https://drive.google.com/uc?/export=download&id={}", updated_file.id.as_ref().unwrap());
//                 let responce = reqwest::get(target).await;
//                 match responce {
//                     Ok(res) => {
//                         let x = res.bytes().await.unwrap();
//                         let len = x.len() as f32;
//                         let mut downloaded = 0f32;
//                         let mut new_file = File::create(current_files_info.game_path.as_ref().unwrap().join(updated_file.name.as_ref().unwrap())).unwrap();
//                         for chunk in x.chunks(100000) {
//                             let mut content = Cursor::new(chunk);
//                             std::io::copy(&mut content, &mut new_file); 
//                             downloaded += 10000f32;
//                             app.app_handle().emit_to("main", "test", Payload{download_percent: (downloaded / len) as f32}).unwrap();
//                         }
//                         // let mut content = Cursor::new(res.bytes().await.unwrap());
//                         // std::io::copy(&mut content, &mut new_file);
//                     },
//                     Err(err) => {
//                         println!("error {:?}", err);
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }