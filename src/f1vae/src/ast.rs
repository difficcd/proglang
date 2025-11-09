use std::fmt ;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Op(Box<Expr>, Opr, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Use(String),
    Call(String, Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub enum Opr {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub struct Prog(pub Vec<Fun>, pub Box<Expr>) ;

#[derive(Debug, Clone)]
pub struct Fun(pub String, pub String, pub Box<Expr>) ;

impl fmt::Display for Fun {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fun(funid, param, expr) => write!(f, "fun {}({})={}", funid, param, expr) ,
        }
    }

}

impl fmt::Display for Expr {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Num(n) => write!(f, "{}", n),
			Expr::Op(l, op, r) => write!(f, "({} {} {})", l, op, r),
			Expr::Let(x, id, e) => write!(f, "let {}={} in {}", x, id, e),
			Expr::Use(id) => write!(f, "{}", id),
                        Expr::Call(funid, arg) => write!(f, "{}({})", funid, arg),
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

impl fmt::Display for Prog {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prog(flist, expr) => {
                for fun in flist {
                    write!(f, "{} \n", fun).expect(" ") ;
                }
                write!(f, "{}", expr) 
            },
        }
    }
}


