use std::fmt ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opr, Box<Expr>),

    Val(String, Box<Expr>, Box<Expr>),
    Use(String),

	// Val("i", Num(5), Op(Use("i"), Add Num(10)))
	//      >> val i=5 in (i + 10)

	// Use 는 Num(3)으로 매핑된 변수 i를 사용한다는 뜻
	// :Use(id) => *env.get(&id).unwrap()

	Call(String, Box<Expr>),  //String(Expr) 꼴로 호출하기
	//LALRPOP >>    f(a: Expr) => Box::new(Expr::Call(String::from(f), a))

}

#[derive(Debug, Copy, Clone)]
pub enum Opr {
    Add,
    Sub,
}

// helper functions

pub fn add (l: Box<Expr>, r: Box<Expr>) -> Box<Expr> 
{ Box::new(Expr::Op(l, Opr::Add, r)) }

pub fn sub (l: Box<Expr>, r: Box<Expr>) -> Box<Expr>
{ Box::new(Expr::Op(l, Opr::Sub, r)) }

pub fn num (n: i32) -> Box<Expr>
{ Box::new(Expr::Num(n)) }



pub fn val (id: String, v: Box<Expr>, e: Box<Expr>) -> Box<Expr>
{ Box::new(Expr::Val(id, v, e)) }

pub fn useid (id: String) -> Box<Expr>
{ Box::new(Expr::Use(id)) }



impl fmt::Display for Expr 
{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Num(n) => write!(f, "{}", n),
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
			Expr::Val(x, id, e) => write!(f, "val {}={} in {}", x, id, e),
			Expr::Use(id) => write!(f, "{}", id),
			Expr::Call(fname, arg) => write!(f, "{}({})", fname, arg), 
			  // Call에 대한 Display trait 구현
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
