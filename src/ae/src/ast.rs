use std::fmt ;

#[derive(Debug, Clone)]   
// Expr을 Debug/Clone trait 선언
// Debug:dbg, Clone:clone 쉽게 사용 (easyrust 29-30 참고)
// 직접 구현하지 않아도 Rust 가 알아서 enum 구조 파악하고 작성해줌
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    // Box<Expr> : recursive 

    /* 그냥 Op(Expr, Opcode, Expr)로 정의해버리면 
       Expr 의 크기결정 불가능! (가능한 PL도 있기는 함)
       이런 이유로 Rust의 heap을 사용하는 pointer격 개념 필요
       malloc : Box, 메모리 잡아주면서 포인팅 가능

       Box는 결국 reference이므로 recursive 구조 정의 가능하다!

       ex)  struct a {int n;  struct a next;}; (X)
            struct a {int n;  struct a* next;}; (O) 
                      **포인터는 가리킬 뿐이니 크기결정 가능 */
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Add,
    Sub,
}

// Expr ::= num | (Expr + Expr) | (Expr - Expr) 
// num ::= [0-9]+                *Reg ex 로 정의
// Opcode ::= '+' | '-'

// 이런 문법을 파싱해서 AST 만들어줌 (이렇게 정의된 object들의 집합)
// RV : Rust-enum 은 | 의 역할을 해줄 수 있음
// Opcode Add, Sub (constant) 




//pub type ExprBox = Box<Expr> ;

pub fn add (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> 
{
    Box::new(Expr::Op(l, Opcode::Add, r))
}

pub fn sub (l: Box<Expr>, r: Box<Expr>) -> Box<Expr>
{
    Box::new(Expr::Op(l, Opcode::Sub, r))
}

pub fn num (n: i32) -> Box<Expr>
{
    Box::new(Expr::Num(n))
}

// fmt=format
// use std::fmt::Display 를 떠올려 보면 이해하기 쉽다.

impl fmt::Display for Expr  
    // Expr Display 함수구현
{
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r)
        }
    }
}

impl fmt::Display for Opcode 
    // Opcode Display 함수구현
{
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-")
        }
    }
}
