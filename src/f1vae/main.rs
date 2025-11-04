
use std::collections::BTreeMap ;


use lalrpop_util::lalrpop_mod ;

type Env = BTreeMap<String, i32>;
type FEnv = BTreeMap<String, (String, Box<Expr>)>; 
  // 함수이름 ↦ (매개변수이름, 함수본문)

pub mod ast ;
use ast::Expr ;
use ast::Expr::{Op, Num, Let, Use, Call} ;
use ast::Opr ;
use ast::Opr::{Add, Sub} ;
use ast::{add, sub, num, let_, useid} ;

lalrpop_mod!(pub vae) ;

fn interp (e: Box<Expr>, env: &Env, fenv: &FEnv) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l, env, fenv) + interp(r, env, fenv),
        Op(l, Sub, r) => interp(l, env, fenv) - interp(r, env, fenv),
        Num(n) => n,

        // BTreeMap reference env 에서, value 를 가져오려면
        // *env.get(&key) 로 가져오되 Result type 처리로 unwrap() 붙여줘야 함

        Use(id) => *env.get(&id).unwrap(),
        Let(id, v, e) => {
            // Sematics: id-v 매핑후 expression 에 적용(전제) : evaluate
            let mut nenv = env.clone() ;
            nenv.insert(id, interp(v, env, fenv)) ;
            
            println!("nenv inside Val: {:?}", nenv) ;
            interp(e, &nenv, fenv)
            // block expression
        }

        //fname(arg_expr)
        Call(fname, arg_expr) => {
            let (param, body) = fenv.get(&fname).unwrap(); // 함수 정의 가져오기
            let arg_val = interp(arg_expr, env, fenv); //  σ,Λ Ͱ arg_expr (x(e)의 e해석)

            // 새로운 환경 σ[param -> arg_val]
            let mut nenv = env.clone();
            nenv.insert(param.clone(), arg_val);
            //param에다가 arg_expr을 evaluate 한 값을 넣은 환경으로 갱신

            // 본문 평가
            interp(body.clone(), &nenv, fenv)
        }
    } 
}


fn main() 
{
    // Main에서의 테스트 코드도 작성할 줄 알아야 함
    // BTreeMap 만드는 과정, parse 함수 작성 과정도 연습
    // 파싱하는거랑, helper 없이 하는거랑, helper 로 하는거 세가지 모두 익히기

    let env = Env::new();
    let mut fenv = FEnv::new();

    let e1 = vae::ExprParser::new().parse("let i=3 in (i + let i=5 in (1 + i))").unwrap() ;
    println!("e1: {}", e1) ;
    println!("e1: {:?}", e1) ;
    println!("interp(e1,[]): {}", interp(e1, &env, &fenv)) ;
        println!() ;

    let e2 = vae::ExprParser::new().parse("let i=(0 + 5) in (i + 10)").unwrap() ;
    println!("e2: {}", e2) ;
    println!("e2: {:?}", e2) ;
    println!("interp(e2,[]): {}", interp(e2, &env, &fenv)) ;
        println!() ;

	// 왜 두 번째 요소도 &Box여야 하는가?
	//  i=(0+5) 같은 형식일 수 있기 때문! 

    
    // use helper function : let i=(1+10) in (i-1)
    // 중요! 그냥 let ... 이런식으로 주면 &str 으로, 파서에 보낸다면 파서는 알아서 처리하지만
    // helper 함수를 사용하는 경우, 그냥 ""로 적으면 String이 들어가야 할 자리에 &str이 들어가버린다.
    let e3 = let_(String::from("i"), add(num(1),num(10)), sub(useid(String::from("i")),num(1))) ; 
    println!("e3: {}", e3) ;
    println!("e3: {:?}", e3) ;
    println!("interp(e3,[]): {}", interp(e3, &env, &fenv)) ;
        println!() ;



    fenv.insert(
    String::from("f"), // fname
        (String::from("x"), Box::new(Expr::Op( // function definition
            Box::new(Expr::Use(String::from("x"))),
            Opr::Sub,
            Box::new(Expr::Num(30)),   // Λx.x-30 이름이 있으니, f(x)=x-30
        ))) 
    );
    // 위의 구문에서 f, 함수정의만 떼서 lalrpop한테 알아서 fenv 에 넣는건 왜 안되나?
    // fun 이라는 토큰을 만나면 대응되는 AST 노드를 만들 자리가 없기 때문! 

    /*
    문법 분석(parser) 과 평가(evaluation) 는 단계가 다름. 
        파서(parser) : “문자열 → AST(구조체)”
        인터프리터(evaluator) : “AST → 값(Env/Fenv 반영)”
    따라서 파서는 fenv에 직접 접근하면 안 됨(해석기의 역할)
    파서는 그저 AST를 만드는 역할만 담당해야 함
    */

    let e = vae::ExprParser::new().parse("let i=(0 + 5) in (i + f(10))").unwrap();
                                        // 5 + (-20)  => -15
    println!("Result: {}", interp(e, &env, &fenv));

}
