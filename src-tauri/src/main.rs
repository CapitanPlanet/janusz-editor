#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

#[tauri::command]
fn save_scene_json(content: String, day_name: String) -> Result<String, String> {
    let base_path = std::env::current_dir().map_err(|e| e.to_string())?;
    let data_path = base_path.join("..").join("Assets").join("Data");
    fs::create_dir_all(&data_path).map_err(|e| e.to_string())?;
    let file_path = data_path.join(format!("{}.json", day_name));
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn copy_asset_file(src_path: String, asset_type: String) -> Result<String, String> {
    let base_path = std::env::current_dir().map_err(|e| e.to_string())?;
    let dest_folder = match asset_type.as_str() {
        "Backgrounds" => base_path.join("../Assets/Backgrounds"),
        "Voices" => base_path.join("../Assets/Voices"),
        "Music" => base_path.join("../Assets/Music"),
        _ => base_path.join("../Assets/Misc")
    };
    fs::create_dir_all(&dest_folder).map_err(|e| e.to_string())?;
    let file_name = PathBuf::from(&src_path)
      .file_name()
      .ok_or("Brak nazwy pliku")?
      .to_string_lossy()
      .to_string();
    let dest_path = dest_folder.join(&file_name);
    fs::copy(&src_path, &dest_path).map_err(|e| e.to_string())?;
    Ok(format!("Assets/{}/{}", asset_type, file_name).replace("\\", "/"))
}

fn main() {
    tauri::Builder::default()
      .plugin(tauri_plugin_dialog::init())
      .plugin(tauri_plugin_fs::init())
      .invoke_handler(tauri::generate_handler![
            save_scene_json,
            copy_asset_file
        ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}