use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

// thread_local을 사용하여 더 안전한 전역 상태 관리
thread_local! {
    static TODO_LIST: RefCell<TodoList> = RefCell::new(TodoList::new());
}

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
        TodoList {
            items: Vec::new(),
            next_id: 1,
        }
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
    // thread_local로 자동 초기화되므로 별도 초기화 불필요
}

// 할 일 항목을 추가하고, 업데이트된 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn add_todo(task: String) -> String {
    TODO_LIST.with(|list| {
        let mut list = list.borrow_mut();
        list.add_item(task);
        list.get_list_json()
    })
}

// ID에 해당하는 항목을 완료 처리하고, 업데이트된 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn complete_todo(id: usize) -> String {
    TODO_LIST.with(|list| {
        let mut list = list.borrow_mut();
        // 에러 처리 대신 그냥 완료를 시도하고 목록을 반환합니다.
        let _ = list.complete_item(id);
        list.get_list_json()
    })
}

// ID에 해당하는 항목을 삭제하고, 업데이트된 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn remove_todo(id: usize) -> String {
    TODO_LIST.with(|list| {
        let mut list = list.borrow_mut();
        let _ = list.remove_item(id);
        list.get_list_json()
    })
}

// 현재 목록을 JSON 문자열로 반환합니다.
#[wasm_bindgen]
pub fn get_todos() -> String {
    TODO_LIST.with(|list| {
        let list = list.borrow();
        list.get_list_json()
    })
}
