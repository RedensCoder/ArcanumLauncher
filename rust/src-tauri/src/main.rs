use std::fs::File;
use std::fs;
use std::io::{self, Write};
use reqwest::get;
use std::process::{Command, exit};
use std::env;
use std::path::{Path, PathBuf};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn download(name: String, data: String) {
    let mut file_data = File::create("../public/token.txt").unwrap();

    file_data.write_all(data.as_bytes()).unwrap();

    let url = format!("http://127.0.0.1:8080/api/v1/game/{}", name);
    let file_path = format!("../public/{}.exe", name);

    let mut file = File::create(file_path).unwrap();

    let response = get(url).await.unwrap();

    let bytes = response.bytes().await.unwrap();
    file.write_all(&bytes).unwrap();
}

#[tauri::command]
async fn check_download(game: String) -> Vec<String> {
    let entries = fs::read_dir("../public").unwrap();

    let mut files: Vec<String> = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    files.push(file_name_str.to_string());
                }
            }
        }
    }

    return files;
}

#[tauri::command]
async fn play(game: String) {
    let relative_path = format!("../public/{}.exe", game);

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            exit(1);
        }
    };

    let absolute_path = current_dir.join(Path::new(&relative_path));

    let executable_path = match absolute_path.canonicalize() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Failed to get canonical path for the executable: {}", err);
            exit(1);
        }
    };

    Command::new(&executable_path).output().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download, check_download, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
