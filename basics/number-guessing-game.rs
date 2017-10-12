// number-guessing-game.rs | Sat, Feb 18, 2017 | Roman S. Collins
//
// Not my code. Following along to Rust Tutorial by Derek Banas:
// https://www.youtube.com/watch?v=U1EFgCNLDB8
//
// A number guessing game.
//

//use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

// Input module for accepting input
// from the user (keyboard)
use std::io::stdin;

fn main() {
    'outer: loop {
        // TODO: Make number random
        let number: i32 = 10;
        println!("Pick a number: ");

        loop {
            let mut line = String::new();

            // Some reference thingy: &mut
            // Get input
            let input = stdin().read_line(&mut line);

            let guess: Option<i32> =
            // |_| is or
            input.ok().map_or(None, |_|
            line.trim().parse().ok());

            match guess {
                None => println!("Enter a number!: "),
                Some(n) if n == number => {
                    println!("You guessed it!");
                    break 'outer;
                }

                Some (n) if n < number =>
                    println!("Too low"),
                Some (n) if n > number =>
                    println!("Too high"),
                Some(_) => println!("Error: Something went wrong.")
            }

        }
    }
}
