use eframe::{egui, epaint::Color32}; // eframe GUI 라이브러리와 색상 관련 모듈 임포트
use evalexpr::eval; // 문자열 수식을 평가하기 위한 evalexpr 라이브러리 임포트

// iOS 계산기 앱의 색상 테마를 정의합니다.
const ORANGE: Color32 = Color32::from_rgb(255, 149, 0); // 주황색
const LIGHT_GRAY: Color32 = Color32::from_rgb(165, 165, 165); // 밝은 회색
const DARK_GRAY: Color32 = Color32::from_rgb(51, 51, 51); // 어두운 회색
const BACKGROUND: Color32 = Color32::BLACK; // 배경색은 검정색

// 계산기의 상태를 저장하는 구조체입니다.
struct Calculator {
    input: String,           // 현재 사용자 입력을 나타내는 문자열 (표시되는 숫자/수식)
    post_calculation: bool,  // 마지막으로 '=' 버튼을 눌러 계산을 완료했는지 여부
}

// Calculator 구조체에 대한 기본값 구현입니다.
impl Default for Calculator {
    fn default() -> Self {
        Self {
            input: "0".to_string(), // 초기 입력값은 "0"으로 설정
            post_calculation: false, // 초기에는 계산이 완료되지 않은 상태
        }
    }
}

// Calculator 구조체의 메서드를 구현합니다.
impl Calculator {
    /// 버튼 클릭 이벤트를 처리하는 핵심 함수입니다.
    fn handle_button_click(&mut self, label: &str) {
        let digits = "0123456789"; // 숫자 문자열 정의

        // 1. 숫자 버튼이 눌렸을 때의 처리 로직
        if digits.contains(label) {
            // 직전 계산 후에 숫자를 누르면 새로운 계산을 시작합니다.
            if self.post_calculation {
                self.input.clear();         // 입력창을 비우고
                self.post_calculation = false; // 계산 완료 상태를 초기화합니다.
            }
            // 현재 입력이 "0"일 경우, 새 숫자로 교체합니다. (예: "0" -> "5")
            if self.input == "0" {
                self.input.clear();
            }
            self.input.push_str(label); // 입력된 숫자를 현재 입력 문자열에 추가합니다.
            return; // 숫자 버튼 처리 후 함수 종료
        }

        // 2. 연산자 및 기능 버튼이 눌렸을 때의 처리 로직
        // 숫자 버튼이 아닌 다른 버튼이 눌리면 post_calculation 상태를 false로 재설정합니다.
        // 이는 새로운 연산을 시작할 수 있도록 합니다.
        self.post_calculation = false; 

        match label {
            "." => { // 소수점 버튼 처리
                // 직전 계산 후에 소수점을 누르면 "0."으로 시작합니다.
                if self.post_calculation {
                    self.input = "0".to_string();
                    self.post_calculation = false;
                }
                // 이미 소수점이 없으면 추가합니다. (중복 방지)
                if !self.input.contains('.') {
                    self.input.push('.');
                }
            }
            "=" => { // 등호 버튼 처리 (계산 실행)
                if !self.input.is_empty() {
                    // 사용자 친화적인 연산자 기호를 실제 계산 가능한 기호로 변경합니다.
                    let expression = self.input
                        .replace("÷", "/")
                        .replace("×", "*")
                        .replace("−", "-");
                    
                    // evalexpr 라이브러리를 사용하여 수식을 평가합니다.
                    self.input = match eval(&expression) {
                        Ok(value) => value.to_string(), // 성공하면 결과값을 문자열로 변환
                        Err(_) => "Error".to_string(), // 오류 발생 시 "Error" 표시
                    };
                    self.post_calculation = true; // 계산 완료 상태로 설정
                }
            }
            "AC" => { // All Clear (모두 지우기) 버튼 처리
                self.input = "0".to_string(); // 입력값을 "0"으로 초기화
                self.post_calculation = false; // 계산 완료 상태 초기화
            }
            "+/-" => { // 부호 변경 버튼 처리
                if self.input != "0" && !self.input.is_empty() {
                    if self.input.starts_with('-') {
                        self.input.remove(0); // 이미 음수면 '-' 제거하여 양수로
                    } else {
                        self.input.insert(0, '-'); // 양수면 '-' 추가하여 음수로
                    }
                }
            }
            "%" => { // 퍼센트 버튼 처리
                // 현재 입력값을 부동 소수점으로 파싱하여 100으로 나눕니다.
                if let Ok(val) = self.input.parse::<f64>() {
                    self.input = (val / 100.0).to_string();
                }
            }
            "÷" | "×" | "−" | "+" => { // 사칙연산자 버튼 처리
                self.post_calculation = false; // 연산자 누르면 계산 완료 상태 해제
                // 현재 입력의 마지막 문자가 이미 연산자이면 중복 추가를 방지합니다.
                if let Some(last_char) = self.input.chars().last() {
                    if !"÷×−+".contains(last_char) { // 마지막 문자가 연산자가 아니면
                        self.input.push_str(label); // 연산자를 추가합니다.
                    } else {
                        // 마지막 문자가 연산자라면 새 연산자로 교체합니다. (예: "5+" -> "5-")
                        self.input.pop(); // 이전 연산자 제거
                        self.input.push_str(label); // 새 연산자 추가
                    }
                } else if !self.input.is_empty() {
                    // 입력이 비어있지 않은데 마지막 문자가 없으면 (오류 케이스) 그냥 추가
                    self.input.push_str(label);
                }
            }
            _ => {} // 그 외의 버튼은 무시합니다. (예기치 않은 입력 방지)
        }
    }
}

