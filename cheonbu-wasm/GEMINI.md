# Gemini 가이드

이 문서는 Gemini와 함께 이 프로젝트를 개발하기 위한 가이드입니다.

## 프로젝트 개요

이 프로젝트는 Rust로 작성되었으며 WebAssembly로 컴파일됩니다. `wasm-bindgen`을 사용하여 Rust와 JavaScript 간의 상호 작용을 처리합니다.

## 개발 환경 설정

### 1. Rust 설치

Rust가 설치되어 있지 않다면 [rustup](https://rustup.rs/)을 사용하여 설치하세요.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. `wasm-pack` 설치

`wasm-pack`은 Rust 코드를 WebAssembly로 빌드하고 JavaScript와 상호 운용할 수 있도록 패키징하는 데 사용됩니다.

```bash
cargo install wasm-pack
```

## 빌드 및 실행

### 1. WebAssembly 모듈 빌드

다음 명령어를 사용하여 Rust 코드를 WebAssembly로 컴파일합니다.

```bash
wasm-pack build --target web
```

이 명령어는 `pkg` 디렉토리에 WebAssembly 모듈과 관련 JavaScript 파일을 생성합니다.

### 2. 웹 서버 실행

간단한 웹 서버를 사용하여 `index.html` 파일을 제공해야 합니다. Python이 설치되어 있다면 다음 명령어를 사용할 수 있습니다.

```bash
python3 -m http.server
```

또는 `npm`을 사용하는 경우 `http-server`를 설치하여 사용할 수 있습니다.

```bash
npm install -g http-server
http-server .
```

서버를 실행한 후 웹 브라우저에서 `http://localhost:8000` (또는 서버가 사용하는 포트)으로 이동하여 애플리케이션을 확인하세요.

## 테스트

프로젝트에 테스트가 추가되면 여기에 테스트 실행 방법을 추가할 예정입니다.

## Gemini와 상호 작용

- **코드 변경 요청:** 특정 파일의 코드를 변경하려면 파일 경로와 함께 원하는 변경 사항을 명확하게 설명해주세요.
- **빌드 및 테스트 실행:** 저에게 빌드 또는 테스트를 요청할 수 있습니다.
- **질문:** 프로젝트의 코드나 구조에 대해 질문하면 답변해 드립니다.
