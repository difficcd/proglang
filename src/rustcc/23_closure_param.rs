fn apply<F>(f: F) where F: Fn () -> () {
    f() ;
}

fn apply_twice<F>(mut f: F, n: i32) where F: FnMut(i32) -> i32 {
    f(n) ;
    f(n) ;
}

fn main () {
    let greeting = String::from("hello") ;
    let mut farewell = String::from("goodbye") ;

    let f = || {
        println!("I said {}", greeting) ;
        //farewell.push_str("!!!") ;
        println!("Then I said {}", farewell) ;
    } ;

    apply(f) ;

    let mut count = 0 ;
    let inc = |n: i32| -> i32 { 
        count = count + n ;
        count 
    } ;

    apply_twice(inc, 5) ;
    println!("count: {}", count) ;
}
