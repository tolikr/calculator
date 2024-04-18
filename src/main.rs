use read_utils::reader;
use read_utils::ExpressionPart;
use read_utils::Operation;

mod read_utils;

fn main() {
    println!("Hello! Let's manipulate some numbers.");
    println!("Type an expression, for example 2+1+34 and hit Enter. Type `exit` to exit.");

    let mut continue_work: bool = true;

    while continue_work {
        let input = read_utils::read_string();
        let trimmed = input.trim();

        continue_work = !("exit" == trimmed);

        if continue_work {
            main_action(&input);
        }
    }
}

fn main_action(input: &str) {
    let expression: Vec<ExpressionPart> = loop_parse(&input, Vec::new());

    if expression.is_empty() {
        print!("Print something")
    } else {
        let mut operation: Operation = Operation::Plus;
        let mut result: Option<f32> = None;

        for d in expression {
            match d {
                ExpressionPart::Digit(d) =>
                    match result {
                        None => result = Some(d),
                        Some(r) => {
                            result = Some(make_operation(r, &operation, d));
                        }
                    },
                ExpressionPart::Operation(o) => operation = o
            }
        }

        match result {
            Some(r) => println!("Result is {}", r),
            None => println!("Result is undefined"),
        }
        
    }
}

fn loop_parse(input: &str, mut expression: Vec<ExpressionPart>) -> Vec<ExpressionPart> {
    if input.is_empty() {
        expression
    } else {
        let (expr, rest) = reader(&input);
        expression.push(expr);
        loop_parse(&rest, expression)
    }
}

fn make_operation(first: f32, operation: &Operation, second: f32) -> f32 {
    match operation {
        Operation::Multiplication => first * second,
        Operation::Division => first / second,
        Operation::Minus => first - second,
        Operation::Plus => first + second,
    }
}
