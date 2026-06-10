use std::io;

fn main() {
    println!("=== Rust Calculator ===");
    let a = read_float("First number: ");
    let op = read_string("Operator (+, -, *, /): ");
    let b = read_float("Second number: ");
    let result = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                println!("Error: division by zero");
                return;
            }
            a / b
        }
        _ => {
            println!("Error: invalid operator");
            return;
        }
    };
    println!("Result: {}", result);
}

fn read_float(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

fn read_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
