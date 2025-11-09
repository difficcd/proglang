// https://doc.rust-lang.org/rust-by-example/fn/closures/capture.html

fn main () {
    
    /*
    let color = String::from("green") ;

    let print = || println!("color: {}", color) ; //borrowed

    print() ;

    let _reborrow = &color ;
    print() ;

    let color_moved = color ;
    //print() ; // compile error 

    
    let mut count = 0 ;
    let mut inc = || {
        count = count + 1 ;
        println!("count: {}", count) ; // borrowed mutably
    } ;

    inc() ;
    inc() ;

    
    //let count_borrowed = &count ;
    let count_borrowed = &mut count ;
    //let count_moved = count ;
    inc() ;

    */
    
    let mut b = Box::new(42) ;
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
