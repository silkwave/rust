use std::collections::HashMap;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Value {
    Int(i32),
    Bool(bool),
    Str(String),
    IntList(Vec<i32>),
    StrList(Vec<String>),
    Map(HashMap<String, Value>),
}

// Display 트레잇 구현 (출력용)
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::IntList(v) => write!(f, "{:?}", v),
            Value::StrList(v) => write!(f, "{:?}", v),
            Value::Map(m) => {
                write!(f, "{{ ")?;
                let mut iter = m.iter().peekable();
                while let Some((k, v)) = iter.next() {
                    write!(f, "\"{}\": {}", k, v)?;
                    if iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " }}")
            }
        }
    }
}

// 맵 출력 함수
fn print_map(title: &str, map: &HashMap<String, Value>) {
    println!("\n{}", title);
    for (key, value) in map {
        println!("키 '{}'의 값: {}", key, value);
    }
}

// ① 초기 값 설정
fn init_ctx_map() -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert("A".to_string(), Value::IntList(vec![1, 2, 3]));
    map.insert("B".to_string(), Value::Str("Hello".to_string()));
    map.insert("C".to_string(), Value::Int(123));
    map
}

// ② 값 추가
fn add_more_values(ctx_map: &mut HashMap<String, Value>) {
    ctx_map.insert("D".to_string(), Value::Bool(true));
    ctx_map.insert("E".to_string(), Value::StrList(vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
    ]));
}

// ③ data 맵 생성 및 ctx_map에 삽입
fn insert_data_map(ctx_map: &mut HashMap<String, Value>) {
    let mut data = HashMap::new();
    data.insert("a1".to_string(), Value::Str("a1".to_string()));
    data.insert("a2".to_string(), Value::Int(22));
    data.insert("a3".to_string(), Value::IntList(vec![1, 2, 3]));
    ctx_map.insert("data".to_string(), Value::Map(data));
}

// ④ 'data' 맵 출력
fn print_data_map(ctx_map: &HashMap<String, Value>) {
    println!("\n키 'data'의 값:");
    if let Some(Value::Map(data_map)) = ctx_map.get("data") {
        for (key, value) in data_map {
            println!("    {}: {}", key, value);
        }
    }
}

// ⑤ JSON 예제 삽입
fn insert_json_example(ctx_map: &mut HashMap<String, Value>) {
    let mut json_obj = HashMap::new();

    json_obj.insert("name".to_string(), Value::Str("홍길동".to_string()));
    json_obj.insert("age".to_string(), Value::Int(30));
    json_obj.insert("is_member".to_string(), Value::Bool(true));
    json_obj.insert("tags".to_string(), Value::StrList(vec![
        "rust".to_string(),
        "json".to_string(),
        "enum".to_string(),
    ]));

    let mut address = HashMap::new();
    address.insert("city".to_string(), Value::Str("서울".to_string()));
    address.insert("zip".to_string(), Value::Int(12345));
    json_obj.insert("address".to_string(), Value::Map(address));

    ctx_map.insert("json_example".to_string(), Value::Map(json_obj));
}

// 실행 함수
fn run() {
    let mut ctx_map = init_ctx_map();
    print_map("① 초기 값:", &ctx_map);

    add_more_values(&mut ctx_map);
    print_map("② 값 추가 후:", &ctx_map);

    insert_data_map(&mut ctx_map);
    print_data_map(&ctx_map);

    insert_json_example(&mut ctx_map);
    print_map("⑤ JSON 예제 포함:", &ctx_map);
}

// 진입점
fn main() {
    run();
}
