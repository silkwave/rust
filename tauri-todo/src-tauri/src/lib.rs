use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

// State for our application
pub struct AppState(Mutex<Vec<Todo>>);

// Tauri Command to get all todos
#[tauri::command]
fn get_todos(state: State<'_, AppState>) -> Vec<Todo> {
    state.0.lock().unwrap().clone()
}

// Tauri Command to add a new todo
#[tauri::command]
fn add_todo(state: State<'_, AppState>, description: String) -> Todo {
    let mut todos = state.0.lock().unwrap();
    let id = todos.len() as u32 + 1; // Simple ID generation
    let new_todo = Todo {
        id,
        description,
        completed: false,
    };
    todos.push(new_todo.clone());
    new_todo
}

// Tauri Command to toggle a todo's completion status
#[tauri::command]
fn toggle_todo(state: State<'_, AppState>, id: u32) -> Option<Todo> {
    let mut todos = state.0.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
        Some(todo.clone())
    } else {
        None
    }
}

// Tauri Command to delete a todo
#[tauri::command]
fn delete_todo(state: State<'_, AppState>, id: u32) -> bool {
    let mut todos = state.0.lock().unwrap();
    let initial_len = todos.len();
    todos.retain(|t| t.id != id);
    todos.len() < initial_len
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(Vec::new()))) // Initialize our state
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            toggle_todo,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
