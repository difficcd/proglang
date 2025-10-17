#### ae_lalrpop 코드 이해
*md 파일이지만 가독성을 위해 주석 형식으로 작성
*다운로드 후 실행에 문제가 없도록 md 파일에 작성

<br>

#요약
pub Expr: Box<Expr> = {  
    <n:Num> => Box::new(Expr::Num(n)),  
    "(" <l:Expr> <op:ExprOp> <r:Expr> ")" => Box::new(Expr::Op(l, op, r)),  
    "(" <e:Expr> ")" => e,  
};  

<n:Num> 을 받았다 : Box::new(Expr::Num(n)) 로 반환  
"(" <l:Expr> <op:ExprOp> <r:Expr> ")" 을 받았다 :  Box::new(Expr::Op(l, op, r)) 로 반환  
"(" <e:Expr> ")" 를 받았다 : e로 반환  

이런식으로 해석하면 됨 (위에있는 e, Box::new... 다 Box<Expr>이다  
pub Expr: Box<Expr> 이것이 Expr이라는 이름을 가진 규칙은 Box<Expr>을 반환한다는 뜻임  



* Summary  
1 입력 문자열을 문법에 맞게 분해하고   
2 사용자가 작성한 Rust 코드(예: Box::new(Expr::Num(n)))를 실행해서  
3 AST를 만들어주는 역할이 LALRPOP역할  

<br>

```
// 사실 이 코드는 lalrpop library 가 이해하는 script code로 작성되었고 Rust code는 아님  
// lalrpop lib 이 해당 스크립트를 해석하여 문법을 파싱하는 Rust code를 만들어준다고 이해  


// [ 구조 ]  =========
// (1) valid String 을 위한 grammar   
// (2) 정의된 "grammar 를 인식하는 동시에 AST 코드를 생성하는 코드" 
// 각 "grammar Rule 이 발생할 때마다 어떤 AST를 어떻게 만들어야 하는지"가 프로그래밍 되어있음




use std::str::FromStr;
use crate::ast::{Expr, Opcode};

// Rust에서 crate = 현재 프로젝트의 루트
// use crate::ast::{Expr, Opr};
//  → ast.rs에 정의된 Expr과 Opr를 현재 파일에서 바로 사용
//  → 예: Box::new(Expr::Num(n)), Opr::Add

Rust 모듈 시스템에서 다른 파일의 타입, 함수, 구조체를 가져오는 표준 방식
grammar;


// pub : 외부 사용 허용
// 이 parsing 시작지점이 Expr 라는 뜻


pub Expr: Box<Expr> = {  // Expr 까지는 grammar의 세계, 이후는 Rust code  
                         // 아래는 화살표 전까지는 파서의 세계, 이후는 Rust code   
    "(" <l:Expr> <op:ExprOp> <r:Expr> ")" => Box::new(Expr::Op(l, op, r)),                  
    <n:Num> => Box::new(Expr::Num(n)),   
                // Expr ::= num | Expr Opcode Expr 을 떠올려보면 이해가 쉽다  
    
};
// Box::new : Box하나를 heap에다 잡고 값 저장후 포인터 넘겨줌 (malloc과 유사)  
// op:ExprOp (ExprOp를 op라고 두겠다는 뜻) 


ExprOp: Opcode = {  // ExprOp 까지는 grammar의 세계, 이후는 AST Rust code
    "+" => Opcode::Add,
    "-" => Opcode::Sub,ㅂ
    // Opcode ::= '+' | '-' 
};

Num: i32 = {  // Num 까지는 grammar의 세계, i32은 Rust code
    r"(-)?[0-9]+" => i32::from_str(<>).unwrap(),
    // 매칭이 되면 String 이 넘어가기에, i32::from_str() 메소드 사용 (atoi같은 느낌)
    // from_str(<>) 의 <> 는 매칭되어 넘어온 String
    // unwrap() 은 예외처리
}
```


*** 

```
grammar / Rust 영역을 나누어 보면 아래와 같음

use std::str::FromStr;
use crate::ast::{Expr, Opcode};

grammar;

// ====== grammar ======                               ====== Rust code ====== //
pub Expr:                                                Box<Expr> = { 
    "(" <l:Expr> <op:ExprOp> <r:Expr> ")"                => Box::new(Expr::Op(l, op, r)), 
    <n:Num>                                              => Box::new(Expr::Num(n)),
};

ExprOp:                                                  Opcode = {
    "+"                                                  => Opcode::Add,
    "-"                                                  => Opcode::Sub,
};

Num:                                                     i32 = {
    r"(-)?[0-9]+"                                        => i32::from_str(<>).unwrap(),
}  // integer 값이므로, i32로 구현 가능

// grammar 만 적어서 돌리면 문법 검사만 해주는 파서가 생김 (AST 생기지 않음)

```



