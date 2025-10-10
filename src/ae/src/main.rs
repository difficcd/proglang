
use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::{add, sub, num} ;
use ast::Expr::{Op, Num} ;
use ast::Opcode::{Add, Sub} ;

lalrpop_mod!(pub ae) ;

fn interp (e: Box<Expr>) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l) + interp(r),
        Op(l, Sub, r) => interp(l) - interp(r),
        Num(n) => n
    } 
}


fn main() 
{
    let e0 = Box::new( Op( Box::new( Op(Box::new( Num(5) ), 
                                     Sub, 
                                     Box::new( Num(1) )) 
                                    ), 
                          // leftchild : Op(left:Num(5),sub,Num(1))
                          
                           Add, // == Op
                           Box::new( Num(3) )) 
                          // rightchild : Exprssion으로서 Num(3)
                          //           heap 에다 올려놓고 주소를 줌
        
  /* e0는 Op(Add)라는 값의 Box
     e0는 (5 Sub 1) + 3 이라는 일종의 트리 형태를 가리키는 reference 
     heap 에다가 직접 잡아야 해서 Box::new가 각각 필요하다 */

  // ast.rs 에 add, sub, num helper 함수 있음
  // 가독성을 위해 helper함수를 사용하면 아래와 같음 
                      ) ;
  
    println!("e0: {}", e0) ;
    println!("interp(e0): {}", interp(e0)) ;
    println!("") ;


  
    let e1 = add(sub(num(5), num(1)), num(3)) ;    
    // e1은 ast.rs를 끌어다 와서 간단히 표기한 것, 가독성 개선
  
    println!("e1: {}", e1) ;
    println!("interp(e1): {}", interp(e1)) ;
    println!("") ;

  
    let e2 = ae::ExprParser::new().parse("((5 - 1) + 3)").unwrap() ;
    // e2는 lalrpop parser generator 를 활용해 만든 파서에 직접 넣는것
    // new() : 파서 instance 만드는 것
    // lalrpop의 파서가 parse() 를 통해 작동하여 String 을 자동으로 파싱해줌
    //    * 문법이 틀리면? : return이 Result이므로 result.rs 예제와 깉이 error 출력
    //      unwrap() RV : Result에서 무조건 Ok 나온다고 가정하고 값을 까서 줌
  
    println!("e2: {}", e2) ;
    println!("interp(e2): {}", interp(e2)) ;

  // ####  앞으로 많이 쓸 것은 e2 표기
  // ####  여기서의 interp 가 Semantics 구현
  
}
