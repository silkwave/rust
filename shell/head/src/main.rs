use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() {
    // 명령행 인수 가져오기
    let args: Vec<String> = env::args().collect();

    // 기본값 설정: 10줄 표시
    let mut lines_to_show = 10;
    let mut files = Vec::new();
    let mut i = 1;

    // 명령행 인수 파싱
    while i < args.len() {
        match args[i].as_str() {
            "-n" => {
                // 줄 수 옵션 처리
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(n) => lines_to_show = n,
                        Err(_) => {
                            eprintln!("head: invalid number of lines: {}", args[i + 1]);
                            process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("head: option requires an argument -- n");
                    process::exit(1);
                }
            }
            arg => {
                // 파일 이름으로 처리
                files.push(arg.to_string());
                i += 1;
            }
        }
    }

    // 파일이 없으면 표준 입력에서 읽기
    if files.is_empty() {
        if let Err(e) = head_from_stdin(lines_to_show) {
            eprintln!("head: {}", e);
            process::exit(1);
        }
    } else {
        // 여러 파일 처리
        for (idx, file) in files.iter().enumerate() {
            if files.len() > 1 {
                if idx > 0 {
                    println!();
                }
                println!("==> {} <==", file);
            }

            if let Err(e) = head_from_file(file, lines_to_show) {
                eprintln!("head: {}: {}", file, e);
                process::exit(1);
            }
        }
    }
}

// 파일에서 지정된 줄 수만큼 읽어 출력하는 함수
fn head_from_file(filename: &str, lines: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // 지정된 줄 수만큼만 읽어서 출력
    for (line_num, line) in reader.lines().enumerate() {
        if line_num >= lines {
            break;
        }
        println!("{}", line?);
    }

    Ok(())
}

// 표준 입력에서 지정된 줄 수만큼 읽어 출력하는 함수
fn head_from_stdin(lines: usize) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    // 지정된 줄 수만큼만 읽어서 출력
    for (line_num, line) in reader.lines().enumerate() {
        if line_num >= lines {
            break;
        }
        println!("{}", line?);
    }

    Ok(())
}
