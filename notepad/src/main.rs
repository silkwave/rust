use aes::Aes128;
use base64::{engine::general_purpose::STANDARD, Engine};
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use rand::{Rng, thread_rng};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
const BLOCK_SIZE: usize = 16;

// 랜덤 IV (Initialization Vector) 생성 함수
fn generate_iv() -> [u8; BLOCK_SIZE] {
    // thread_rng()의 r#gen() 메서드를 사용해 16바이트 랜덤 생성
    thread_rng().r#gen()
}
/// AES-128-CBC 암호화 함수
/// key: 16바이트 키, plaintext: 암호화할 평문
/// 반환: Ok((Base64 IV, Base64 암호문)) 또는 Err(메시지)
fn encrypt(key: &[u8], plaintext: &str) -> Result<(String, String), String> {
    if key.len() != BLOCK_SIZE {
        return Err("키는 16바이트여야 합니다.".to_string());
    }
    let iv = generate_iv();
    let cipher = Aes128Cbc::new_from_slices(key, &iv)
        .map_err(|_| "암호화 초기화 실패".to_string())?;
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());
    Ok((
        STANDARD.encode(iv),
        STANDARD.encode(ciphertext),
    ))
}

/// AES-128-CBC 복호화 함수
/// key: 16바이트 키, iv_b64: Base64 IV, cipher_b64: Base64 암호문
/// 반환: Ok(복호화된 평문) 또는 Err(메시지)
fn decrypt(key: &[u8], iv_b64: &str, cipher_b64: &str) -> Result<String, String> {
    if key.len() != BLOCK_SIZE {
        return Err("키는 16바이트여야 합니다.".to_string());
    }
    let iv = STANDARD.decode(iv_b64).map_err(|_| "IV Base64 디코딩 실패".to_string())?;
    let ciphertext = STANDARD.decode(cipher_b64).map_err(|_| "암호문 Base64 디코딩 실패".to_string())?;
    let cipher = Aes128Cbc::new_from_slices(key, &iv)
        .map_err(|_| "복호화 초기화 실패".to_string())?;
    let decrypted = cipher.decrypt_vec(&ciphertext)
        .map_err(|_| "복호화 실패: 암호문 또는 키/IV 확인".to_string())?;
    String::from_utf8(decrypted).map_err(|_| "UTF-8 변환 실패".to_string())
}

fn main() {
    let key = b"0123456789abcdef";
    let plaintext = "안녕하세요 AES CBC PKCS7";

    // 암호화
    match encrypt(key, plaintext) {
        Ok((iv_b64, cipher_b64)) => {
            println!("IV (base64): {}", iv_b64);
            println!("Encrypted (base64): {}", cipher_b64);

            // 복호화
            match decrypt(key, &iv_b64, &cipher_b64) {
                Ok(decrypted) => println!("Decrypted: {}", decrypted),
                Err(e) => eprintln!("복호화 에러: {}", e),
            }
        }
        Err(e) => eprintln!("암호화 에러: {}", e),
    }
}
