use std::io;

fn main() {
    println!("Hello! Let's manipulate some numbers.");

    let first: f32 = get_number("first");
    let operation = get_operation();
    let second: f32 = get_number("second");

    make_operation(first, operation, second);
}

fn get_number(arg_name: &str) -> f32 {
    println!("Please input {} number.", arg_name);

    let readed_str: String = read();

    let first: f32 = parse(readed_str);

    println!("Your input: {}", first);

    first
}

fn get_operation() -> Operation {
    println!("Please input operation.");

    let readed_str: String = read();

    match readed_str.trim()  {
        "+" => Operation::Plus,
        "-" => Operation::Minus,
        "/" => Operation::Division,
        "*" => Operation::Multiplication,
        _ => panic!("Error")
    }
}

fn make_operation(first: f32, operation: Operation, second: f32) {
    let result: f32 = match operation {
        Operation::Multiplication => first * second,
        Operation::Division => first / second,
        Operation::Minus => first - second,
        Operation:: Plus => first + second
    };

    println!("Result of operation be {}", result);
}

fn read() -> String {
    let mut string: String = String::new();

    io::stdin().read_line(&mut string)
        .expect("Failed to read line");
    string
}

fn parse(string: String) -> f32 {
    string.trim().parse().unwrap()
}

enum Operation {
    Plus,
    Minus,
    Division,
    Multiplication
}