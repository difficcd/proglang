fn main () {
    let name = "서태지" ; // This is a Korean name. No problem, because a &str is UTF-8.
    let other_name = String::from("Adrian Fahrenheit Țepeș") ; // Ț and ș are no problem in UTF-8.

    println!("{:?} {:?}", name, other_name) ; // 디버깅 출력 포맷

    let together = format!("1: {}, 2:{}", name, other_name) ;  // format! 매크로 사용 예제
    println!("{}", together) ; 

    println!("{:?} {:?}", name, other_name) ;
    dbg!(name, other_name) ; 
    //debug bang* : 출력 후 값 자체를 반환 → 코드 안에서 계속 사용할 수 있음
    // println!보다 편리: 타입을 몰라도 바로 출력 가능, {}나 {:?}를 고민할 필요 없음

    println!("{} {} {}", name, std::mem::size_of_val(name), std::mem::size_of_val("서  태  지")) ;
    // println!("{} {}", std::mem::size_of_val(other_name), std::mem::size_of_val("Adrian Fahrenheit Țepeș")) ;
    // println!("{} {}", other_name.len(), std::mem::size_of_val("Adrian Fahrenheit Țepeș")) ;

}
