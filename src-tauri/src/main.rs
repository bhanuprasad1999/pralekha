// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use std::fs;

// fn read_nested_files(path:String) -> Result<Vec<String>,  String> {
//   match fs:: read_dir(path){
//     Ok(entries) =>{
//       let mut files = Vec::new();
//       for entry in entries{
//         if let Ok(entry) = entry{

//           files.push(entry.path().to_string_lossy().to_string());
//         }
//       }
//     }
//   }
// }


#[command]
fn read_file_details(path: String) -> Result<Vec<String>, String> {
    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.is_file() {
                    files.push(path.to_string_lossy().to_string());
                }
            }
            Ok(files)
        },
        Err(e) => Err(e.to_string()),
    }
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_file_details])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
