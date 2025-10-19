use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

#[derive(Serialize, Deserialize)]
struct DanResult {
    dan: i32,
    table: Vec<String>,
}

/// Generates the multiplication table for a given number.
fn generate_dan_result(dan: i32) -> Option<DanResult> {
    // Guard Clause: 유효하지 않은 단(dan)은 처리하지 않고 바로 반환합니다.
    if !(2..=9).contains(&dan) {
        return None;
    }

    let mut table = Vec::new();

    // 각 단에서 1부터 9까지 반복합니다.
    for i in 1..=9 {
        table.push(format!("{} * {} = {}", dan, i, dan * i));
    }
    
    Some(DanResult { dan, table })
}

/// 구구단 결과를 JSON 파일로 저장합니다.
fn save_results_to_json(filename: &str, data: &Vec<DanResult>) -> io::Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

/// JSON 파일을 읽고 내용을 출력합니다.
fn read_and_print_json(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    // JSON을 DanResult 벡터로 역직렬화합니다.
    let results: Vec<DanResult> = serde_json::from_reader(reader)?;

    println!("\n--- 'gugudan.json' 파일 내용 ---");
    for result in results {
        println!("--- {}단 ---", result.dan);
        for line in result.table {
            println!("{}", line);
        }
        println!(); // 각 단 사이에 공백 추가
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let filename = "gugudan.json";

    // 2단부터 9단까지의 결과를 병렬로 생성하여 수집합니다.
    let all_results: Vec<DanResult> = (2..=9)
        .into_par_iter()
        .filter_map(generate_dan_result) // map에서 filter_map으로 변경하여 None 값을 걸러냅니다.
        .collect();

    // 생성된 결과를 "gugudan.json" 파일에 저장합니다.
    save_results_to_json(filename, &all_results)?;
    println!("'{}' 파일에 결과가 성공적으로 저장되었습니다.", filename);

    // 저장된 파일을 다시 읽어서 내용을 출력합니다.
    read_and_print_json(filename)?;

    Ok(())
}
