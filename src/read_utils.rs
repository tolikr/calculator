use std::io;

use regex::{Match, Regex};

pub fn read_string() -> String {
    let mut string: String = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    string
}

pub fn reader(input: &str) -> (ExpressionPart, String) {
    // TODO make it a component
    let digit = Regex::new(r"^\d+\s*").unwrap();
    // TODO make it a component
    let operation = Regex::new(r"^[+-/*]\s*").unwrap();

    digit
        .find(input)
        .map(|matched| {
            (
                ExpressionPart::Digit(matched.as_str().trim().parse::<f32>().unwrap()),
                get_rest(&input, matched),
            )
        })
        .or(operation.find(input).map(|matched| {
            (
                ExpressionPart::Operation(parse_operation(matched.as_str().trim())),
                get_rest(&input, matched),
            )
        }))
        .unwrap()
}

fn parse_operation(str: &str) -> Operation {
    match str {
        "+" => Operation::Plus,
        "-" => Operation::Minus,
        "/" => Operation::Division,
        "*" => Operation::Multiplication,
        _ => panic!("Unsupported operation."),
    }
}

fn get_rest(input: &str, matched: Match<'_>) -> String {
    let (_, rest) = input.split_at(matched.end());

    rest.to_string()
}

pub enum Operation {
    Plus,
    Minus,
    Division,
    Multiplication,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Plus => write!(f, "+"),
            Operation::Minus => write!(f, "-"),
            Operation::Division => write!(f, "/"),
            Operation::Multiplication => write!(f, "*"),
        }
    }
}

pub enum ExpressionPart {
    Digit(f32),
    Operation(Operation),
}

impl std::fmt::Display for ExpressionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionPart::Digit(d) => write!(f, "{}", d),
            ExpressionPart::Operation(o) => write!(f, "{}", o),
        }
    }
}