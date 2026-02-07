use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// 파일 내용을 출력하는 함수
/// show_number가 true이면 라인 번호를 함께 출력
fn print_file(path: &str, show_number: bool, line_counter: &mut usize) -> io::Result<()> {
    // 파일 열기
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // 파일을 한 줄씩 읽어서 출력
    for line in reader.lines() {
        let line = line?;
        if show_number {
            // 라인 번호와 함께 출력 (6자리 포맷)
            println!("{:6}\t{}", *line_counter, line);
            *line_counter += 1;
        } else {
            // 라인 번호 없이 출력
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // 커맨드 라인 인자 수집
    let args: Vec<String> = env::args().collect();

    // 인자가 부족한 경우 사용법 출력
    if args.len() < 2 {
        eprintln!("Usage: rcat [-n] <file>...");
        std::process::exit(1);
    }

    // 옵션과 파일 목록 분리
    let mut show_number = false;
    let mut files = Vec::new();

    // 인자 파싱
    for arg in &args[1..] {
        if arg == "-n" {
            show_number = true; // 라인 번호 표시 옵션
        } else {
            files.push(arg); // 파일 목록에 추가
        }
    }

    // 파일이 지정되지 않은 경우
    if files.is_empty() {
        eprintln!("No files provided.");
        std::process::exit(1);
    }

    // 라인 카운터 초기화
    let mut line_counter = 1;

    // 각 파일을 순서대로 처리
    for file in files {
        if let Err(e) = print_file(file, show_number, &mut line_counter) {
            // 파일 처리 중 오류 발생 시 에러 메시지 출력
            eprintln!("rcat: {}: {}", file, e);
        }
    }

    // 출력 버퍼 비우기
    io::stdout().flush()?;
    Ok(())
}
