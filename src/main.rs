use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");

    let answer = rand::thread_rng().gen_range(1, 101);

    loop {
        match do_round(answer) {
            Ok(result) => match result {
                Ordering::Less => println!("Too low"),
                Ordering::Greater => println!("Too high"),
                Ordering::Equal => {
                    println!("You got it!");
                    break;
                }
            },
            Err(msg) => {
                println!("{}", msg);
            },
        }
    }
}

fn do_round(answer: u32) -> Result<Ordering, &'static str> {
    println!("Please input your guess");
    print!(">");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    if let Err(_) = io::stdin().read_line(&mut guess) {
        return Err("Cannot read line");
    }

    if let Ok(guess) = guess.trim().parse::<u32>() {
        return Ok(guess.cmp(&answer));
    } else {
        return Err("Please input a number");
    }
}
