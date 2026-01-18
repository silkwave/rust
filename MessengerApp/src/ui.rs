// src/ui.rs
// 이 파일은 UI 렌더링 관련 함수들을 모아둡니다.

use eframe::egui;
use crate::{Message, User}; // Message와 User 구조체를 가져옵니다.

pub fn render_side_panel(ctx: &egui::Context, users: &Vec<User>) {
    egui::SidePanel::left("user_panel").show(ctx, |ui| {
        ui.heading("접속자 목록");
        ui.separator();
        for user in users {
            ui.label(format!("{} {}", if user.is_online { "✅" } else { "👤" }, user.name));
        }
    });
}

pub fn render_bottom_panel(
    ctx: &egui::Context,
    current_message_input: &mut String,
    chat_history: &mut Vec<Message>,
    user_name: &str,
) {
    egui::TopBottomPanel::bottom("input_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let res = ui.add(
                egui::TextEdit::singleline(current_message_input)
                    .hint_text("메시지를 입력하세요...")
                    .desired_width(f32::INFINITY),
            );

            // 엔터키를 누르거나 전송 버튼 클릭 시 메시지 추가
            if (ui.button("전송").clicked() || (res.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)))) 
               && !current_message_input.is_empty() 
            {
                chat_history.push(Message::new(
                    user_name.to_string(),
                    current_message_input.drain(..).collect(),
                ));
                res.request_focus(); // 입력창 포커스 유지
            }
        });
        ui.add_space(10.0);
    });
}

pub fn render_central_panel(ctx: &egui::Context, chat_history: &Vec<Message>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("💬 팀 채팅방");
        ui.separator();

        egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            for msg in chat_history {
                ui.horizontal(|ui| {
                    ui.label(msg.to_display_string());
                });
            }
        });
    });
}