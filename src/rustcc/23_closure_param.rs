fn apply<F>(mut f: F) where F: FnMut () -> () {
    f() ;
} 
// FnMut, mut 을 붙여 주면 push_str 해도 잘 받음
// mut f:F >> f라는 클로저 객체를 받아 mutable하게 쓰겠다 (move)
// apply 를 여러 번 호출하려면 f: &mut F —> 참조만 빌려서 사용하면 됨

fn apply_twice<F>(mut f: F, n: i32) where F: FnMut(i32) -> i32 {
// FnMut(i32) -> i32 가 f라는 closure의 Type
// <F> 는 해당 타입을 제네릭으로 표현한 것
    f(n) ;
    f(n) ;
}


// f(){} == [f=(){}] 임을 생각하며 | | 이해

fn main () {
    let greeting = String::from("hello") ;
    let mut farewell = String::from("goodbye") ;

    let f = || { 
        // parameter 없음 : 간단히 그냥 값
        // 매개변수X 클로저는 정의에 위배?
        //     >> 클로저는 반만 바인딩된 값일수도 있고
        //     >> 모두 바인딩된 값일수도 있음 (| |)
        
        println!("I said {}", greeting) ;
        farewell.push_str("!!!") ;
        println!("Then I said {}", farewell) ;
    } ; // 클로저의 의의:: 이걸 여기서 실행하는 게 아니라*

    apply(f) ; 
    // *여기서 실행 가능! => 함수 실행을 원하는 시점에 가능하게 해줌
    //  apply(f), f(): 함수를 값으로 다루고, 그 값을 Expr 안에서 호출하는 
    //                 first class function 에서의 Call 표현
    
    // apply(f) ;  f가 move 되었으므로 2회 불가

    let mut count = 0 ;
    let inc = |n: i32| -> i32 { 
        // λn. {count = count+n} 
        // 이를 람다값이라고 하기도 하고, 람다 abstraction 이라고 하기도 함
        // count값, 그에대한 연산/인터페이스를 코팅해서 들고다니는게 inc:closure 
        // (count 직접접근 불가, 간접가능)
        
        // count를 캡처-22_capturing 참조
        count = count + n ;
        count 
    } ; // inc = count 업데이트하는 클로저

    apply_twice(inc, 5) ;
    println!("count: {}", count) ;
}
