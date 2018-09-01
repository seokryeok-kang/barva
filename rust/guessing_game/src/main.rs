extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자가 나왔다!!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("자, 게임을 시작하지!");
    println!("숫자를 입력 해 봐.");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).ok().expect("입력 값 실패");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("숫자가 아니면 안 되지!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("작다!"),
            Ordering::Greater => println!("크다!"),
            Ordering::Equal   => {
                println!("쉿!");
                break;
            }
        }
    }
}
