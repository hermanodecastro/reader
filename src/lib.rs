//! This crate was inspired by Python's input function. It allows easy reading of data from the terminal.
//!
//! A simple example of its use is:
//! ```
//! extern crate reader;
//! use reader::input;
//!
//! let name = input("Enter your name: ");
//! println!("Your name is: {}", name);
//! ```

use std::io::Write;
use std::num::{ParseIntError, ParseFloatError};

/// Read a String.
///
/// This function works in a similar way to the python "input" function,
/// that receives as an argument a sentence that will be displayed on the screen
/// and returns a String.
///
/// # Examples
///
/// Basic usage
/// ```
/// extern crate reader;
/// use reader::input;
///
/// let name = input("Enter your name: ");
/// println!("Your name is: {}", name);
/// ```
pub fn input(text: &'static str) -> String {
	let mut string = String::new();

	print!("{}", text);
	std::io::stdout().flush().unwrap();

	std::io::stdin()
		.read_line(&mut string)
		.expect("Failed to read line"); 

	string.truncate(string.len() - 1); // Removes the '\n', important to avoid line skipping.
	
	return string;
}

/// Convert a String to integer (i64)
///
/// # Examples
///	
/// Basic usage
/// ```
/// extern crate reader;
/// use reader::{input, int};
///
/// Reading a integer (i64)
///
/// let age = int(input("Enter your age: ")).unwrap();
/// println!("Your age is: {}", age);
///
/// Reading a integer (i32)
///
/// let age = int(input("Enter your age: ")).unwrap() as i32;
/// println!("Your age is: {}", age);
///
/// Reading a integer (i16)
///
/// let age = int(input("Enter your age: ")).unwrap() as i16;
/// println!("Your age is: {}", age);
///
/// Reading a integer (i8)
///
/// let age = int(input("Enter your age: ")).unwrap() as i8;
/// println!("Your age is: {}", age);
/// ```
pub fn int(string: String) -> Result<i64, ParseIntError> {
	return match string.trim().parse::<i64>() {
		Ok(integer) => Ok(integer),
		Err(e) => Err(e)
	}
} 	

/// Convert a String to float (f64)
///
/// # Examples
///
/// Basic usage
/// ```
/// extern crate reader;
/// use reader::{input, float};
///
/// Reading a float (f64) 
///
/// let salary = float(input("Enter your salary: ")).unwrap();
/// println!("Your salary is: {}", salary);
///
/// Reading a float (f32)
///
/// let salary = float(input("Enter your salary: ")).unwrap() as f32;
/// println!("Your salary is: {}", salary);
/// ```
pub fn float(string: String) -> Result<f64, ParseFloatError> {
	return  match string.trim().parse::<f64>() {
		Ok(float) => Ok(float),
		Err(e) => Err(e)
	}
}


