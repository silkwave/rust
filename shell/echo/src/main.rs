// 표준 입력/출력 및 환경 변수 모듈 임포트
use std::env;
use std::io::{self, Write};

fn main() {
    // 커맨드 라인 인수를 벡터로 수집
    let args: Vec<String> = env::args().collect();

    // 인수가 있는 경우 (프로그램 이름 제외)
    if args.len() > 1 {
        // 인수들을 공백으로 연결하여 출력
        let output = args[1..].join(" ");
        println!("{}", output);
    } else {
        // 인수가 없는 경우 표준 입력에서 한 줄 읽기
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        // 읽은 내용을 바로 출력
        print!("{}", line);
        // 출력 버퍼 즉시 플러시
        io::stdout().flush().unwrap();
    }
}
