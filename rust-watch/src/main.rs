use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

// Counter 구조체 정의
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    // 새로운 Counter 인스턴스를 생성하는 연관 함수
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

// Counter에 Iterator 트레이트 구현
impl Iterator for Counter {
    type Item = u32; // 이터레이터가 반환할 값의 타입

    // 다음 항목을 반환하는 next 메서드 구현
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count) // 현재 카운트를 Some으로 감싸 반환
        } else {
            None // max에 도달하면 None 반환하여 이터레이션 종료
        }
    }
}

fn main() {
    // 1. 명령행 인자 받기 (예: rust-watch 2 ls -la)
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("사용법: rust-watch <초> <명령어> [인자...]");
        // 명령행 인자가 부족할 경우 이터레이터 예제를 실행
        run_iterator_example();
        return;
    }

    // 2. 실행 주기(초) 설정
    let interval = args[1].parse::<u64>().unwrap_or(2);
    let cmd = &args[2];
    let cmd_args = &args[3..];

    loop {
        // 3. 화면 지우기 (ANSI Escape Code 사용)
        // \x1B[2J: 화면 전체 지우기, \x1B[H: 커서를 왼쪽 위로 이동
        print!("\x1B[2J\x1B[H");
        
        println!("정기 실행 중: {} {:?} (주기: {}초)", cmd, cmd_args, interval);
        println!("---------------------------------------");

        // 4. 명령어 실행
        let output = Command::new(cmd)
            .args(cmd_args)
            .output();

        match output {
            Ok(out) => {
                io::stdout().write_all(&out.stdout).unwrap();
                io::stderr().write_all(&out.stderr).unwrap();
            }
            Err(e) => {
                eprintln!("명령어 실행 실패: {}", e);
            }
        }

        // 5. 지정된 시간만큼 대기
        thread::sleep(Duration::from_secs(interval));
    }
}

fn run_iterator_example() {
    println!("\n--- 이터레이터 예제 ---");
    let mut counter = Counter::new(5);

    println!("Counter 이터레이션 (next() 호출):");
    // next() 메서드를 직접 호출하여 이터레이터 사용
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }

    println!("\nCounter 이터레이션 (for 루프 사용):");
    // for 루프는 Iterator 트레이트를 구현한 타입에서 동작
    for number in Counter::new(3) {
        println!("  {}", number);
    }

    println!("\nCounter 이터레이션 (collect 사용):");
    // collect() 메서드를 사용하여 이터레이터의 모든 항목을 벡터로 수집
    let collected_items: Vec<u32> = Counter::new(4).collect();
    println!("  수집된 항목: {:?}", collected_items);

    println!("\n--- 이터레이터 예제 종료 ---");
}
