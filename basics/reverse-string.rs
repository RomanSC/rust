// reverse-string.rs | Mon, Feb 20, 2017 | Roman S. Collins

use std::string::String;

// macro_rules!
// http://rustbyexample.com/macros.html
// This char_at function is not mine.
// https://gist.github.com/dwhinham/d7d4b3c541de2921489beab0cbd9fdef
macro_rules! char_at {
    ($str: ident, $index: ident) => {
        &$str[$index..$index + 1]
    }
}

// We change "mystring" to "yostring" to prevent ownership confusion
fn reverse_string(yostring: String) -> String {
    // .len() in Rust returns byte count for a given string
    // not character count.
    // http://brandonio21.com/2016/04/rust-you-probably-meant-to-use-chars-count/
    let string_length = &yostring.chars().count();

    let mut work_string: String = yostring.clone(); // Not sure if I need to use this yet

    //print!("{}", string_length);

    //if yostring.chars().count() <= 1 {
    if &string_length <= &&1usize {
        yostring // Return; base case.
    } else {
        //println!("DEBUG: Well it's not one");
        //yostring
        // let i: usize = 0;
        // return reverse_string(char_at!(i)) + reverse_string(char_at!(i));
        // Rust says "No."
        yostring
    }
}

fn main() {
    //let mut mystring: str = "thisismystring";
    let mut mystring: String = String::from("This is my string.");
    let mut one_char: String = String::from("s");

    //TODO: Take string from user

    /*  Using recursion to reverse the string:

        This algorithm in Python:

        def reverse(text):
            if len(text) <= 1:
                return text

            return reverse(text[1:]) + text[0]

        reverse(hello)
        reverse(ello) + h           # The recursive step
        reverse(llo) + e + h
        reverse(lo) + l + e + h
        reverse(o) + l + l + e + h  # Base case
        o + l + l + e + h
        olleh

    */

    println!("{}", reverse_string(one_char));
    println!("{}", reverse_string(mystring));

    // How char_at! macro works
    // for i in 0..mystring.chars().count() {
    //     println!("{}", char_at!(mystring, i));
    // }
}
