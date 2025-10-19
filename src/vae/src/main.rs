use std::collections::BTreeMap ;

use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Val, Use} ;
use ast::Opr::{Add, Sub} ;

lalrpop_mod!(pub vae) ;


// BTreeMap<K, V>는 본래 키-값 쌍을 저장하는 정렬된 맵
// 참조를 붙인 &BTreeMap<K, V> 는 실제 맵을 복사하지 않고 참조만 전달할 때 쓰는 표현
//     let mut map = BTreeMap::new(); 이런 식으로 정의하고,
//     map.insert("one".to_string(), 1); 이런 식으로 사용함

// 이러한 BTreeMap의 key-value 구조로 identifier 를 binding occurrence!
// String 은 값에 부여할 변수 이름이고, i32는 본래의 value

fn interp (e: Box<Expr>, env: &BTreeMap::<String, i32>) -> i32 
{
    // env 는 전제(σ)로, match *e시 variable의 evaluate에 직접관여(Use,Val)
    match *e {
        Op(l, Add, r) => interp(l, env) + interp(r, env),
        Op(l, Sub, r) => interp(l, env) - interp(r, env),
        Num(n) => n,
        Use(id) => *env.get(&id).unwrap(),
         // BTreeMap reference env 에서, value 를 가져오려면
        // *env.get(&key) 로 가져오되 Result type 처리로 unwrap() 붙여줘야 함
        Val(id, v, e) => {
            // Sematics: id-v 매핑후 expression 에 적용(전제) : evaluate
            let mut nenv = env.clone() ;
            nenv.insert(id, interp(v, env)) ;
            println!("nenv inside Val: {:?}", nenv) ; // 확인용 출력 추가
            interp(e, &nenv) // 재귀적으로 e를 다시 해석하도록 함
                        // block expression
        }
    } 
}


fn main() 
{
    // Main에서의 테스트 코드도 작성할 줄 알아야 함
    // BTreeMap 만드는 과정, parse 함수 작성 과정도 연습
    // 파싱하는거랑, helper 없이 하는거랑, helper 로 하는거 세가지 모두 익히기
    
    let env = BTreeMap::<String, i32>::new() ;
    let e0 = vae::ExprParser::new().parse("val i=3 in (i + (1 + i))").unwrap() ;
    println!("e0: {}", e0) ;
    println!("e0: {:?}", e0) ; // AST 출력 
                        // Val("i", Num(3), Op(Use("i"), Add, Op(Num(1), Add, Use("i"))))
    println!("interp(e0,[]): {}", interp(e0, &env)) ;

    let e1 = vae::ExprParser::new().parse("val i=3 in (i + val i=5 in (1 + i))").unwrap() ;
    println!("e1: {}", e1) ;
    println!("e1: {:?}", e1) ;
            // AST: Val("i", Num(3), Op(Use("i"), Add, Val("i", Num(5), Op(Num(1), Add, Use("i")))))
    println!("interp(e1,[]): {}", interp(e1, &env)) ;


    let e2 = vae::ExprParser::new().parse("val i=5 in (i + 10)").unwrap() ;
    println!("e2: {}", e2) ;
    println!("e2: {:?}", e2) ;
    println!("interp(e2,[]): {}", interp(e2, &env)) ;

     let e2 = vae::ExprParser::new().parse("val i=(0 + 5) in (i + 10)").unwrap() ;
    println!("e2: {}", e2) ;
    println!("e2: {:?}", e2) ;
    println!("interp(e2,[]): {}", interp(e2, &env)) ;
    // 간단한 case 추가, AST : Val("i", Op(Num(0), Add, Num(5)), Op(Use("i"), Add, Num(10)))
    	// 왜 두 번째 요소도 &Box여야 하는가?
	    //  i=(0+5) 같은 형식일 수 있기 때문! 

    
    // use helper function : val i=(1+10) in (i-1)
    // 중요! 그냥 val ... 이런식으로 주면 &str 으로, 파서에 보낸다면 파서는 알아서 처리하지만
    // helper 함수를 사용하는 경우, 그냥 ""로 적으면 String이 들어가야 할 자리에 &str이 들어가버린다.
    let e3 = val(String::from("i"), add(num(1),num(10)), sub(useid(String::from("i")),num(1))) ; 
    println!("e3: {}", e3) ;
    println!("e3: {:?}", e3) ;
    println!("interp(e3,[]): {}", interp(e3, &env)) ;
    
}
