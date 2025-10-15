use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Neg} ;
use ast::Opr::{Add, Sub} ;

lalrpop_mod!(pub ae) ;

fn interp (e: Box<Expr>) -> i32 
  // Negation 에 대한 Semantics도 정의하긴 함
{
    match *e {
        Op(l, Add, r) => interp(l) + interp(r),
        Op(l, Sub, r) => interp(l) - interp(r),
        Neg(exp) => -1 * interp(exp), // 여기서 Semantics 정의
        // 사실 아래의 desugaring 과 Semantics 정의 둘 중 하나만 하면 됨
      
        Num(n) => n,
    } 
}

fn desugar (e: Box<Expr>) -> Box<Expr>
  // Desugaring method
  // (e: Box<Expr>) -> Box<Expr> : AST 를 AST 로 넘겨주는 것
  //                               (AST 수준에서 바꾸는 것)
  
{
    match *e {
        Op(l, Add, r) => Box::new(Op(desugar(l), Add, desugar(r))),
        Op(l, Sub, r) => Box::new(Op(desugar(l), Sub, desugar(r))),
        Neg(e) => Box::new(Op(Box::new(Num(0)), Sub, desugar(e))),
                  //   "-" "(" <expr> ")"   to   "0“ - ”(" <expr> ")“
        Num(n) => Box::new(Num(n)), // 소유권 문제 없이 copy됨 (premitive)

      // 값 자체를 바꿀 수는 없으므로 (immutable) node 정의를 새로 해주는 것
      // 추가한 AST요소는 Neg(e)! 얘를 위해서 다른애들을 다 desugar로 감싸주는 것
    }
}


fn main() 
{   
  
    let e0 = ae::ExprParser::new().parse("(-(5 - 1) + 3)").unwrap() ;
    println!("e0: {}", e0) ;
    println!("e0 AST: {:?}", e0) ; // AST 출력 추가  
                                    //  Op(Neg(Op(Num(5), Sub, Num(1))), Add, Num(3))
    println!("interp(e0): {}", interp(e0.clone())) ; // borrow를 위해 clone
    println!("") ;

    let e1 = desugar(e0.clone()) ;
    println!("e1=desugar(e0): {}", e1) ;
    println!("e1 AST: {:?}", e1) ; // AST 출력 추가 
                                  //  Op(Op(Num(0), Sub, Op(Num(5), Sub, Num(1))), Add, Num(3))
                                  // desugaring에 의해 AST->AST 변환된 것을 확인할 수 있다!
    println!("interp(e1): {}", interp(e1)) ;
  
}
