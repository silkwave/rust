use std::collections::HashMap;

pub struct HexCharBase26Converter;

impl HexCharBase26Converter {
    // 0-9와 A-P까지의 문자 배열 (총 26개)
    const HEX_CHAR: &'static [char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 
        'K', 'L', 'M', 'N', 'O', 'P',
    ];

    // 숫자->문자 맵핑 (문자 -> 값 조회에 최적화된 맵)
    fn build_char_to_decimal_map() -> HashMap<char, usize> {
        let mut map = HashMap::new();
        for (i, &ch) in Self::HEX_CHAR.iter().enumerate() {
            map.insert(ch, i);
        }
        map
    }

    // 10진수를 26진수로 변환 (0-9, A-P까지)
    pub fn to_base26(mut number: usize) -> String {
        if number == 0 {
            return "0".to_string();  // 0일 경우 특별 처리
        }

        let mut result = String::new();
        while number > 0 {
            result.push(Self::HEX_CHAR[number % 26]); // HEX_CHAR 배열에서 문자 가져오기
            number /= 26;  // 다음 자리 계산
        }

        result.chars().rev().collect()  // 결과 문자열 뒤집기
    }

    // 26진수 문자를 10진수 값으로 변환
    pub fn to_decimal(base26: &str) -> Result<usize, String> {
        if base26.is_empty() {
            return Err("유효하지 않은 입력입니다.".to_string());
        }

        let char_to_decimal_map = Self::build_char_to_decimal_map();
        let mut result = 0;
        
        for c in base26.chars() {
            match char_to_decimal_map.get(&c) {
                Some(&value) => {
                    result = result * 26 + value; // 각 자리의 값을 계산
                }
                None => {
                    return Err(format!("유효하지 않은 문자입니다. 0-9, A-P만 가능합니다. 발견된 문자: {}", c));
                }
            }
        }
        Ok(result)
    }
}

fn main() {
    // 테스트: 10진수 -> 26진수
    let number = 456975;
    let base26 = HexCharBase26Converter::to_base26(number);
    println!("10진수 {} 의 26진수 표현: {}", number, base26);

    // 테스트: 26진수 -> 10진수
    let base26 = "PPPP";
    match HexCharBase26Converter::to_decimal(base26) {
        Ok(decimal) => {
            println!("26진수 {} 의 10진수 표현: {}", base26, decimal);
        }
        Err(e) => {
            println!("오류: {}", e);
        }
    }
}
