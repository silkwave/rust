use std::io;
use std::process::Command;

fn clear_screen() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if cfg!(target_os = "windows") {
        println!("windows 환경에서 cls 실행\n");
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        println!("linux/mac 환경에서 clear 실행\n");
        Command::new("clear").status().unwrap();
    }
}


use std::sync::Arc;
use std::thread;

fn main() {
    let num = Arc::new(100);

    let mut handles = vec![];

    for i in 0..3 {
        let n = Arc::clone(&num);
        let handle = thread::spawn(move || {
            println!("스레드 {i}: num = {}", n);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("최종 참조 개수: {}", Arc::strong_count(&num));
}
