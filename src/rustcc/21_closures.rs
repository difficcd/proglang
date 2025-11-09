// https://doc.rust-lang.org/rust-by-example/fn/closures.html

fn main () {
    let x : i32 = 3 ;
    //let f = |v: i32| -> i32 {v + x} ;
    let f = |v| v + x ;

    println!("f(2): {}", f(2)) ;

    let one = || 1 ;

    println!("one: {}", one()) ;

}
