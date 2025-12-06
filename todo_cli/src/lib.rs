use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

// Rust의 전역 상태 관리는 복잡할 수 있으므로, 
// 간단한 예시를 위해 Mutex로 TodoList를 감싸 전역으로 관리합니다.
static mut TODO_LIST: Option<Mutex<TodoList>> = None;

// --- 구조체 및 로직 정의 (CLI와 유사) ---

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub task: String,
    pub completed: bool,
}

pub struct TodoList {
    items: Vec<TodoItem>,
    next_id: usize,
}

impl TodoList {
    fn new() -> TodoList {
        // Wasm 환경에서는 파일 I/O를 직접 사용하지 않습니다.
        TodoList { items: Vec::new(), next_id: 1 }
    }
    
    // ... add_item, complete_item, remove_item 로직은 거의 동일 ...
    
    fn add_item(&mut self, task: String) {
        let new_item = TodoItem {
            id: self.next_id,
            task,
            completed: false,
        };
        self.items.push(new_item);
        self.next_id += 1;
    }

    fn complete_item(&mut self, id: usize) -> Result<(), &str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            Ok(())
        } else {
            Err("해당 ID의 작업을 찾을 수 없습니다.")
        }
    }

    fn remove_item(&mut self, id: usize) -> Result<(), &str> {
        let initial_len = self.items.len();
        self.items.retain(|item| item.id != id);

        if self.items.len() < initial_len {
            Ok(())
        } else {
            Err("해당 ID의 작업을 찾을 수 없습니다.")
        }
    }
    
    // 목록 전체를 JSON 문자열로 반환하여 JS로 보냅니다.
    fn get_list_json(&self) -> String {
        serde_json::to_string(&self.items).unwrap_or_else(|_| "[]".to_string())
    }
}


// --- Wasm 공개 함수 ---

// wasm 모듈이 로드될 때 한 번만 초기화합니다.
#[wasm_bindgen(start)]
pub fn init() {
    // console.log("Wasm TodoList 초기화됨");
    unsafe {
        if TODO_LIST.is_none() {
            TODO_LIST = Some(Mutex::new(TodoList::new()));
        }
    }
}

// 할 일 항목을 추가하고, 업데이트된 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn add_todo(task: String) -> String {
    let list = unsafe { TODO_LIST.as_ref().unwrap().lock().unwrap() };
    let mut list = list;
    
    list.add_item(task);
    
    list.get_list_json()
}

// ID에 해당하는 항목을 완료 처리하고, 업데이트된 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn complete_todo(id: usize) -> String {
    let list = unsafe { TODO_LIST.as_ref().unwrap().lock().unwrap() };
    let mut list = list;

    // 에러 처리 대신 그냥 완료를 시도하고 목록을 반환합니다.
    let _ = list.complete_item(id);
    
    list.get_list_json()
}

// ID에 해당하는 항목을 삭제하고, 업데이트된 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn remove_todo(id: usize) -> String {
    let list = unsafe { TODO_LIST.as_ref().unwrap().lock().unwrap() };
    let mut list = list;

    let _ = list.remove_item(id);
    
    list.get_list_json()
}

// 현재 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn get_todos() -> String {
    let list = unsafe { TODO_LIST.as_ref().unwrap().lock().unwrap() };
    let list = list;
    
    list.get_list_json()
}