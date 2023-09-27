use rand::prelude::random;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t = 1)]
    flip_count: u8,
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let args = Args::parse();

    for _ in 0..args.flip_count {
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