use std::collections::HashMap;
use std::fmt;

// `map!` 매크로: HashMap<String, Value> 생성을 단순화합니다.
// 예: map!{ "key1": "value1", "key2": 123 }
#[macro_export]
macro_rules! map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key.to_string(), $value.into());
            )*
            map
        }
    };
}

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

// From 트레잇 구현: 다양한 타입에서 Value로 쉽게 변환할 수 있도록 합니다.
// 이를 통해 `.into()` 메서드를 사용하여 코드를 간결하게 만들 수 있습니다.
impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Str(s.to_string())
    }
}

impl From<Vec<i32>> for Value {
    fn from(v: Vec<i32>) -> Self {
        Value::IntList(v)
    }
}

impl From<Vec<String>> for Value {
    fn from(v: Vec<String>) -> Self {
        Value::StrList(v)
    }
}

// &str의 벡터를 String의 벡터로 변환하여 Value로 만듭니다.
impl From<Vec<&str>> for Value {
    fn from(v: Vec<&str>) -> Self {
        Value::StrList(v.into_iter().map(String::from).collect())
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(m: HashMap<String, Value>) -> Self {
        Value::Map(m)
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
    // map! 매크로를 사용하여 간결하게 맵 생성
    map!{
        "A" => vec![1, 2, 3],
        "B" => "Hello",
        "C" => 123
    }
}

// ② 값 추가
fn add_more_values(ctx_map: &mut HashMap<String, Value>) {
    ctx_map.insert("D".to_string(), true.into());
    ctx_map.insert("E".to_string(), vec!["Apple", "Banana", "Cherry"].into());
}

// ③ data 맵 생성 및 ctx_map에 삽입
fn insert_data_map(ctx_map: &mut HashMap<String, Value>) {
    let data = map!{
        "a1" => "a1",
        "a2" => 22,
        "a3" => vec![1, 2, 3]
    };
    ctx_map.insert("data".to_string(), data.into());
}

// ⑤ JSON 예제 삽입
fn insert_json_example(ctx_map: &mut HashMap<String, Value>) {
    let json_obj = map!{
        "name" => "홍길동",
        "age" => 30,
        "is_member" => true,
        "tags" => vec!["rust", "json", "enum"],
        "address" => map!{
            "city" => "서울",
            "zip" => 12345
        }
    };
    ctx_map.insert("json_example".to_string(), json_obj.into());
}

// 실행 함수
fn run() {
    let mut ctx_map = init_ctx_map();
    print_map("① 초기 값:", &ctx_map);

    add_more_values(&mut ctx_map);
    print_map("② 값 추가 후:", &ctx_map);

    insert_data_map(&mut ctx_map);
    // ④ 'data' 맵 출력: 별도 함수 대신 직접 값을 가져와 출력
    println!("\n④ 키 'data'의 값:");
    if let Some(data_value) = ctx_map.get("data") {
        println!("{}", data_value);
    }

    insert_json_example(&mut ctx_map);
    print_map("⑤ JSON 예제 포함:", &ctx_map);
}

// 진입점
fn main() {
    run();
}
