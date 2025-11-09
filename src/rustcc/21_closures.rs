// https://doc.rust-lang.org/rust-by-example/fn/closures.html

fn main() {
    let x: i32 = 3;
    //let f = |v: i32| -> i32 {v + x} ;
    // ->는 있어도 되고, 없어도 됨

    let f = |v| v + x; // v의 바인딩미룬다 (parameter)
                       // f의 type :i32->i32 closure(=function)
                       //     function, closure 사실 넓게보면 동일하긴 함

    /* function = 코드(계산 방법)
    closure = 코드 + 그 코드가 참조하는 변수 환경(스코프) */

    println!("f(2): {}", f(2));

    let one = || 1;
    // first-class function 에서는 모든 값이 constant function=closure
    // closure 는 값보다 큰 superset

    println!("one: {}", one()); 

    /* 람다대수 = closure (λ) :계산방식
         모든 수는 람다(λ) : 값하나 받고 바꾸는것 (복수 바꾸기)
            ==> all 수학 opertion 표현가능
    */
}
