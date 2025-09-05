
use std::io::{self, Write};

// 1️ AST 정의
#[derive(Debug)]
enum Expr {
    Number(i64),
    Binary(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// 2️ Lexer: 단순히 스페이스로 분리
fn tokenize(input: &str) -> Vec<&str> {
    input.trim().split_whitespace().collect()
}

// 3️ Parser: 매우 단순, 후위 표기법 같은 간단한 가정
fn parse(tokens: &[&str]) -> Expr {
    let mut iter = tokens.iter();
    let left = iter.next().unwrap().parse::<i64>().unwrap();
    let mut expr = Expr::Number(left);

    while let Some(op_str) = iter.next() {
        let op = match *op_str {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => panic!("Unknown operator"),
        };
        let right = iter.next().unwrap().parse::<i64>().unwrap();
        expr = Expr::Binary(Box::new(expr), op, Box::new(Expr::Number(right)));
    }

    expr
}

// 4️ Evaluator
fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary(left, op, right) => {
            let l = eval(left);
            let r = eval(right);
            match op {
                Operator::Add => l + r,
                Operator::Subtract => l - r,
                Operator::Multiply => l * r,
                Operator::Divide => l / r,
            }
        }
    }
}

// 5️ 간단 REPL
fn main() {
    println!("Rust 간단 인터프리터 (종료: Ctrl+C)");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }

        let tokens = tokenize(&input);
        let ast = parse(&tokens);
        let result = eval(&ast);

        println!("{}", result);
    }
}
