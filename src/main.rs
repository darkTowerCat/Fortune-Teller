//Copyright © 2019 Terry Tower

use std::io;
//based on code from https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0

fn main() {
	let max_answer = 3;
	let mut input_text = String::new();

	println!("Thank you for visiting the fortune teller:\nPlease enter your lucky number");

	io::stdin()
		.read_line(&mut input_text)
		.expect("failed to read from stdin");

	let trimmed = input_text.trim();
	match trimmed.parse::<u32>() {
		Ok(number) => responce(number, max_answer),
			Err(..) => println!("{} is was not an integer", trimmed),
	};
}
fn responce(number:u32, max_answer:u32) {
	match number % max_answer {
		0 => println!("The road is rocky now, but will be worth it."),
		  1 => println!("Keep going you are almost there."),
		  2 => println!("Your future is bright."),
		  3 => println!("No matter how bad today is, tomorrow will come."),
		  _ => println!("Focus on the light at the end of the tunnel."),
	}
}
