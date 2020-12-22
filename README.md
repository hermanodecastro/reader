# Reader
![GitHub](https://img.shields.io/github/license/hermanodecastro/reader)
> A simple python-inspired terminal data reader.

## Examples
### A simple example of its use is:
``` 
extern crate reader;
use reader::input;

let name = input("Enter your name: ");
println!("Your name is: {}", name);
```

### Convert a String to Integer:
```
extern crate reader;
use reader::{input, int};

// Reading a integer (i64)
let age = int(input("Enter your age: ")).unwrap();
println!("Your age is: {}", age);

// Reading a integer (i32)
let age = int(input("Enter your age: ")).unwrap() as i32;
println!("Your age is: {}", age);

// Reading a integer (i16)
let age = int(input("Enter your age: ")).unwrap() as i16;
println!("Your age is: {}", age);

// Reading a integer (i8)
let age = int(input("Enter your age: ")).unwrap() as i8;
println!("Your age is: {}", age);
```

### Convert a String to Float:
```
extern crate reader;
use reader::{input, float};

// Reading a float (f64) 
let salary = float(input("Enter your salary: ")).unwrap();
println!("Your salary is: {}", salary);

// Reading a float (f32)
let salary = float(input("Enter your salary: ")).unwrap() as f32;
println!("Your salary is: {}", salary);
```

