# AGENTS.md

## 프로젝트 개요

이 프로젝트는 Rust WebAssembly 기반의 To-Do List 애플리케이션입니다.
- **언어**: Rust (WebAssembly)
- **타겟**: 웹 브라우저
- **아키텍처**: Rust 라이브러리 + JavaScript 프론트엔드
- **주요 기능**: 할 일 추가, 완료, 삭제, 목록 조회

## 빌드 명령어

### WebAssembly 빌드
```bash
# 개발 빌드 (디버그 정보 포함)
wasm-pack build --dev --target web

# 릴리즈 빌드 (최적화)
wasm-pack build --target web

# 특정 출력 디렉토리 지정
wasm-pack build --target web --out-dir pkg
```

### 코드 검사 및 포맷팅
```bash
# 코드 컴파일 검사
cargo check

# 릴리즈 모드로 컴파일 검사
cargo check --release

# 코드 포맷팅
cargo fmt

# 코드 포맷팅 검사만
cargo fmt --check

# 린트 실행
cargo clippy

# 린트를 경고로 에러 처리
cargo clippy -- -D warnings

# 모든 린트 규칙 적용
cargo clippy -- -W clippy::all
```

### 로컬 테스트 서버
```bash
# Python 3
python3 -m http.server 8000

# Node.js (설치 필요)
npx serve .

# 빌드 후 테스트
wasm-pack build --target web && python3 -m http.server 8000
```

## 테스트 명령어

### Rust 테스트
```bash
# 전체 테스트 실행
cargo test

# 라이브러리 테스트만 실행
cargo test --lib

# 단일 테스트 실행
cargo test test_name

# 특정 테스트 패턴
cargo test todo

# 상세 출력으로 테스트
cargo test -- --nocapture

# 테스트 컴파일만 (실행 안 함)
cargo test --no-run
```

### WebAssembly 테스트
```bash
# headless 모드로 테스트
wasm-pack test --headless

# Firefox로 테스트
wasm-pack test --headless --firefox

# Chrome로 테스트
wasm-pack test --headless --chrome

# Node.js로 테스트
wasm-pack test --node
```

## 코드 스타일 가이드라인

### 임포트 구성
```rust
// 1. 표준 라이브러리
use std::sync::Mutex;
use std::collections::HashMap;

// 2. 외부 크레이트
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// 3. 로컬 모듈
use crate::todo::TodoItem;
use crate::utils::helper;
```

### 명명 규칙
- **타입/구조체**: `PascalCase` (예: `TodoItem`, `TodoList`)
- **함수/메서드**: `snake_case` (예: `add_todo`, `get_list`)
- **변수**: `snake_case` (예: `task_name`, `item_id`)
- **상수**: `SCREAMING_SNAKE_CASE` (예: `MAX_ITEMS`, `DEFAULT_TIMEOUT`)
- **WebAssembly 내보내기**: `snake_case` (JavaScript와 호환)

### 타입 및 에러 처리
```rust
// Result 타입 사용
fn add_item(&mut self, task: String) -> Result<usize, &str> {
    if task.is_empty() {
        return Err("작업 내용이 비어있습니다.");
    }
    // ...
    Ok(new_id)
}

// WebAssembly 내보내기 함수에서는 간단한 에러 처리
#[wasm_bindgen]
pub fn add_todo(task: String) -> String {
    // 에러 발생 시 빈 목록 반환
    let list = unsafe { TODO_LIST.as_ref().unwrap().lock().unwrap() };
    let mut list = list;
    list.add_item(task);
    list.get_list_json()
}
```

### WebAssembly 특이사항
```rust
// 내보내기 함수는 반드시 #[wasm_bindgen] 사용
#[wasm_bindgen]
pub fn get_todos() -> String {
    // JSON으로 데이터 직렬화
