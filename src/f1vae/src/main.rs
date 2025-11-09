use std::collections::BTreeMap ;

use lalrpop_util::lalrpop_mod ;

pub mod ast ;
use ast::{Prog, Expr, Fun} ;
use ast::Expr::{Op, Num, Let, Use, Call} ;
use ast::Opr::{Add, Sub} ;

lalrpop_mod!(pub f1vae) ;

fn fundef (flist: Vec<Fun>) -> BTreeMap::<String, Fun> {
    let mut fenv = BTreeMap::<String, Fun>::new() ;
    for f in flist {
        let fid = f.0.clone() ;
        fenv.insert(fid, f) ;
    }
    fenv 
}


fn interp (e: Box<Expr>, env: &BTreeMap::<String, i32>, fenv: &BTreeMap::<String, Fun>) -> i32 
{
    match *e {
        Op(l, Add, r) => interp(l, env, fenv) + interp(r, env, fenv),
        Op(l, Sub, r) => interp(l, env, fenv) - interp(r, env, fenv),
        Num(n) => n,
        Use(id) => *env.get(&id).unwrap(),
        Let(id, v, e) => {
            let mut nenv = env.clone() ;
            nenv.insert(id, interp(v, env, fenv)) ;
            interp(e, &nenv, fenv)
        },
        Call(funid, arg) => {
            //let f = fenv.get(&funid).unwrap() ;
            let Fun(_, pid, fexpr) = fenv.get(&funid).unwrap() ;
            let argv : i32 = interp(arg, env, fenv) ;

            let mut nenv = BTreeMap::<String, i32>::new() ; // []  {}  
            nenv.insert(pid.clone(), argv) ; // [pid -> argv]
            interp(fexpr.clone(), &nenv, fenv) 
        },
    } 
}


fn main() 
{
    let p0 = f1vae::ProgParser::new().parse("fun twice(x)=(x+x) ; 
                                            fun inc(y)=(y+1) ; 
                                            let i=3 in (inc(i) + (1 + twice(i)))").unwrap() ;
    println!("{}", p0) ;
    println!("{:?}", p0) ;

    let Prog(flist0, e0) = *p0 ;
    
    let env = BTreeMap::<String, i32>::new() ;    
    let fenv = fundef(flist0) ;
    println!("interp(p0.expr,[],p0.funlist): {}", interp(e0, &env, &fenv)) ;
    
}
