use iced::widget::{button, column, row, text, container, Row};
use iced::{Alignment, Element, Length, Sandbox, Settings, Color};
use chrono::{Datelike, Local, NaiveDate, Duration};

pub fn main() -> iced::Result {
    CalendarApp::run(Settings::default())
}

struct CalendarApp {
    view_date: NaiveDate, // 현재 화면에 보여주는 달의 1일
}

#[derive(Debug, Clone)]
enum Message {
    NextMonth,
    PrevMonth,
}

impl Sandbox for CalendarApp {
    type Message = Message;

    fn new() -> Self {
        let now = Local::now().date_naive();
        Self {
            view_date: NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap(),
        }
    }

    fn title(&self) -> String {
        String::from("Rust Iced Calendar")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PrevMonth => {
                // 이전달 1일로 계산
                if let Some(prev) = self.view_date.checked_sub_months(chrono::Months::new(1)) {
                    self.view_date = prev;
                }
            }
            Message::NextMonth => {
                // 다음달 1일로 계산
                if let Some(next) = self.view_date.checked_add_months(chrono::Months::new(1)) {
                    self.view_date = next;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let year = self.view_date.year();
        let month = self.view_date.month();

        // 1. 헤더 (연도/월 및 이동 버튼)
        let header = row![
            button(text("<")).on_press(Message::PrevMonth),
            text(format!("  {}/{}  ", year, month)).size(30),
            button(text(">")).on_press(Message::NextMonth),
        ]
        .align_items(Alignment::Center);

        // 2. 요일 이름 행
        let weekdays = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
        // let weekdays = ["일화", "수", "목", "금", "토"];
        let days_header: Row<Message> = Row::with_children(
            weekdays.iter().map(|d| {
                text(*d).width(Length::Fill).horizontal_alignment(iced::alignment::Horizontal::Center).into()
            }).collect::<Vec<_>>()
        );

        // 3. 날짜 그리드 생성
        let mut calendar_column = column![header, days_header].spacing(20);
        
        // 해당 월의 첫 번째 요일 (0: 일, 1: 월 ...)
        let first_day_weekday = self.view_date.weekday().num_days_from_sunday() as i32;
        // 첫 번째 날짜 객체
        let mut current_day = self.view_date - Duration::days(first_day_weekday as i64);

        // 6주 분량의 날짜 출력
        for _week in 0..6 {
            let mut week_row = Row::new().spacing(10);
            for _day in 0..7 {
                let is_current_month = current_day.month() == self.view_date.month();
                
                let day_label = text(format!("{}", current_day.day()))
                    .style(if is_current_month { Color::BLACK } else { Color::from_rgb(0.7, 0.7, 0.7) });

                week_row = week_row.push(
                    container(day_label)
                        .width(Length::Fill)
                        .center_x()
                        .padding(10)
                );
                current_day = current_day + Duration::days(1);
            }
            calendar_column = calendar_column.push(week_row);
        }

        container(calendar_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .into()
    }
}