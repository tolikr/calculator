use core::slice;
use regex::Match;
use regex::Regex;
use std::{
    io::{self, Read},
    thread::sleep,
    time,
};

fn main() {
    println!("Hello! Let's manipulate some numbers.");

    let mut continue_work: bool = true;

    while continue_work {
        println!("Type an expression, for example 2+1 and hit Enter: ");
        println!("Type `exit` to exit.");
        let input: String = read();

        continue_work = !"exit".eq_ignore_ascii_case(&input);

        println!("Result {} input {}", continue_work, input);

        if (continue_work) {
            let (first, rest) = parse_digit(&input);
            let (operation, rest2) = parse_operation(&rest);
            let (second, rest3) = parse_digit(&rest2);

            make_operation(first, operation, second);
        }
    }
}

fn parse_digit(input: &str) -> (f32, String) {
    // TODO make it a component
    let digit = Regex::new(r"\s*\d+\s*").unwrap();

    let matched = digit.find(&input).unwrap();
    let res = matched.as_str().trim().parse::<f32>().unwrap();

    (res, get_rest(input, matched))
}

fn parse_operation(input: &str) -> (Operation, String) {
    // TODO make it a component
    let operation = Regex::new(r"[+-/*]").unwrap();

    let matched = operation.find(&input).unwrap();
    let res = matched.as_str();

    let oper = match res {
        "+" => Operation::Plus,
        "-" => Operation::Minus,
        "/" => Operation::Division,
        "*" => Operation::Multiplication,
        _ => panic!("Unsupported operation."),
    };

    (oper, get_rest(input, matched))
}

fn get_rest(input: &str, matched: Match<'_>) -> String {
    let (_, rest) = input.split_at(matched.end());

    rest.to_string()
}

fn make_operation(first: f32, operation: Operation, second: f32) {
    let result: f32 = match operation {
        Operation::Multiplication => first * second,
        Operation::Division => first / second,
        Operation::Minus => first - second,
        Operation::Plus => first + second,
    };

    println!("Result of operation be {}", result);
}

fn read() -> String {
    let mut string: String = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    string
}

enum Operation {
    Plus,
    Minus,
    Division,
    Multiplication,
}
