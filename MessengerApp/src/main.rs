#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use std::net::UdpSocket;
use std::collections::HashSet;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    Aes256Gcm, Nonce, Key
};
use hex;

const KEY: [u8; 32] = [1; 32]; // 32 bytes for AES256, all 1s for example
use std::sync::{Arc, Mutex};
use std::thread;

struct P2PChatApp {
    msg_history: Arc<Mutex<Vec<String>>>,
    input_text: String,
    target_ip: String,
    socket: Arc<UdpSocket>,
    cipher: Aes256Gcm,
    discovered_users: Arc<Mutex<HashSet<String>>>,
    discovery_active: bool,
}

impl P2PChatApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 8080 포트로 바인딩 (실제 구현시 포트 설정 필요)
        let socket = UdpSocket::bind("0.0.0.0:8080").expect("포트 바인딩 실패");
        socket.set_broadcast(true).ok();
        let socket = Arc::new(socket);
        let msg_history = Arc::new(Mutex::new(Vec::new()));

        // 폰트 설정 시작
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "Pretendard-Regular".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/Pretendard-Regular.ttf")),
        );

        // 기본 텍스트 스타일에 Pretendard 폰트 추가
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Pretendard-Regular".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "Pretendard-Regular".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        // 폰트 설정 끝

        // 암호화기 초기화
        let key = Key::<Aes256Gcm>::from_slice(&KEY);
        let cipher = Aes256Gcm::new(key);

        // 메시지 수신 스레드 시작
        let rx_socket = Arc::clone(&socket);
        let rx_history = Arc::clone(&msg_history);
        let rx_cipher = cipher.clone(); // 암호화기 클론

        thread::spawn(move || {
            let mut buf = [0u8; 2048]; // 암호화된 데이터를 위해 버퍼 크기 증가
            loop {
                if let Ok((size, addr)) = rx_socket.recv_from(&mut buf) {
                    let received_data = &buf[..size];
                    let hex_decoded_result = hex::decode(received_data);

                    match hex_decoded_result {
                        Ok(decoded_data) => {
                            if decoded_data.len() >= 12 { // Nonce는 12바이트 (96비트)
                                let nonce_bytes = &decoded_data[..12];
                                let ciphertext = &decoded_data[12..];
                                let nonce = Nonce::from_slice(nonce_bytes);

                                // 메시지 복호화
                                let decrypt_result = rx_cipher.decrypt(nonce, ciphertext);

                                match decrypt_result {
                                    Ok(plaintext) => {
                                        let msg = String::from_utf8_lossy(&plaintext);
                                        let mut history = rx_history.lock().unwrap();
                                        history.push(format!("[{} (복호화됨)]: {}", addr, msg));
                                    },
                                    Err(_) => {
                                        let mut history = rx_history.lock().unwrap();
                                        history.push(format!("[{} (복호화 실패)]: 유효하지 않은 암호화된 메시지", addr));
                                    }
                                }
                            } else {
                                let mut history = rx_history.lock().unwrap();
                                history.push(format!("[{} (오류)]: 짧은 암호화 데이터 수신", addr));
                            }
                        },
                        Err(_) => {
                            // 16진수 디코딩에 실패하면 평문 메시지 또는 손상된 데이터로 간주
                            let msg = String::from_utf8_lossy(received_data);
                            let mut history = rx_history.lock().unwrap();
                            history.push(format!("[{} (평문 또는 오류)]: {}", addr, msg));
                        }
                    }
                }
            }
        });

        let discovered_users = Arc::new(Mutex::new(HashSet::new()));
        let discovery_active = true; // Start discovery by default

        // UDP Broadcast용 소켓 생성 및 바인딩
        let discovery_socket = UdpSocket::bind("0.0.0.0:8081").expect("Discovery 포트 바인딩 실패");
        discovery_socket.set_broadcast(true).ok();
        let discovery_socket = Arc::new(discovery_socket);

        // Discovery 스레드 시작
        let discovery_socket_tx = Arc::clone(&discovery_socket); // 송신용
        let discovery_socket_rx = Arc::clone(&discovery_socket); // 수신용
        let discovery_cipher = cipher.clone(); // Discovery용 암호화기 클론
        let discovery_history = Arc::clone(&msg_history); // 메시지 기록용
        let discovered_users_clone = Arc::clone(&discovered_users);

        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            let broadcast_addr = "255.255.255.255:8081".to_string(); // 브로드캐스트 주소

            loop {
                // 1. 주기적으로 브로드캐스트 메시지 전송
                let ping_message = "DISCOVERY_PING";
                let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
                let ciphertext = discovery_cipher.encrypt(&nonce, ping_message.as_bytes()).expect("Discovery encryption failed!");

                let mut encrypted_data = Vec::new();
                encrypted_data.extend_from_slice(nonce.as_slice());
                encrypted_data.extend_from_slice(&ciphertext);
                let hex_encoded_data = hex::encode(&encrypted_data);

                discovery_socket_tx.send_to(hex_encoded_data.as_bytes(), &broadcast_addr).ok();

                // 2. 브로드캐스트 응답 수신 대기 및 처리
                // non-blocking으로 수신 시도
                discovery_socket_rx.set_read_timeout(Some(std::time::Duration::from_secs(1))).ok();
                if let Ok((size, addr)) = discovery_socket_rx.recv_from(&mut buf) {
                    let received_data = &buf[..size];
                    let hex_decoded_result = hex::decode(received_data);

                    match hex_decoded_result {
                        Ok(decoded_data) => {
                            if decoded_data.len() >= 12 {
                                let nonce_bytes = &decoded_data[..12];
                                let ciphertext = &decoded_data[12..];
                                let nonce = Nonce::from_slice(nonce_bytes);

                                let decrypt_result = discovery_cipher.decrypt(nonce, ciphertext);

                                if let Ok(plaintext) = decrypt_result {
                                    if String::from_utf8_lossy(&plaintext) == ping_message {
                                        // 자신에게 보낸 메시지 무시
                                        if addr.ip().is_loopback() || addr.ip().is_unspecified() {
                                            // do nothing
                                        } else {
                                            // 새로운 사용자 발견!
                                            let mut users = discovered_users_clone.lock().unwrap();
                                            if users.insert(addr.ip().to_string()) {
                                                let mut history = discovery_history.lock().unwrap();
                                                history.push(format!("[시스템]: 새로운 사용자 발견: {}", addr.ip()));
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Err(_) => {
                            // 디코딩 실패는 무시 (다른 메시지일 수 있음)
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(5)); // 5초마다 브로드캐스트
            }
        });


        Self {
            msg_history,
            input_text: String::new(),
            target_ip: "127.0.0.1:8080".to_string(), // 기본값
            socket,
            cipher,
            discovered_users,
            discovery_active,
        }
    }
}

impl eframe::App for P2PChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust P2P LAN Messenger (egui)");

            ui.horizontal(|ui| {
                ui.label("상대방 IP: ");
                ui.text_edit_singleline(&mut self.target_ip);
            });

            // 채팅 내역 출력 영역
            ui.separator();
            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                let history = self.msg_history.lock().unwrap();
                for msg in history.iter() {
                    ui.label(msg);
                }
            });

            // 메시지 입력 영역
            ui.separator();
            let re = ui.text_edit_singleline(&mut self.input_text);
            if ui.button("전송").clicked() || (re.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                if !self.input_text.is_empty() {
                    // 메시지 암호화
                    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96비트; 메시지마다 고유
                    let ciphertext = self.cipher.encrypt(&nonce, self.input_text.as_bytes()).expect("Encryption failed!");

                    // nonce와 암호문을 결합하여 전송을 위해 16진수 문자열로 인코딩
                    let mut encrypted_data = Vec::new();
                    encrypted_data.extend_from_slice(nonce.as_slice());
                    encrypted_data.extend_from_slice(&ciphertext);
                    let hex_encoded_data = hex::encode(&encrypted_data);

                    self.socket.send_to(hex_encoded_data.as_bytes(), &self.target_ip).ok();
                    let mut history = self.msg_history.lock().unwrap();
                    history.push(format!("[나 (암호화됨)]: {}", self.input_text)); // 암호화되었음을 표시
                    self.input_text.clear();
                }
            }
            // 발견된 사용자 목록 표시
            ui.separator();
            ui.heading("발견된 사용자");
            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                let users = self.discovered_users.lock().unwrap();
                if users.is_empty() {
                    ui.label("네트워크에서 사용자를 탐색 중입니다...");
                } else {
                    for user in users.iter() {
                        if ui.button(user).clicked() {
                            self.target_ip = format!("{}:8080", user); // 클릭 시 대상 IP 설정
                        }
                    }
                }
            });
        });
        
        // 지속적으로 화면을 갱신하여 새 메시지 표시
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "BeeBEEP Clone - Rust",
        native_options,
        Box::new(|cc| Box::new(P2PChatApp::new(cc))),
    )
}
