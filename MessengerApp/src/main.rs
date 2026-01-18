#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use chrono::{Local, DateTime};
use serde::{Deserialize, Serialize};

mod ui; // Add this line

// 메시지 구조체 정의
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Message {
    sender: String,
    content: String,
    timestamp: DateTime<Local>,
}

impl Message {
    fn new(sender: String, content: String) -> Self {
        Self {
            sender,
            content,
            timestamp: Local::now(),
        }
    }

    fn to_display_string(&self) -> String {
        format!(
            "[{}] {}: {}",
            self.timestamp.format("%H:%M"),
            self.sender,
            self.content
        )
    }
}

// 사용자 구조체 정의
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    is_online: bool,
}

impl User {
    fn new(id: String, name: String, is_online: bool) -> Self {
        Self { id, name, is_online }
    }
}

// 메신저 애플리케이션의 상태를 관리하는 구조체
struct ChatApp {
    chat_history: Vec<Message>,
    current_message_input: String,
    user_name: String,
    users: Vec<User>,
}

impl Default for ChatApp {
    fn default() -> Self {
        Self {
            chat_history: vec![
                Message::new("시스템".to_string(), "사내 메신저에 접속되었습니다.".to_string()),
            ],
            current_message_input: String::new(),
            user_name: "나".to_string(),
            users: vec![
                User::new("kim".to_string(), "김철수 팀장".to_string(), true),
                User::new("lee".to_string(), "이영희 대리".to_string(), true),
                User::new("me".to_string(), "나".to_string(), true),
            ],
        }
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::render_side_panel(ctx, &self.users);
        ui::render_bottom_panel(ctx, &mut self.current_message_input, &mut self.chat_history, &self.user_name);
        ui::render_central_panel(ctx, &self.chat_history);
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let result = eframe::run_native(
        "사내 메신저 v1.0",
        native_options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();

            // 폰트 데이터를 로드합니다.
            let font_data = include_bytes!("../assets/fonts/Pretendard-Regular.ttf");

            // 더미 파일인지 확인합니다. 실제 폰트 파일이 아니면 로드하지 않습니다.
            if font_data.starts_with(b"This is a dummy") {
                eprintln!("경고: assets/fonts/Pretendard-Regular.ttf 파일이 더미 파일입니다. 실제 폰트 파일로 교체해주세요.");
            } else {
                // Pretendard 폰트를 로드합니다.
                fonts.font_data.insert(
                    "Pretendard".to_owned(),
                    egui::FontData::from_static(font_data),
                );

                // Pretendard 폰트를 기본 폰트로 설정합니다.
                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "Pretendard".to_owned());
                
                fonts.families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, "Pretendard".to_owned());
            }

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(ChatApp::default())) // MessengerApp 대신 ChatApp 사용
        }),
    );

    if let Err(e) = result {
        eprintln!("❌ 애플리케이션 실행 오류: {}", e);
        eprintln!("💡 팁: 이 프로그램은 GUI 환경이 필요합니다. SSH나 디스플레이가 없는 환경에서는 실행되지 않습니다.");
        eprintln!("   Windows용으로 빌드하려면 다음 명령어를 사용하세요: cargo build --release --target x86_64-pc-windows-gnu");
    }
}

