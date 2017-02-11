// printing.rs | Sat, Feb 11, 2017 | Roman S. Collins
//
// Demonstrating some printing basics in Rust.

fn main () {
	println!("Welcome {}", 1);

	// Print abc
	println!("{a} {b} {c}",
		a = "a",
		b = "b",
		c = "c");

}
