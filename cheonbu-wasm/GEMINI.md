# Gemini 가이드

이 문서는 Gemini와 함께 이 프로젝트를 개발하기 위한 가이드입니다.

## 프로젝트 개요

이 프로젝트는 한국의 고전 경전인 **천부경(天符經)**을 주제로 한 인터랙티브 시각화 애플리케이션입니다. Rust를 사용하여 WebAssembly로 컴파일되었으며, HTML Canvas에 동적인 파티클 애니메이션을 렌더링합니다. 사용자는 웹 인터페이스를 통해 파티클의 수, 속도, 테마 등을 실시간으로 조작하며 시각적 변화를 탐색할 수 있습니다.

- **핵심 기술:** Rust, WebAssembly, `wasm-bindgen`, `web-sys`, JavaScript, HTML Canvas
- **주요 기능:**
    - 파티클 기반의 동적 애니메이션
    - 천부경 구절 순환 표시
    - 파티클 수, 속도, 리듬, 색상 테마 등 사용자 컨트롤 기능

## 프로젝트 구조

- `src/lib.rs`: 애플리케이션의 핵심 로직이 담긴 Rust 코드입니다. 파티클의 생성, 업데이트, 렌더링 및 JavaScript와의 상호작용을 위한 `App` 구조체를 정의합니다.
- `index.html`: 웹 페이지의 UI를 구성하고, WASM 모듈을 로드하여 실행하는 스크립트가 포함되어 있습니다. 사용자의 입력을 받아 Rust 코드의 함수를 호출하는 역할을 합니다.
- `Cargo.toml`: Rust 프로젝트의 의존성 및 빌드 설정을 관리하는 파일입니다. `wasm-bindgen`, `web-sys` 등의 라이브러리가 명시되어 있습니다.
- `pkg/`: `wasm-pack`으로 빌드한 결과물(WASM, JavaScript, TypeScript 정의)이 저장되는 디렉토리입니다.

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

## Gemini와 상호 작용

- **코드 변경 요청:** 특정 파일의 코드를 변경하려면 파일 경로와 함께 원하는 변경 사항을 명확하게 설명해주세요.
    - 예: "`src/lib.rs` 파일에 '우주(space)' 테마를 추가해주세요."
    - 예: "`index.html`에 있는 파티클 수 슬라이더의 최대값을 1000으로 변경해주세요."
- **빌드 및 테스트 실행:** 저에게 빌드 또는 테스트를 요청할 수 있습니다. (현재 프로젝트에는 자동화된 테스트가 없습니다.)
- **질문:** 프로젝트의 코드나 구조에 대해 질문하면 답변해 드립니다.
    - 예: "`src/lib.rs`의 `Particle` 구조체는 어떤 역할을 하나요?"