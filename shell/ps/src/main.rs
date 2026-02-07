use clap::{Arg, Command};
use std::fs;

// 프로세스 정보 구조체
#[derive(Debug)]
struct ProcessInfo {
    pid: u32,     // 프로세스 ID
    comm: String, // 프로세스 명령어 이름
    ppid: u32,    // 부모 프로세스 ID
    tty_nr: u32,  // 터미널 번호
    utime: u64,   // 사용자 모드 CPU 시간
    stime: u64,   // 커널 모드 CPU 시간
}

// /proc/[pid]/stat 파일을 파싱하여 프로세스 정보 추출
fn parse_stat(pid: u32) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    // stat 파일 경로 생성
    let stat_path = format!("/proc/{}/stat", pid);
    let content = fs::read_to_string(&stat_path)?;

    // 공백으로 필드 분리
    let parts: Vec<&str> = content.split_whitespace().collect();

    // 최소 필드 개수 확인
    if parts.len() < 15 {
        return Err("Invalid stat format".into());
    }

    // 명령어 이름 추출 (괄호 안에 있음)
    let comm_start = content.find('(').unwrap_or(0) + 1;
    let comm_end = content.rfind(')').unwrap_or(content.len());
    let comm = content[comm_start..comm_end].to_string();

    // 필드값 파싱
    let pid_val: u32 = parts[0].parse()?; // PID
    let ppid_val: u32 = parts[3].parse()?; // PPID
    let tty_nr_val: u32 = parts[6].parse()?; // TTY 번호
    let utime_val: u64 = parts[13].parse()?; // 사용자 시간
    let stime_val: u64 = parts[14].parse()?; // 시스템 시간

    Ok(ProcessInfo {
        pid: pid_val,
        comm,
        ppid: ppid_val,
        tty_nr: tty_nr_val,
        utime: utime_val,
        stime: stime_val,
    })
}

// /proc 디렉토리에서 모든 프로세스 목록 가져오기
fn get_process_list() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();

    // /proc 디렉토리 읽기
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // 디렉토리 이름이 숫자(프로세스 ID)인지 확인
                if let Some(pid_str) = path.file_name().and_then(|n| n.to_str()) {
                    if pid_str.chars().all(|c| c.is_ascii_digit()) {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            // 프로세스 정보 파싱
                            if let Ok(process) = parse_stat(pid) {
                                processes.push(process);
                            }
                        }
                    }
                }
            }
        }
    }

    // PID로 정렬
    processes.sort_by_key(|p| p.pid);
    processes
}

// 프로세스 정보 출력 함수
fn print_processes(full_format: bool) {
    let processes = get_process_list();

    if full_format {
        // 전체 형식 출력 (-f 옵션)
        println!("PID          TTY          TIME CMD");
        for process in processes {
            // TTY 정보 포맷팅
            let tty = if process.tty_nr == 0 || process.tty_nr == 4194303 {
                "?".to_string() // 터미널 없음
            } else {
                format!("pts/{}", process.tty_nr - 1) // 의사 터미널
            };

            // CPU 시간 계산 (사용자+시스템 시간)
            let total_time = process.utime + process.stime;
            let seconds = total_time / 100; // 클럭 틱을 초로 변환
            let minutes = seconds / 60;
            let hours = minutes / 60;

            // 시간 형식 포맷팅
            let time_str = if hours > 0 {
                format!("{:02}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
            } else {
                format!("{:02}:{:02}", minutes, seconds % 60)
            };

            // 프로세스 정보 출력
            println!(
                "{:<12} {:<12} {:<8} {}",
                process.pid, tty, time_str, process.comm
            );
        }
    } else {
        // 기본 형식 출력
        println!("  PID TTY          TIME CMD");
        for process in processes {
            // TTY 정보 포맷팅
            let tty = if process.tty_nr == 0 || process.tty_nr == 4194303 {
                "?".to_string() // 터미널 없음
            } else {
                format!("pts/{}", process.tty_nr - 1) // 의사 터미널
            };

            // CPU 시간 계산 (사용자+시스템 시간)
            let total_time = process.utime + process.stime;
            let seconds = total_time / 100; // 클럭 틱을 초로 변환
            let minutes = seconds / 60;
            let hours = minutes / 60;

            // 시간 형식 포맷팅
            let time_str = if hours > 0 {
                format!("{:02}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
            } else {
                format!("{:02}:{:02}", minutes, seconds % 60)
            };

            // 프로세스 정보 출력
            println!(
                "{:5} {:<12} {:<8} {}",
                process.pid, tty, time_str, process.comm
            );
        }
    }
}

// 메인 함수
fn main() {
    // 명령행 인자 파싱
    let matches = Command::new("rust-ps")
        .version("0.1.0")
        .about("Rust로 구현한 ps 명령어")
        .arg(
            Arg::new("a")
                .short('a')
                .help("모든 프로세스 표시")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("e")
                .short('e')
                .help("모든 프로세스 표시")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("f")
                .short('f')
                .help("전체 형식으로 출력")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // 옵션 처리
    let _show_all = matches.get_flag("a") || matches.get_flag("e"); // 모든 프로세스 옵션
    let full_format = matches.get_flag("f"); // 전체 형식 옵션

    // 프로세스 목록 출력
    print_processes(full_format);
}
