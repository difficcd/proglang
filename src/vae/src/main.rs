use std::collections::BTreeMap ;

use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Ref, Val} ;
use ast::Opcode::{Add, Sub} ;

lalrpop_mod!(pub vae) ;


// BTreeMap<K, V>는 본래 키-값 쌍을 저장하는 정렬된 맵
// 참조를 붙인 &BTreeMap<K, V> 는 실제 맵을 복사하지 않고 참조만 전달할 때 쓰는 표현
//     let mut map = BTreeMap::new(); 이런 식으로 정의하고,
//     map.insert("one".to_string(), 1); 이런 식으로 사용함

// 이러한 BTreeMap의 key-value 구조로 identifier 를 binding occurrence!
// String 은 값에 부여할 변수 이름이고, i32는 본래의 value

fn interp (e: Box<Expr>, env: &BTreeMap::<String, i32>) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l, env) + interp(r, env),
        Op(l, Sub, r) => interp(l, env) - interp(r, env),
        Num(n) => n,
        Ref(id) => *env.get(&id).unwrap(),
        // id 형태면 값을 가져오도록 env 에 역참조시킴
        Val(id, v, e) => {
        // 새 변수 선언하고, nenv(env복사본:scope구분위해) 에 바인딩
            let mut nenv = env.clone() ;
            nenv.insert(id, interp(v, env)) ;
            interp(e, &nenv) // 재귀적으로 e를 다시 해석하도록 함
        }
    } 
}


fn main() 
{
    let env = BTreeMap::<String, i32>::new() ;
    
    // val 이라는 keyword 는 lalrpop 에 저장되어 있음!
    let e = vae::ExprParser::new().parse("val i=3 in (i + (1 + i))").unwrap() ;
    println!("e: {}", e) ;
    println!("interp(e,env): {}", interp(e, &env)) ;

    let e = vae::ExprParser::new().parse("val i=3 in (i + val i=5 in (1 + i))").unwrap() ;
    println!("e: {}", e) ;
    println!("interp(e,env): {}", interp(e, &env)) ;
}
