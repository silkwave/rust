use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    let symbols = vec!["🍒", "🍋", "🔔", "⭐", "7️⃣"];
    let mut rng = thread_rng();

    println!("🎰 러스트 슬롯머신 게임 🎰");
    println!("Enter 키를 눌러서 슬롯을 돌리세요. (q 입력 시 종료)");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            println!("게임을 종료합니다!");
            break;
        }

        let slot1 = symbols.choose(&mut rng).unwrap();
        let slot2 = symbols.choose(&mut rng).unwrap();
        let slot3 = symbols.choose(&mut rng).unwrap();

        println!("-------------------------");
        println!("| {} | {} | {} |", slot1, slot2, slot3);
        println!("-------------------------");

        if slot1 == slot2 && slot2 == slot3 {
            println!("🎉 잭팟! 축하합니다! 🎉");
        } else if slot1 == slot2 || slot2 == slot3 || slot1 == slot3 {
            println!("✨ 두 개 일치! 아쉽지만 보너스!");
        } else {
            println!("😢 꽝! 다음 기회에...");
        }
    }
}
