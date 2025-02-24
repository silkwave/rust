use eframe::egui;
use evalexpr::eval;
use std::env;
use std::io::{self, Write};
use std::mem;
use std::alloc::Layout;

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

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 중앙 패널을 사용하여 UI를 업데이트합니다.
        egui::CentralPanel::default().show(ctx, |ui| {
            // 계산기 제목을 표시합니다.
            ui.heading(egui::RichText::new("Simple Calculator").size(30.0)); // 제목 표시
            
            // 입력창을 표시하는 부분입니다.
            ui.add_space(10.0);  // 입력창과 다른 요소 사이에 여백을 추가
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Enter Expression:").size(20.0)); // "Enter Expression" 라벨 표시
                // 사용자가 수식을 입력할 수 있도록 단일 라인 텍스트 입력창을 표시
                ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(200.0)); // 수식 입력창
            });
            
            ui.add_space(10.0);  // 입력창과 버튼 사이에 여백 추가

            let button_size = egui::vec2(80.0, 80.0); // 버튼 크기 설정
            let button_text_size = 24.0; // 버튼 텍스트 크기 설정
            
            // 숫자 및 연산자 버튼을 배치합니다.
            for row in &[ 
                ["7", "8", "9", "+"],
                ["4", "5", "6", "-"],
                ["1", "2", "3", "*"],
                ["0", ".", "=", "/"]
            ] {
                ui.horizontal(|ui| {
                    // 각 버튼을 생성하고 클릭 이벤트를 처리합니다.
                    for &num in row {
                        if ui.add_sized(button_size, egui::Button::new(egui::RichText::new(num).size(button_text_size))).clicked() {
                            if num == "=" {
                                // "=" 버튼을 눌렀을 때, 입력된 수식을 평가하여 결과를 계산
                                self.result = match eval(&self.input) {
                                    Ok(value) => format!("{}", value), // 계산된 값을 문자열로 변환하여 결과에 저장
                                    Err(_) => "Error".to_string(), // 계산 오류가 발생한 경우 "Error"를 표시
                                };
                            } else {
                                // 숫자나 연산자를 입력할 때마다 입력창에 추가
                                self.input.push_str(num);
                            }
                        }
                    }
                });
            }
            
            ui.add_space(10.0); // 버튼과 Clear 버튼 사이에 여백 추가

            // Clear 버튼을 표시하여 클릭 시 입력 및 결과를 초기화합니다.
            if ui.add_sized(button_size, egui::Button::new(egui::RichText::new("Clear").size(button_text_size))).clicked() {
                self.input.clear(); // 수식 입력창 초기화
                self.result.clear(); // 결과 초기화
            }
            
            ui.add_space(10.0); // 결과와 메모리 사용량 사이에 여백 추가
            
            // 계산 결과를 화면에 표시합니다.
            ui.label(egui::RichText::new(format!("Result: {}", self.result)).size(24.0));

            // 메모리 사용량을 표시하는 부분입니다.
            ui.add_space(10.0); // 여백 추가
            let input_size = mem::size_of::<String>();  // input 변수 메모리 크기 계산
            let result_size = mem::size_of::<String>(); // result 변수 메모리 크기 계산
            // 화면에 메모리 사용량을 출력
            ui.label(egui::RichText::new(format!("Memory usage: input = {} bytes, result = {} bytes", input_size, result_size)).size(16.0));

            // 동적 메모리 사용량 추적
            ui.add_space(10.0); // 여백 추가
            let input_dynamic_size = get_dynamic_size_of_string(&self.input);
            let result_dynamic_size = get_dynamic_size_of_string(&self.result);
            ui.label(egui::RichText::new(format!("Dynamic memory size: input = {} bytes, result = {} bytes", input_dynamic_size, result_dynamic_size)).size(16.0));
        });
    }
}

// 문자열의 동적 메모리 크기를 계산하는 함수
fn get_dynamic_size_of_string(s: &String) -> usize {
    let layout = Layout::for_value(s);
    layout.size()
}

fn main() -> Result<(), eframe::Error> {
    // 한글 출력 깨짐 방지를 위한 환경 변수 설정
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("LANG", "en_US.UTF-8");
    env::set_var("LC_ALL", "en_US.UTF-8");
    io::stdout().flush().unwrap(); // 표준 출력 버퍼를 플러시하여 한글 깨짐 방지

    let options = eframe::NativeOptions::default(); // 기본 네이티브 옵션 설정
    // `eframe::run_native`를 사용하여 계산기 애플리케이션을 실행
    eframe::run_native(
        "Calculator", // 애플리케이션 제목
        options,      // 네이티브 옵션
        Box::new(|_cc| Box::new(Calculator::default())), // 계산기 기본 상태로 초기화
    )
}
