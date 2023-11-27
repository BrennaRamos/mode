use chrono::*;
use colored::Colorize;
use rand::Rng;
use std::io;

fn main() {
    let mut level = 1;
    let mut rng = rand::thread_rng();

    while level < 100 {
        let mut green = 0;
        let mut red = 0;
        let mut input = String::new();
        let now = Local::now();

        println!("Level: {}\n", level);

        for _iter in 0..10 {
            let (color, _print_amt) = print_shapes(rng.gen_range(0..10), rng.gen_range(0..10));

            if color == 'g' {
                green += _print_amt;
            } else {
                red += _print_amt;
            }
        }

        println!();
        println!("Which color has more tallies? g/r: ");
        // MAKE A 5 SECOND TIMER
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {}
            Err(error) => println!("error: {error}"),
        }

        if process_guess(input, green, red) {
            level += 1;
        } else {
            println!("Game Over");
            return;
        }
    }

    println!("You Win!");
}

fn print_shapes(random: i32, print_amt: i32) -> (char, i32) {
    if random % 2 == 0 {
        for _iter in 0..print_amt {
            print!("{}", "x ".green());
        }
        return ('g', print_amt);
    } else {
        for _iter in 0..print_amt {
            print!("{}", "x ".red());
        }
        return ('r', print_amt);
    }
}

fn process_guess(guess: String, green: i32, red: i32) -> bool {
    if guess.trim() == "g" && green > red {
        println!("{} Correct!\n", "✅".green());
        return true;
    } else if guess.trim() == "r" && red > green {
        println!("{} Correct!\n", "✅".green());
        return true;
    } else {
        println!("{} Incorrect!\n", "❌".red());
        return false;
    }
}
