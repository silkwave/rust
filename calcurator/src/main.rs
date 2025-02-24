use eframe::egui;
use evalexpr::eval;
use std::env;
use std::io::{self, Write};

struct Calculator {
    input: String,
    result: String,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: String::new(),
        }
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(egui::RichText::new("Simple Calculator").size(30.0));
            
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Enter Expression:").size(20.0));
                ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(200.0));
            });
            
            ui.add_space(10.0);
            let button_size = egui::vec2(80.0, 80.0);
            let button_text_size = 24.0;
            
            for row in &[ ["7", "8", "9", "+"],
                          ["4", "5", "6", "-"],
                          ["1", "2", "3", "*"],
                          ["0", ".", "=", "/"] ] {
                ui.horizontal(|ui| {
                    for &num in row {
                        if ui.add_sized(button_size, egui::Button::new(egui::RichText::new(num).size(button_text_size))).clicked() {
                            if num == "=" {
                                self.result = match eval(&self.input) {
                                    Ok(value) => format!("{}", value),
                                    Err(_) => "Error".to_string(),
                                };
                            } else {
                                self.input.push_str(num);
                            }
                        }
                    }
                });
            }
            
            ui.add_space(10.0);
            if ui.add_sized(button_size, egui::Button::new(egui::RichText::new("Clear").size(button_text_size))).clicked() {
                self.input.clear();
                self.result.clear();
            }
            
            ui.add_space(10.0);
            ui.label(egui::RichText::new(format!("Result: {}", self.result)).size(24.0));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // Prevent Korean character corruption
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("LANG", "en_US.UTF-8");
    env::set_var("LC_ALL", "en_US.UTF-8");
    io::stdout().flush().unwrap();
    
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Box::new(Calculator::default())),
    )
}