// eframe::App 트레이트를 구현하여 애플리케이션의 UI와 동작을 정의합니다.
impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // eframe의 시각적 스타일을 설정합니다.
        let mut visuals = egui::Visuals::dark(); // 다크 모드 기본 설정
        visuals.window_fill = BACKGROUND; // 창 배경색을 검정으로 설정
        visuals.widgets.noninteractive.bg_fill = BACKGROUND; // 비활성 위젯의 배경도 검정으로 설정
        ctx.set_visuals(visuals); // 설정된 시각적 스타일을 적용합니다.

        // 중앙 패널에 UI 요소를 배치합니다.
        egui::CentralPanel::default().show(ctx, |ui| {
            // UI 상단에 디스플레이 영역을 위한 여백을 추가합니다.
            ui.add_space(ui.available_height() * 0.2); 

            // 계산 결과를 표시하는 디스플레이 영역입니다.
            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                // 입력 길이에 따라 동적으로 폰트 크기를 조절합니다.
                                const MAX_DIGITS_BEFORE_SCALE: usize = 6;         // 크기 조절이 시작되는 최대 글자 수
                const BASE_FONT_SIZE: f32 = 80.0;         // 기본 폰트 크기
                const MIN_FONT_SIZE: f32 = 30.0;          // 최소 폰트 크기

                let font_size = if self.input.len() > MAX_DIGITS_BEFORE_SCALE {
                    // 글자 수가 기준을 초과하면 비율에 따라 폰트 크기를 줄입니다.
                    let scale = MAX_DIGITS_BEFORE_SCALE as f32 / self.input.len() as f32;
                    (BASE_FONT_SIZE * scale).max(MIN_FONT_SIZE) // 최소 크기 이하로 작아지지 않도록 합니다.
                } else {
                    BASE_FONT_SIZE // 기준 이내일 경우 기본 크기를 사용합니다.
                };

                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&self.input) // 현재 입력(또는 결과)을 표시
                            .size(font_size) // 동적으로 계산된 폰트 크기 사용
                            .color(Color32::WHITE), // 폰트 색상
                    )
                    .wrap(false), // 텍스트 줄바꿈 방지
                );
            });
            
            ui.add_space(20.0); // 디스플레이와 버튼 사이에 여백 추가

            // 계산기 버튼의 레이아웃을 정의합니다.
            let button_layout: &[&[&str]] = &[
                &["AC", "+/-", "%", "÷"],
                &["7", "8", "9", "×"],
                &["4", "5", "6", "−"],
                &["1", "2", "3", "+"],
                &["0", ".", "="],
            ];

            // 버튼 크기를 UI의 사용 가능한 공간에 맞춰 동적으로 설정합니다.
            let button_size = egui::vec2(ui.available_width() / 4.5, ui.available_height() / 7.0);
            let button_text_size = 32.0; // 버튼 텍스트 크기

            // 각 버튼 행을 순회하며 UI에 추가합니다.
            for row in button_layout {
                ui.horizontal(|ui| { // 각 행을 가로로 배치합니다.
                    for &label in *row {
                        // 버튼의 종류에 따라 채우기 색상과 텍스트 색상을 설정합니다.
                        let (fill_color, text_color) = match label {
                            "÷" | "×" | "−" | "+" | "=" => (ORANGE, Color32::WHITE), // 주황색 버튼
                            "AC" | "+/-" | "%" => (LIGHT_GRAY, Color32::BLACK),     // 밝은 회색 버튼
                            _ => (DARK_GRAY, Color32::WHITE),                       // 어두운 회색 버튼 (숫자, 소수점)
                        };

                        let is_zero = label == "0"; // "0" 버튼인지 확인
                        // "0" 버튼은 두 칸 너비로 설정하고, 나머지 버튼은 일반 크기로 설정합니다.
                        let desired_size = if is_zero {
                            egui::vec2(button_size.x * 2.0 + ui.spacing().item_spacing.x, button_size.y)
                        } else {
                            button_size
                        };
                        
                        // 버튼 스타일을 동적으로 변경합니다.
                        let mut style = ui.style_mut().clone(); // 현재 UI 스타일을 복제
                        // 버튼 모서리를 둥글게 만듭니다 (90.0은 거의 원형에 가깝게 만듭니다).
                        style.visuals.widgets.inactive.rounding = egui::Rounding::same(90.0);
                        style.visuals.widgets.hovered.rounding = egui::Rounding::same(90.0);
                        style.visuals.widgets.active.rounding = egui::Rounding::same(90.0);
                        // 버튼의 배경색을 설정합니다.
                        style.visuals.widgets.inactive.bg_fill = fill_color;
                        // 호버(마우스 오버) 및 활성(클릭) 시 배경색을 약간 어둡게 합니다.
                        style.visuals.widgets.hovered.bg_fill = fill_color.linear_multiply(0.8);
                        style.visuals.widgets.active.bg_fill = fill_color.linear_multiply(0.7);
                        // 텍스트 색상을 설정합니다.
                        style.visuals.widgets.inactive.fg_stroke.color = text_color;


                        ui.style_mut().clone_from(&style); // 변경된 스타일을 UI에 적용합니다.

                        // 버튼을 생성합니다.
                        let button = egui::Button::new(egui::RichText::new(label).size(button_text_size));
                        
                        // 버튼을 UI에 추가하고 클릭 여부를 확인합니다.
                        if ui.add_sized(desired_size, button).clicked() {
                            self.handle_button_click(label); // 클릭 시 이벤트 핸들러 호출
                        }
                    }
                });
            }
        });
    }
}

// 메인 함수: 애플리케이션을 시작합니다.
fn main() -> Result<(), eframe::Error> {
    // eframe 네이티브 창의 초기 옵션을 설정합니다.
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 500.0)), // 초기 창 크기 설정
        ..Default::default() // 나머지 옵션은 기본값 사용
    };
    // eframe 애플리케이션을 실행합니다.
    eframe::run_native(
        "Calculator", // 애플리케이션 창 제목
        options,      // 설정된 옵션
        Box::new(|_cc| Box::new(Calculator::default())), // Calculator 앱의 기본 인스턴스를 생성하여 전달
    )
}
