use std::fmt ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opr, Box<Expr>),
    Neg(Box<Expr>), 
	// -(1+2) 같은 표현이 들어갈 수 있기에
	// Neg 안의 요소는 Box<Expr> 레퍼런스이어야 함
}

#[derive(Debug, Copy, Clone)]
pub enum Opr {
    Add,
    Sub,
}

//pub type ExprBox = Box<Expr> ;


pub fn add (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Op(l, Opr::Add, r))
}

pub fn sub (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Op(l, Opr::Sub, r))
}

pub fn num (n: i32) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

pub fn neg (e: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Neg(e)) 
	// Box<Expr> -> Box<Expr> 임에 주의하라 것
}


impl fmt::Display for Expr {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Num(n) => write!(f, "{}", n),
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Expr::Neg(e) => write!(f, "-({})", e)
		}
	}
}

impl fmt::Display for Opr {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Opr::Add => write!(f, "+"),
			Opr::Sub => write!(f, "-")
		}
	}
}

