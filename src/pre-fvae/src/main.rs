use std::collections::BTreeMap ;
use std::fmt;

use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::Expr ;
use ast::Opr::{Add, Sub} ;
use ast::Expr::{Op, Num, Let, Use, Fun, App} ; 

lalrpop_mod!(pub vae) ;

// 값도메인(codomain) 확장: 𝑉 = ℤ ∪ 𝐼𝑑×𝐸×𝐸𝑛𝑣
// interp -> [i32 + Closure] 
#[derive(Debug, Clone)]
pub enum Value { 
    Num(i32),
    CloV(Closure),  // Closure = 𝐼𝑑×𝐸×𝐸𝑛𝑣
                    // Id: param, E: body, Env: env
}

#[derive(Debug, Clone)]
pub struct Closure {
    param: String,  // 매개변수
    body: Box<Expr>,  // 함수의 body
    env: BTreeMap<String, Value>, // 생성 시점의 환경
}

// 값(Value)을 출력하기 위한 Display trait 구현
impl fmt::Display for Value {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{}", n),
            Value::CloV(_) => write!(f, "<function>"),
        }
    }
}

// interp 함수 - codomain: Value >> 𝑉 = ℤ ∪ 𝐼𝑑×𝐸×𝐸𝑛𝑣  확장
//               𝐸𝑛𝑣 = 𝐼𝑑 ⇀ "𝑉": &BTreeMap::<String, "Value">

fn interp (e: Box<Expr>, env: &BTreeMap::<String, Value>) -> Value
{
    match *e {
        Op(l, Add, r) => {
            // for n1 ∈ ℤ and n2 ∈ ℤ 
            // 현재 Expr은 Closure, 연산 불가능 가능성 존재:  ℤ 판별 추가

            match (interp(l, env), interp(r, env)) {
                (Value::Num(vl), Value::Num(vr)) => Value::Num(vl + vr),
                _ => panic!("Type error: Add operands must be numbers"),
            }
        },
        Op(l, Sub, r) => {
            match (interp(l, env), interp(r, env)) {
                (Value::Num(vl), Value::Num(vr)) => Value::Num(vl - vr),
                _ => panic!("Type error: Sub operands must be numbers"),
            }
        },
        Num(n) => Value::Num(n), // i32 대신 Value::Num 래핑
        Use(id) => env.get(&id).unwrap().clone(), // 환경에서 Value를 클론
        Let(id, v, e) => {
            let mut nenv = env.clone() ;
            let val = interp(v, env); // 값을 먼저 계산
            nenv.insert(id, val) ; // Value를 환경에 삽입
            interp(e, &nenv)
        }

        // 람다 추상화: 클로저 생성
        Fun(param, body) => {
            // 클로저 생성: 현재 환경(env)을 캡처합니다.
            Value::CloV(Closure {
                param: param,
                body: body,
                env: env.clone(),
            })
        }
        
        // 함수 적용(호출)
        App(f, arg) => {
            let f_val = interp(f, env); // 함수 부분을 평가
            let arg_val = interp(arg, env); // 인자 부분을 평가

            match f_val {
                Value::CloV(closure) => {
                    // 1. 클로저의 캡처된 환경으로 시작
                    let mut fun_env = closure.env.clone();

                    // 2. 파라미터를 인자 값으로 바인딩
                    fun_env.insert(closure.param, arg_val);

                    // 3. 함수 본체를 확장된 환경에서 평가
                    interp(closure.body, &fun_env)
                }
                _ => panic!("Type error: Tried to apply a non-function value"),
            }
        }
    } 
}


fn main() 
{

    let env = BTreeMap::<String, Value>::new() ;
    

    // let add = 𝜆y.𝜆x.(x + y)
    // let add5 = (add 5)
    // (add5 3)  
    
    // => 8 

    let e_str = "let add = 𝜆x.(𝜆y.(x + y)) in (let add5 = (add 5) in (add5 3))";
    let e = vae::ExprParser::new().parse(e_str).unwrap();
    println!("\nTest 1 (Curried Lambda): {}", e);
    println!("Result: {}", interp(e, &env)); // 8
    
    // let f = 𝜆y.𝜆x.(y + x) in ((f 1) 2) ==> 결과 3
    let e2_str = "let f = (𝜆y.(𝜆x.(y + x))) in ((f 1) 2)";
    let e2 = vae::ExprParser::new().parse(e2_str).unwrap();
    println!("\nTest 2 (User Example): {}", e2);
    println!("Result: {}", interp(e2, &env)); // 3

}