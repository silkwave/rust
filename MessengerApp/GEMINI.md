# 사내 메신저 (MessengerApp)

## 프로젝트 개요
1. 코드 설명 및 보완점
    *   **UI 구성**: `eframe`과 `egui`를 사용하여 직관적인 인터페이스를 구성했습니다. `ScrollArea`를 통해 메시지가 많아져도 스크롤이 가능합니다. UI는 `ctx.request_repaint()`를 통해 지속적으로 갱신되어 새로운 메시지를 즉시 표시합니다.

    *   **한글 폰트 적용**: 한글 깨짐 문제를 해결하기 위해 `Pretendard-Regular.ttf` 폰트를 `assets/fonts/Pretendard-Regular.ttf` 경로에서 로드하여 `egui`의 기본 폰트로 설정했습니다. 이는 `P2PChatApp::new` 함수에서 `egui::FontDefinitions`를 사용하여 `Pretendard-Regular` 폰트를 추가하고 `Proportional` 및 `Monospace` 폰트 계열의 첫 번째 폰트로 지정함으로써 이루어집니다.

    *   **P2P 통신**: `std::net::UdpSocket`을 사용하여 서버 없이 데이터를 주고받습니다. `UdpSocket::bind("0.0.0.0:8080")`을 통해 8080 포트에 바인딩하며, `socket.set_broadcast(true)`를 설정하여 브로드캐스트 통신을 가능하게 합니다. 같은 LAN 환경이라면 상대방의 로컬 IP를 입력하여 즉시 대화가 가능합니다.

    *   **멀티스레딩**: 메시지 수신 로직은 별도의 `thread::spawn` 스레드에서 실행되어 UI가 멈추지 않도록(Non-blocking) 설계했습니다. 메시지 기록은 `Arc<Mutex<Vec<String>>>`를 사용하여 여러 스레드 간에 안전하게 공유되고 접근됩니다.

    *   **보안 강화**: 메시지 전송 시 `aes-gcm` 크레이트를 이용한 AES-256-GCM 암호화를 적용했습니다. 메시지를 보내기 전에 `nonce`와 함께 암호화하고, 수신된 메시지는 `hex` 디코딩 후 복호화하여 보안을 강화합니다. 이를 위해 `KEY` 상수를 정의하고 `Aes256Gcm` 암호화기를 초기화하여 사용합니다.

    *   **사용자 자동 탐색**: UDP 브로드캐스트 기능을 활용하여 네트워크 내의 다른 사용자를 자동으로 탐색합니다. `P2PChatApp::new` 함수 내에서 별도의 스레드가 `8081` 포트를 통해 주기적으로 암호화된 `DISCOVERY_PING` 메시지를 브로드캐스트하고, 응답을 수신하여 `discovered_users` 목록을 업데이트합니다. UI에서는 발견된 사용자 목록을 표시하고, 클릭 시 해당 사용자의 IP를 대상 IP로 설정할 수 있도록 합니다.

2.  **Windows 실행 파일 생성**:
    ```bash
    cargo build --release --target x86_64-pc-windows-gnu
    ```
    생성된 실행 파일: `target/x86_64-pc-windows-gnu/release/MessengerApp.exe`

## 이터레이터 예제
Rust에서는 `Iterator` 트레이트를 구현하여 반복자를 만들 수 있습니다. 다음은 0부터 n까지 숫자를 생성하는 간단한 반복자 예제입니다.

```rust
struct Counter {
    count: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count - 1)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new(5);
    for i in counter {
        println!("{}", i); // 0, 1, 2, 3, 4 출력
    }

    let sum: usize = Counter::new(4).sum(); // 0 + 1 + 2 + 3 = 6
    println!("Sum: {}", sum);
}
```

## 빌드 환경 및 문제 해결
*   **Windows GNU 크로스 컴파일 문제**:
    Linux 환경에서 `x86_64-pc-windows-gnu` 타겟으로 크로스 컴파일 시, `eframe`의 내부 의존성인 `winapi` 크레이트에서 `winuser` 및 `windef` 기능이 활성화되지 않아 컴파일 오류가 발생할 수 있습니다.
    이를 해결하기 위해 `Cargo.toml` 파일에 `winapi`를 직접 의존성으로 추가하고 필요한 기능을 명시적으로 활성화했습니다.

    ```toml
    winapi = { version = "0.3", features = ["winuser", "windef"] }
    ```
    이 설정은 `winapi` 크레이트가 올바른 Windows API 기능과 함께 컴파일되도록 강제하여, `eframe`이 해당 기능을 사용할 수 있도록 합니다.

## 개발 노트

## 라이선스
[필요한 경우 라이선스 추가]