
//#1 Rust 의 first-class function 지원
fn call_one <F>(f: F) where F: Fn() -> () {
    f() ;
}

fn hello () {
    // Rust 에서 모든 fn 은 closure 처럼 사용가능
    println!("hello") ;
}

fn main () {
    call_one(hello) ;


    // c1, c2 는 각각 함수가 리턴한 closure를 값으로 가짐
    let mut c1 = create_counter() ;
    let mut c2 = create_counter() ;
    
    println!("c1: {}", c1()) ;
    println!("c1: {}", c1()) ;
    println!("c2: {}", c2()) ;
    println!("c1: {}", c1()) ;
    println!("c2: {}", c2()) ;
    // c1, c2 가 같은 클로저를 공유하지 않으므로
    // 독립적으로 증가 
    
}

//#2 closure 를 return 해주는 함수
fn create_counter () -> impl FnMut() -> i32 {
    // () -> (impl FnMut() -> i32)
    // impl FnMut() : 클로저 구체타입 숨기고 이 trait 만족 어떤값 반환
    // 쉽게 void 받아서 mutFun 으로서 i32 내놓는 closure return 
    //     * FnMut = 내부 상태를 바꾸는 closure 
    
    let mut cnt = 0 ;

    move || { 
        cnt = cnt + 1 ;
        cnt
    }
}
