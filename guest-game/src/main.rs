use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    println!("Guest some number: ");
    let target_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        println!("You guessed: {}", number);
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&target_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed right! target {} number {}", target_number, number);
                break;
            }
        }
    }    
}
