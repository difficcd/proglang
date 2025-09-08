fn main () {
    let v : i32 = 2025 ;
    let r = &v ;

    println!("v: {}, r: {}", v, r) ;

    
    println!("v: {}, r: {:p}", v, r) ;
    let p : *const u32 = &v ;
    

    
    let mut v : i32 = 2026 ;
    let p = &mut v ;
    *p = 2024 ;
    println!("p: {}", p) ;
   
    println!("v: {}", v) ;
    

}

// memo
fn main () {
    let v : i32 = 2025 ;
    let r = &v ;

    println!("v: {}, r: {}", v, r) ;

    println!("v: {}, r: {:p}", v, r) ;
    let p : *const i32 = &v ; 
    // unsig X (type 일치해야함)

    let mut v : i32 = 2026 ;
    let p = &mut v ;
    *p = 2024 ;

    println!("p: {}", p) ; // 2024

    println!("v: {}", v) ; // 2024


}
