// rust-tutorial.rs | Sat, Feb 18, 2017 | Roman S. Collins
//
// Me following along to a Rust tutorial, basically just
// learning syntax
//
// Thanks Derek Banas for the video guide:
// https://www.youtube.com/watch?v=U1EFgCNLDB8
//

// include/importing data types, not sure this is even necessary...
// guide said so
use std::{i8, i16, i32, i64, u8, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn variables() {
    println!("\nVariables: ");

    println!("Hello world! ");

    // All variables are immutable
    // by default, meaning,
    // cannot be changed:
    let num = 10;

    // Making a muttable value;
    // changeable value:
    let mut age: i32 = 20;
    // i32 data type for the variable
    // i32 = 32bit integer

    // boolean:
    // Create true/false variable
    // named "is_it_true" of the
    // type "bool" and set to true
    let is_it_true: bool = true;

    // char:
    // Create "let_x" as
    // char data type, set
    // to x
    let let_x: char = 'x';

    // Assigning multiple variables at once
    // Like Python:
    // f_name, l_name = 'Roman', 'Collins'
    let (f_name, l_name) = ("Roman", "Collins");

    println!("Hello, my name is {} {}. I'm {} years old.",
             f_name, l_name, age);
}

fn stringformatting() {
    println!("\nStringFormatting: ");
    let is_it_true: bool = true;
    let let_x: char = 'x';

    // Indexes to format strings
    println!("It is {0} that {1} is {0}", is_it_true, let_x);
    // Output: It is true that x is true

    // Formatting floats
    println!("{:.2}", 1.234);
    // Output: 1.23

    // Binary, hexadecimal, and octal data
    println!("Binary: {:b} Hexadecimal: {:x} Octal: {:o}", 10, 10, 10);
    // Output: Binary: 1010 Hexadecimal: a Octal: 12

    // Named arguments/whitespace
    println!("{ten:>ws$}", ten=10, ws=5);
    // Output:    10

    println!("{ten:>0ws$}", ten=10, ws=5);
    // Output: 00010
}

fn expressions() {
    println!("\nExpressions: ");

    // Addition
    println!("5 + 4 = {}", (5 + 4));
    // Output: 5 + 4 = 9

    // Subtraction
    println!("5 - 4 = {}", (5 - 4));
    // Output: 5 - 4 = 9

    // Multiplication
    println!("5 * 4 = {}", (5 * 4));
    // Output: 5 * 4 = 9

    // Division
    println!("5 / 4 = {}", (5 / 4));
    // Output: 5 / 4 = 9

    // Remainder
    println!("5 % 4 = {}", (5 % 4));
    // Output: 5 % 4 = 9
}

fn maths() {
    println!("\nMaths: ");
    // Create a muttable integer -4 32bit
    // Don't understand these integer
    // types yet, but interesting...
    let mut neg_4 = -4i32;

    // Absolute value function .abs()
    println!("Absolute value of {} = {}", neg_4, neg_4.abs());

    // 6 == 6i32
    println!("{} ^ {} = {}", 4, 6i32, 4i32.pow(6));
    println!("Square root of {} = {}", 9, 9f64.sqrt());
    println!("Cube root of {} = {}", 9, 9f64.cbrt());
    // Round up if >= 5:
    println!("Round of 1.45 = {}", 1.45f64.round());
    // Always round down; floor.
    println!("Floor of 1.45 = {}", 1.45f64.floor());
    // Always round up; ceiling.
    println!("Ceiling of 1.45 = {}", 1.45f64.ceil());
    // Exponent? Hmm..
    println!("e ^ 2 = {}", 2f64.exp());
    println!("log(2) = {}", 2f64.ln());
    println!("log10(2) = {}", 2f64.log10());
    println!("90 to Radians = {}", 90f64.to_radians());
    println!("Pi to Degrees = {}", 3.14f64.to_degrees());
    // Min/Max value:
    println!("Max 4, 5 = {}", 4f64.max(5f64));
    println!("Min 4, 5 = {}", 4f64.min(5f64));

    println!("Sin 3.14 = {}", 3.14f64.sin());
}

fn some_conditionals() {
    // This guide reminds me of codecademy...
    let age_old = 20;

    if (age_old == 5) {
        println!("Go to kindergarten!");
    } else if (age_old > 5) && (age_old <= 18) {
        println!("Go to grade {}", (age_old - 5));
    } else if (age_old > 18) {
        println!("Go to college!");
    } else {
        println!("I don't know, do what you want...");
    }

    // False
    println!("!true = {}", !true);
    // True or False
    println!("true || false = {}", true || false);

    // True not equal to false
    println!("true != false: {}", (true != false));

    // "turnary operator" no built in
    let can_vote = if (age_old >= 18) {true} else {false};

    println!("Can vote: {}", can_vote);
}

fn looping(){
    let mut x = 1;

    // loop until break
    loop {
        // check if x is even
        if ((x % 2) == 0){
            println!("{}", x);
            x += 1;

            // beginning of loop
            continue;
        }

        // exit condition
        // Because fuck it this
        // is a low level language
        // mwuhahahahahah
        if (x > 10000000) {
            break;
        }

        x += 1;
        continue;
    }

}

fn main() {
    //variables()
    //stringformatting()
    //expressions()
    //maths()
    //some_conditionals()
    looping()

}
