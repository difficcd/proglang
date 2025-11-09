fn call_one <F>(f: F) where F: Fn() -> () {
    f() ;
}

fn hello () {
    println!("hello") ;
}

fn main () {
    call_one(hello) ;

    /*
    let mut c1 = create_counter() ;
    let mut c2 = create_counter() ;
    println!("c1: {}", c1()) ;
    println!("c1: {}", c1()) ;
    println!("c2: {}", c2()) ;
    println!("c1: {}", c1()) ;
    println!("c2: {}", c2()) ;
    */
}

fn create_counter () -> impl FnMut() -> i32 {
    let mut cnt = 0 ;

    move || { 
        cnt = cnt + 1 ;
        cnt
    }
}

