use std::fs::File;
use std::io::{self, Write};

// 이 함수는 파일을 생성하고 주어진 이름을 파일에 씁니다.
fn write_info(name: &str) -> io::Result<()> {
    
    // 파일을 생성합니다. '?' 연산자는 오류가 발생하면 전파합니다.
    let mut file = File::create("my_best_friends.txt")?;

    // 파일에 쓸 내용을 준비합니다.
    let content = format!("My best friend is {}.", name);

    // 내용을 파일에 씁니다.
    file.write_all(content.as_bytes())?;

    // 모든 것이 성공적이었으면 Ok를 반환합니다.
    Ok(())
}

fn main() {
    println!("This is a sample Rust program.");

    // 함수를 호출하고 결과를 처리합니다.
    match write_info("Alice") {
        Ok(()) => println!("File 'my_best_friends.txt' written successfully."),
        Err(e) => eprintln!("Failed to write file: {}", e),
    }
}
