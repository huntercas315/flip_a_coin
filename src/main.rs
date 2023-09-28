use clap::Parser;
use rand::prelude::{random, thread_rng};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of coins to flip
    #[arg(short, default_value_t = 1)]
    flip_count: u8,

    /// Switches to rolling dice
    #[arg(short, long, default_value_t = false)]
    dice: bool,

    /// Sets the number of sides the dice has
    #[arg(short, default_value_t = 6)]
    side_count: u8,

    /// Sets the number of dice in each roll
    #[arg(short, default_value_t = 1)]
    num_dice: u8,
}

fn main() {
    let args = Args::parse();

    if args.dice {
        for _ in 0..args.flip_count {
            roll(&args);
        }
    } else {
        for _ in 0..args.flip_count {
            flip();
        }
    }
}

fn roll(args: &Args) {
    if args.num_dice == 1 {
        let dice: u8 = thread_rng().gen_range(1..=args.side_count);
        println!("{dice}");
    } else {
        let mut dice: Vec<u8> = Vec::new();
        for _ in 0..args.num_dice {
            dice.push(thread_rng().gen_range(1..=args.side_count));
        }

        let mut output = String::new();
        let mut total = 0u8;
        for die in dice {
            output.push_str(format!("{die} ").as_str());
            total += die;
        }
        println!("{output}\n{total}");
    }
}

fn flip() {
    if random() {
        println!("Heads");
    } else {
        println!("Tails");
    }
}
