// 카운터 구조체 정의
struct Counter {
    count: u32,
}

// 카운터 구조체의 구현
impl Counter {
    // 새로운 카운터를 생성하는 연관 함수
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 카운터 구조체를 위한 이터레이터 구현
impl Iterator for Counter {
    // 이터레이터가 반환할 타입 지정
    type Item = u32;

    // next 메소드 구현
    fn next(&mut self) -> Option<Self::Item> {
        // 5보다 작은 동안 계속 반복
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None // 5가 되면 반복 중지
        }
    }
}

fn main() {
    println!("--- 1부터 5까지 출력 ---");
    // 새로운 카운터 생성
    let counter = Counter::new();
    // for 루프를 사용하여 이터레이터 소비
    for i in counter {
        println!("i = {}", i);
    }

    // 다른 이터레이터 메소드 사용 예제
    println!("\n--- 이터레이터 어댑터 활용 ---");
    let sum: u32 = Counter::new().sum();
    println!("합계 (1~5): {}", sum);

    let doubled: Vec<u32> = Counter::new().map(|n| n * 2).collect();
    println!("각 숫자를 2배한 값: {:?}", doubled);
}