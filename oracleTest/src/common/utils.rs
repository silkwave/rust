use std::fs;

/// 현재 프로세스의 RSS 메모리 사용량 (KB)을 반환합니다.
/// /proc/self/status 파일을 읽어 VmRSS 값을 파싱합니다.
pub fn current_rss_kb() -> u64 {
    // /proc/self/status 파일은 리눅스 계열 시스템에서 프로세스 정보를 담고 있습니다.
    // 이 파일을 읽어 "VmRSS" (Resident Set Size) 값을 찾습니다.
    let status = fs::read_to_string("/proc/self/status").unwrap();
    for line in status.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // "VmRSS:" 다음의 숫자 값을 파싱하여 u64 타입으로 반환합니다.
            return parts[1].parse::<u64>().unwrap();
        }
    }
    0
}
