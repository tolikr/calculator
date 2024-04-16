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
    println!("Type an expression, for example 2+1+34 and hit Enter. Type `exit` to exit.");

    let mut continue_work: bool = true;

    while continue_work {
        let input = read();
        let trimmed = input.trim();

        continue_work = !("exit" == trimmed);

        if continue_work {
            main_action(&input);
        }
    }
}

fn main_action(input: &str) {
    let expression: Vec<Expression_Part> = loop_parse(&input, Vec::new());

    if expression.is_empty() {
        print!("Print something")
    } else {
        let mut operation: Operation = Operation::Plus;
        let mut result: Option<f32> = None;

        for d in expression {
            match d {
                Expression_Part::Digit(d) =>
                    match result {
                        None => result = Some(d),
                        Some(r) => {
                            result = Some(make_operation(r, &operation, d));
                        }
                    },
                Expression_Part::Operation(o) => operation = o
            }
        }

        match result {
            Some(r) => println!("Result is {}", r),
            None => println!("Result is undefined"),
        }
        
    }
}

fn loop_parse(input: &str, mut expression: Vec<Expression_Part>) -> Vec<Expression_Part> {
    if input.is_empty() {
        expression
    } else {
        let (expr, rest) = reader(&input);
        expression.push(expr);
        loop_parse(&rest, expression)
    }
}

fn reader(input: &str) -> (Expression_Part, String) {
    // TODO make it a component
    let digit = Regex::new(r"^\d+\s*").unwrap();
    // TODO make it a component
    let operation = Regex::new(r"^[+-/*]\s*").unwrap();

    digit
        .find(input)
        .map(|matched| {
            (
                Expression_Part::Digit(matched.as_str().trim().parse::<f32>().unwrap()),
                get_rest(&input, matched),
            )
        })
        .or(operation.find(input).map(|matched| {
            (
                Expression_Part::Operation(parse_operation(matched.as_str().trim())),
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

fn make_operation(first: f32, operation: &Operation, second: f32) -> f32 {
    match operation {
        Operation::Multiplication => first * second,
        Operation::Division => first / second,
        Operation::Minus => first - second,
        Operation::Plus => first + second,
    }
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

enum Expression_Part {
    Digit(f32),
    Operation(Operation),
}

impl std::fmt::Display for Expression_Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression_Part::Digit(d) => write!(f, "{}", d),
            Expression_Part::Operation(o) => write!(f, "{}", o),
        }
    }
}
