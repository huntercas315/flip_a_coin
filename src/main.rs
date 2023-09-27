use rand::prelude::random;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let count: i32 = args[1].parse().unwrap();

        for _ in 0..count {
            flip();
        }
    } else {
        flip();
    }
}

fn flip() {
    let coin: bool = random();
    if coin {
        println!("Heads");
    } else {
        println!("Tails");
    }
}