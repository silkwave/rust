use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom};
use std::path::Path;

// 직렬화/역직렬화를 위한 Todo 아이템 구조체
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: usize,
    task: String,
    completed: bool,
}

// 할 일 목록을 관리하는 구조체
#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    // 새 할 일 목록 생성
    fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    // 파일에서 할 일 목록 불러오기 (파일이 없으면 새로 생성)
    fn load_from_file(filename: &str) -> io::Result<Self> {
        if !Path::new(filename).exists() {
            return Ok(TodoList::new());
        }
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let list = match serde_json::from_reader(reader) {
            Ok(list) => list,
            Err(e) if e.is_eof() => TodoList::new(), // 파일이 비어있으면 새 목록
            Err(e) => {
                eprintln!(
                    "경고: '{}' 파일 분석 중 오류 발생. 새 목록을 시작합니다. (오류: {})",
                    filename, e
                );
                TodoList::new()
            }
        };
        Ok(list)
    }

    // 파일에 할 일 목록 저장하기
    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // 파일이 존재하면 내용을 지움
            .open(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    // 새로운 할 일 추가
    fn add_task(&mut self, task: String) {
        let new_id = self.todos.last().map_or(1, |t| t.id + 1);
        let new_todo = Todo {
            id: new_id,
            task,
            completed: false,
        };
        println!(
            "'{}' 할 일이 추가되었습니다. (ID: {})",
            new_todo.task, new_todo.id
        );
        self.todos.push(new_todo);
    }

    // 할 일 완료 처리
    fn complete_task(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            if !todo.completed {
                todo.completed = true;
                println!("'{}' 할 일을 완료했습니다. (ID: {})", todo.task, id);
            } else {
                println!("이미 완료된 할 일입니다. (ID: {})", id);
            }
        } else {
            println!("해당 ID({})의 할 일을 찾을 수 없습니다.", id);
        }
    }

    // 모든 할 일 목록 출력
    fn list_tasks(&self) {
        if self.todos.is_empty() {
            println!("할 일이 없습니다. 'add' 명령어로 추가해보세요!");
            return;
        }
        println!("--- 할 일 목록 ---");
        for todo in &self.todos {
            let status = if todo.completed { "[x]" } else { "[ ]" };
            println!("{} {} - {}", status, todo.id, todo.task);
        }
    }
}

// Clap을 이용한 CLI 명령어 정의
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "간단한 할 일 목록 CLI 앱")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 새로운 할 일을 추가합니다.
    Add {
        /// 추가할 할 일의 내용
        task: String,
    },
    /// 할 일 목록을 보여줍니다.
    List,
    /// 지정된 ID의 할 일을 완료 처리합니다.
    Done {
        /// 완료할 할 일의 ID
        id: usize,
    },
}

fn main() -> io::Result<()> {
    let filename = "todolist.json";
    let mut todo_list = TodoList::load_from_file(filename)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => {
            todo_list.add_task(task);
        }
        Commands::List => {
            todo_list.list_tasks();
        }
        Commands::Done { id } => {
            todo_list.complete_task(id);
        }
    }

    // 변경된 내용을 파일에 저장
    todo_list.save_to_file(filename)?;

    Ok(())
}
