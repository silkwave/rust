# Gemini AI의 프로젝트 분석

## 1. 프로젝트 개요

이 프로젝트는 Rust로 작성된 고속 푸리에 변환(FFT) 알고리즘을 WebAssembly(Wasm)로 컴파일하여 웹 브라우저에서 시각화하는 데모입니다. Rust의 연산 성능을 활용하여 JavaScript 환경에서 무거운 계산을 효율적으로 처리하는 방법을 보여줍니다.

## 2. 주요 기술

- **Rust**: 핵심 FFT 알고리즘 구현에 사용되었습니다.
- **WebAssembly (Wasm)**: Rust 코드를 웹 브라우저에서 실행 가능한 바이너리 형식으로 컴파일한 결과물입니다.
- **`wasm-bindgen`**: Rust와 JavaScript 간의 상호 운용성을 용이하게 하는 브릿지 역할을 합니다. Rust 함수를 JavaScript에서 직접 호출할 수 있게 해줍니다.
- **JavaScript**: Wasm 모듈을 로드하고, 입력 데이터를 준비하며, FFT 결과를 받아 캔버스(Canvas)에 시각화하는 역할을 담당합니다.

## 3. 프로젝트 구조

- `Cargo.toml`: Rust 프로젝트의 설정 파일입니다. 패키지 정보, 의존성(`wasm-bindgen`), 그리고 라이브러리 출력 형식(`cdylib`) 등을 정의합니다.
- `src/lib.rs`: FFT 알고리즘의 핵심 로직이 담긴 Rust 소스 코드입니다.
  - `Complex` 구조체: 복소수를 표현합니다.
  - `fft` 함수: 재귀적으로 FFT를 수행하는 내부 함수입니다.
  - `fft_js` 함수: `#[wasm_bindgen]` 어트리뷰트를 통해 JavaScript에 노출되는 함수로, 실수와 허수 배열을 입력받아 FFT를 수행하고 결과를 반환합니다.
- `index.html`: 웹 애플리케이션의 UI와 로직을 담고 있습니다.
  - Wasm 모듈(`pkg/fft_wasm.js`)을 동적으로 로드합니다.
  - 'FFT 실행' 버튼 클릭 시, 테스트용 신호를 생성하여 `fft_js` 함수를 호출합니다.
  - 반환된 주파수 데이터의 진폭을 계산하여 `<canvas>`에 그래프로 그립니다.
- `pkg/`: `wasm-pack`으로 빌드 시 생성되는 디렉토리로, 컴파일된 Wasm 파일과 JavaScript 연동을 위한 파일들이 포함됩니다.

## 4. 빌드 및 실행 방법

이 프로젝트를 로컬 환경에서 실행하려면 `wasm-pack`과 간단한 웹 서버가 필요합니다.

1.  **`wasm-pack` 설치**:
    ```bash
    cargo install wasm-pack
    ```

2.  **WebAssembly 모듈 빌드**:
    프로젝트 루트 디렉토리에서 다음 명령어를 실행하여 Rust 코드를 Wasm으로 컴파일합니다. 이 과정에서 `pkg` 디렉토리가 생성됩니다.
    ```bash
    wasm-pack build --target web
    ```

3.  **로컬 웹 서버 실행**:
    `index.html` 파일이 있는 디렉토리에서 웹 서버를 실행해야 합니다. Python이 설치되어 있다면 간단하게 사용할 수 있습니다.
    ```bash
    # Python 3.x
    python -m http.server
    ```
    또는 `npm`을 사용한다면 `serve` 패키지를 이용할 수 있습니다.
    ```bash
    npm install -g serve
    serve .
    ```

4.  **브라우저에서 확인**:
    웹 서버가 실행되면, 터미널에 표시된 주소(예: `http://localhost:8000`)로 접속하여 'FFT 실행' 버튼을 클릭하면 결과가 캔버스에 나타납니다.