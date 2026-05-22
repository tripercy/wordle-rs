use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    let word = "guess";
    let mut guesses_left = 6;
    let mut win = false;
    let mut buffer = String::new();
    while guesses_left > 0 && !win {
        print!("{} guess(es) left: ", guesses_left);
        io::stdout().flush().expect("failed to flush stdout");
        buffer.clear();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to readline");
        let guess = buffer.trim();
        if word.cmp(guess) == Ordering::Equal {
            win = true;
        }
        guesses_left -= 1;
    }
    println!("{}", if win {"You won"} else {"You lost"});
}
