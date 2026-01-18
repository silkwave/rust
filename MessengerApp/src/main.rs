use eframe::egui;
use egui::{FontFamily, FontId};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "사내 메신저 v1.0",
        native_options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();

            // NotoSansKR 폰트를 로드합니다.
            fonts.font_data.insert(
                "NotoSansKR".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSansKR-Regular.otf")),
            );

            // NotoSansKR 폰트를 기본 폰트로 설정합니다.
            fonts.families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "NotoSansKR".to_owned());
            
            fonts.families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, "NotoSansKR".to_owned());

            let mut app = MessengerApp::default();
            cc.egui_ctx.set_fonts(fonts);
            
            // 모든 텍스트 스타일에 대해 NotoSansKR을 기본 폰트로 설정합니다.
            cc.egui_ctx.style_mut(|style| {
                for (_text_style, font_id) in style.text_styles.iter_mut() {
                    *font_id = FontId::new(font_id.size, FontFamily::Proportional);
                }
            });

            Ok(Box::new(app))
        }),
    )
}

struct MessengerApp {
    chat_history: Vec<(String, String)>, // (이름, 메시지)
    current_message: String,
    user_name: String,
}

impl Default for MessengerApp {
    fn default() -> Self {
        Self {
            chat_history: vec![
                ("시스템".to_string(), "사내 메신저에 접속되었습니다.".to_string()),
            ],
            current_message: String::new(),
            user_name: "나".to_string(),
        }
    }
}

impl eframe::App for MessengerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. 왼쪽 사이드바 (사용자 목록)
        egui::SidePanel::left("user_panel").show(ctx, |ui| {
            ui.heading("접속자 목록");
            ui.separator();
            ui.label("👤 김철수 팀장");
            ui.label("👤 이영희 대리");
            ui.label("✅ 나 (온라인)");
        });

        // 2. 하단 입력창 영역
        egui::TopBottomPanel::bottom("input_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let res = ui.add(
                    egui::TextEdit::singleline(&mut self.current_message)
                        .hint_text("메시지를 입력하세요...")
                        .desired_width(f32::INFINITY),
                );

                // 엔터키를 누르거나 전송 버튼 클릭 시 메시지 추가
                if (ui.button("전송").clicked() || (res.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)))) 
                   && !self.current_message.is_empty() 
                {
                    self.chat_history.push((self.user_name.clone(), self.current_message.clone()));
                    self.current_message.clear();
                    res.request_focus(); // 입력창 포커스 유지
                }
            });
            ui.add_space(10.0);
        });

        // 3. 중앙 채팅창 영역
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💬 팀 채팅방");
            ui.separator();

            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                for (name, msg) in &self.chat_history {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("{}:", name)).strong());
                        ui.label(msg);
                    });
                }
            });
        });
    }
}
