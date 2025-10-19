use eframe::egui;
use evalexpr::eval;

// 계산기 구조체 정의
// 이 구조체는 사용자가 입력한 수식(input)과 계산 결과(result)를 저장합니다.
struct Calculator {
    input: String,  // 사용자가 입력한 수식
    result: String, // 계산 결과
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: String::new(),  // 초기에는 빈 문자열로 설정
            result: String::new(), // 초기에는 빈 문자열로 설정
        }
    }
}

impl Calculator {
    /// 버튼 클릭 이벤트를 처리하는 헬퍼 함수입니다.
    fn handle_button_click(&mut self, label: &str) {
        match label {
            "=" => {
                // 입력된 수식을 평가하여 결과를 계산합니다.
                self.result = match eval(&self.input) {
                    Ok(value) => format!("{}", value),
                    Err(_) => "Error".to_string(),
                };
            }
            "Clear" => {
                // 입력과 결과를 모두 지웁니다.
                self.input.clear();
                self.result.clear();
            }
            _ => {
                // 그 외의 버튼은 입력 문자열에 추가합니다.
                self.input.push_str(label);
            }
        }
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 중앙 패널을 사용하여 UI를 업데이트합니다.
        egui::CentralPanel::default().show(ctx, |ui| {
            // 계산기 제목을 표시합니다.
            ui.heading(egui::RichText::new("Simple Calculator").size(30.0)); // 제목 표시

            // 입력창을 표시하는 부분입니다.
            ui.add_space(10.0); // 입력창과 다른 요소 사이에 여백을 추가
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Enter Expression:").size(20.0)); // "Enter Expression" 라벨 표시
                                                                               // 사용자가 수식을 입력할 수 있도록 단일 라인 텍스트 입력창을 표시
                ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(200.0));
                // 수식 입력창
            });

            ui.add_space(10.0); // 입력창과 버튼 사이에 여백 추가

            let button_size = egui::vec2(80.0, 80.0); // 버튼 크기 설정
            let button_text_size = 24.0; // 버튼 텍스트 크기 설정

            // 숫자 및 연산자 버튼을 배치합니다.
            // Clear 버튼까지 레이아웃에 포함하여 UI 생성을 단일 루프로 처리합니다.
            let button_layout: &[&[&str]] = &[
                &["7", "8", "9", "+"],
                &["4", "5", "6", "-"],
                &["1", "2", "3", "*"],
                &["0", ".", "=", "/"],
                &["Clear"],
            ];

            for row in button_layout {
                ui.horizontal(|ui| {
                    // 각 버튼을 생성하고 클릭 이벤트를 처리합니다.
                    for &label in *row {
                        if ui
                            .add_sized(
                                button_size,
                                egui::Button::new(
                                    egui::RichText::new(label).size(button_text_size),
                                ),
                            )
                            .clicked()
                        {
                            self.handle_button_click(label);
                        }
                    }
                });
            }

            ui.add_space(10.0); // 결과와 메모리 사용량 사이에 여백 추가

            // 계산 결과를 화면에 표시합니다.
            ui.label(egui::RichText::new(format!("Result: {}", self.result)).size(24.0));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default(); // 기본 네이티브 옵션 설정
                                                    // `eframe::run_native`를 사용하여 계산기 애플리케이션을 실행
    eframe::run_native(
        "Calculator",                                    // 애플리케이션 제목
        options,                                         // 네이티브 옵션
        Box::new(|_cc| Box::new(Calculator::default())), // 계산기 기본 상태로 초기화
    )
}
