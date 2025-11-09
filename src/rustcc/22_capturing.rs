// https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html

fn main () {

    // capture : closure가 외부변수를 기억하기 위해 붙잡는(borrow/move) 행위
    
        let color = String::from("green") ; 
    
        let print = || println!("color: {}", color) ; //borrowed
         // color라는애를 감싸서 다룸
         // Rust는 capture시 주의: move 등 ownership 문제 가능성
         // owner가 결정되는 지점이 불명확하면 컴파일 오류 생김
        
    
        print() ;
    
        let _reborrow = &color ;
        print() ;
    
        let color_moved = &color ;
        print() ; // compile error 
    
    
        let mut count = 0 ;
        let mut inc = || {
            // inc : local variable을 멤버로 가진 오브젝트같은걸 만든것
            // mut : count값을 갱신하는(감싼) 클로저이므로 mut 필요
            // mut (inc도 value이기 때문)
            
            count = count + 1 ; // count가 mut이므로 가능
            println!("count: {}", count) ; 
            // borrowed mutably
        } ;
    
        inc() ;
        inc() ;
    
        // 이미 closure inc 가 mut borrow를 했으므로 아래 코드는 전부 에러
        // let count_borrowed = &count ;
        // let count_borrowed = &mut &count ;
        // let count_moved = count ;
        
        inc() ;


    // Box : Box일ㄸ도 동일하게 동작한다는 것 (Box RV 필요)
        let mut b = Box::new(42) ;  
        // Box의 포인터값이 b에 들어감 & 메모리 잡힘
        let mut increase = || {
            *b = *b + 1 ;
            println!("b: {}", *b) ;
        } ;
    
        increase() ;
        increase() ;
    
        *b = 0 ;
        //increase() ;
        println!("b: {}", *b) ;


}
