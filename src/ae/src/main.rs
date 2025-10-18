
use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::{add, sub, num} ;
use ast::Expr::{Op, Num} ;
use ast::Opr::{Add, Sub} ;

lalrpop_mod!(pub ae) ;


fn interp (e: Box<Expr>) -> i32 
  // ####  여기서의 interp 가 Semantics 구현 (Evaluation)
  //       04의 15p 를 직접 구현해둔 블록, 중요함
  
  /*       (e: Box<Expr>) -> i32 :
           Box<Expr> 받고 (domain) i32로 보내주는(codomain) 함수
           >> e 자체는 reference 이기 때문에 *e가 e가 가리키는 값이 됨
  */
{
    match *e {
        Op(l, Add, r) => interp(l) + interp(r),
        Op(l, Sub, r) => interp(l) - interp(r),
        Num(n) => n
      
      // Op의 l(left), r(right) 는 Box (ast.rs 참조)
      // Add, Sub 매칭을 구분(+, -)  Op에 해당하는 모든 경우 확인(rustc)
    } 
}


fn main() 
{
// 이부분은 생각나는대로 적고, 모든 Op, Num 에다가 Box::new로 힙공간을 주면됨
// Box::new 만 빠뜨리지 않고 잘 적으면 웬만해서는 문제가 없음
    let e0 = Box::new( Op( Box::new( Op(Box::new( Num(5) ), 
                                     Sub, 
                                     Box::new( Num(1) )) 
                                    ), 
                           Add,
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
    println!("e0 AST: {:?}", e0) ;
     // ### AST 구조로 출력하기 출력내용 Op(Op(Num(5),Sub,Num(1)), Add, Num(3))
    println!("interp(e0): {}", interp(e0)) ;
    println!("") ;


  
    let e1 = add(sub(num(5), num(1)), num(3)) ;    
    // e1은 함수를 통해 가독성을 개선해준 꼴임
  
    println!("e1: {}", e1) ;
    println!("e1 AST: {:?}", e1) ;
    println!("interp(e1): {}", interp(e1)) ;
    println!("") ;

  
    let e2 = ae::ExprParser::new().parse("((5 - 1) + 3)").unwrap() ;
    // e2는 lalrpop parser generator 를 활용해 만든 파서에 직접 넣는것
    // new() : 파서 instance 만드는 것
    // lalrpop의 파서가 parse() 를 통해 작동하여 String 을 자동으로 파싱해줌
    //    * 문법이 틀리면? : return이 Result이므로 result.rs 예제와 깉이 error 출력
    //      unwrap() RV : Result에서 무조건 Ok 나온다고 가정하고 값을 까서 줌

  
    // String 을 파서가 바로 해체를 해주는거임
    // 위에있는건 String이 아니니깐 걍 쌩으로 AST를 주고 걔를 Semantics 함수들로 
    // evalutaion 한 결과만 뱉는거고 아래녀석은 파서가 해체하고 결과뱉는거
    // e2 만 lalrpop이 필요한거임. e0, e1은 lalrpop 파일이 없어도 실행가능(애초에 AST였으니까)
    // String을 AST형식으로 바꿔주는게 lalrpop 이라는 애가 하는일
  
    println!("e2: {}", e2) ;
    println!("e2 AST: {:?}", e2) ;
    println!("interp(e2): {}", interp(e2)) ;

  // ####  앞으로 많이 쓸 것은 e2 표기
  
}
