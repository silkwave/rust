# 프로젝트 개요

이 프로젝트는 Rust 기반의 Tauri 프레임워크를 사용하여 데스크톱 애플리케이션을 개발합니다. 백엔드는 Rust로, 프론트엔드는 TypeScript와 Vite를 사용한 웹 기술로 구현되었습니다. 주요 기능은 간단한 To-Do 목록 관리입니다.

## 기술 스택

### 백엔드 (Rust)
- **언어**: Rust (2021 Edition)
- **프레임워크**: Tauri v2
- **의존성**:
    - `tauri`: Tauri 프레임워크 코어
    - `tauri-plugin-opener`: 외부 링크 열기 플러그인
    - `serde`, `serde_json`: JSON 직렬화/역직렬화
    - `tauri-build`: Tauri 빌드 도구 (빌드 의존성)
- **구조**: `src-tauri/src/main.rs`는 애플리케이션의 진입점이며, `tauri_app_lib::run()`을 호출합니다. 핵심 로직은 `src-tauri/src/lib.rs`에 정의될 것으로 예상됩니다.

### 프론트엔드 (TypeScript + Web)
- **언어**: TypeScript
- **빌드 도구**: Vite v6
- **프레임워크/라이브러리**:
    - `@tauri-apps/api`: Tauri 백엔드와 통신하기 위한 API
    - `@tauri-apps/plugin-opener`: Tauri 플러그인
- **구조**: `src/main.ts`가 프론트엔드의 진입점입니다. HTML 요소를 조작하고 `@tauri-apps/api`의 `invoke` 함수를 사용하여 Rust 백엔드의 명령을 호출합니다. To-Do 항목을 생성, 읽기, 업데이트, 삭제하는 기능을 구현합니다.

## 애플리케이션 기능

- To-Do 항목 추가
- To-Do 항목 완료 상태 토글
- To-Do 항목 삭제
- 애플리케이션 시작 시 To-Do 목록 로드

## 개발 환경 설정

- `package.json`에 정의된 스크립트를 사용하여 개발 및 빌드 프로세스를 관리합니다.
    - `npm run dev`: Vite 개발 서버 시작
    - `npm run build`: TypeScript 컴파일 및 Vite를 사용한 프론트엔드 빌드
    - `npm run tauri`: Tauri CLI 명령 실행 (예: `tauri dev`, `tauri build`)

이 프로젝트는 Tauri의 최신 버전(v2)을 사용하여 Rust와 웹 기술의 통합을 보여주는 좋은 예시입니다.
