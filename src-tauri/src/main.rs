#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Serialize, Deserialize)]
struct LastProject {
    path: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectMeta {
    name: String,
    author: String,
    version: String,
    created: String,
    uuid: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectData {
    meta: ProjectMeta,
    scenes: serde_json::Value,
}

#[tauri::command]
async fn get_last_project(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let config_path = app_dir.join("last_project.json");
    
    if !config_path.exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let data: LastProject = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    if PathBuf::from(&data.path).exists() {
        Ok(Some(data.path))
    } else {
        Ok(None)
    }
}

#[tauri::command]
async fn create_new_project(name: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    let documents = app_handle.path().document_dir().map_err(|e| e.to_string())?;
    let projects_dir = documents.join("Janusz Projects");
    fs::create_dir_all(&projects_dir).map_err(|e| e.to_string())?;
    
    let project_path = projects_dir.join(format!("{}.janproj", name));
    if project_path.exists() {
        return Err("Projekt o tej nazwie już istnieje".to_string());
    }
    
    fs::create_dir_all(&project_path).map_err(|e| e.to_string())?;
    fs::create_dir_all(project_path.join("assets/backgrounds")).map_err(|e| e.to_string())?;
    
    let resource_dir = app_handle.path().resource_dir().map_err(|e| e.to_string())?;
    let template_bg = resource_dir.join("resources/templates/tutorial_bg.jpg");
    if template_bg.exists() {
        fs::copy(&template_bg, project_path.join("assets/backgrounds/tutorial_bg.jpg"))
         .map_err(|e| e.to_string())?;
    }
    
    let project = ProjectData {
        meta: ProjectMeta {
            name: name.clone(),
            author: "Janusz".to_string(),
            version: "1.0.0".to_string(),
            created: chrono::Utc::now().to_rfc3339(),
            uuid: uuid::Uuid::new_v4().to_string(),
        },
        scenes: serde_json::json!([{
            "Id": "dzien_0",
            "SceneTitle": "TUTORIAL - DZIEŃ ZEROWY",
            "Background": "tutorial_bg",
            "Text": "Witaj w Edytorze Janusza!\n\nTo jest dzień zerowy. Tutaj zaczyna się twoja historia o cebuli, wstydzie i Grażynie.\n\nKliknij 'Dodaj scenę' żeby zacząć.",
            "Choices": []
        }]),
    };
    
    let json = serde_json::to_string_pretty(&project).map_err(|e| e.to_string())?;
    fs::write(project_path.join("project.json"), json).map_err(|e| e.to_string())?;
    
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let last_proj = LastProject { path: project_path.to_string_lossy().to_string() };
    fs::write(
        app_dir.join("last_project.json"), 
        serde_json::to_string(&last_proj).unwrap()
    ).map_err(|e| e.to_string())?;
    
    Ok(project_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn load_project(path: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    let project_json = PathBuf::from(&path).join("project.json");
    let content = fs::read_to_string(&project_json).map_err(|e| e.to_string())?;
    
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let last_proj = LastProject { path: path.clone() };
    fs::write(
        app_dir.join("last_project.json"), 
        serde_json::to_string(&last_proj).unwrap()
    ).map_err(|e| e.to_string())?;
    
    Ok(content)
}

fn main() {
    tauri::Builder::default()
     .plugin(tauri_plugin_dialog::init())
     .plugin(tauri_plugin_fs::init())
     .invoke_handler(tauri::generate_handler![
            get_last_project, 
            create_new_project,
            load_project
        ])
     .run(tauri::generate_context!())
     .expect("error while running tauri application");
}