use std::fmt ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opr, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Use(String),

    Fun(String, Box<Expr>), // fun x -> body
    App(Box<Expr>, Box<Expr>), // (f arg)

}

#[derive(Debug, Copy, Clone)]
pub enum Opr {
    Add,
    Sub,
}

// Helper 함수들은 동일

impl fmt::Display for Expr 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Num(n) => write!(f, "{}", n),
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
			Expr::Let(x, id, e) => write!(f, "let {} = {} in {}", x, id, e),
			Expr::Use(id) => write!(f, "{}", id),

			Expr::Fun(param, body) => write!(f, "(fun {} = {})", param, body),
			Expr::App(func, arg) => write!(f, "({} {})", func, arg),
		}
	}
}

impl fmt::Display for Opr 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Opr::Add => write!(f, "+"),
			Opr::Sub => write!(f, "-")
		}
	}
}