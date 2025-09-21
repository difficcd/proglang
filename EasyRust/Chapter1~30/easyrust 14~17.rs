
use std::mem::size_of ;

// https://dhghomon.github.io/easy_rust/Chapter_18.html

fn main(){

//  Chapter 14 : Strings (String , &str)

    /* 1 ) &str : simple string. 불변 문자열 슬라이스(참조) 
              스택에 포인터 + 길이만 저장, immutable
              프로그램 실행 시 메모리의 읽기 전용(static) 영역에 저장
              매우 빠름. let = "~~" 는 기본적으로  &str */
  
    /* 2 ) String : complicated string. 소유권 있는 가변 문자열 타입
              비교적 느리지만 많은 기능.
              String이 heap에 데이터를 두고, 그걸 가리키는 녀석이라 포인터처럼 이해하면 됨
              실제; 포인터, 길이, capacity 스택에 담는 구조체, 실제 문자열 데이터는 heap에 존재
              문자열을 동적으로 만들거나 바꿀 수 있음 (push, insert, replace 등) */
  
  let _name = "서태지" ;
  let _longname = String::from("Adrian Fahrenheit Țepeș") ; // 기본&str -> String

  let _name = "😂 " ; 
  // 언어 차원에서 UTF-8을 기본 문자열 인코딩으로 채택, 특수기호 사용이 간편
  
  println!("\nMy name is actually {}", _name) ;
  println!("This is &str to string : {:?}", _longname) ;
  println!("String={:?}, bytes={:?}", size_of_val(_name), size_of::<String>()) ;


  // # Format!() function
  let my_name = "Billybrobby" ;
  let my_country = "USA" ;
  let my_home = "Korea" ;

  let together = format!(
      "I am {} and I come from {} but I live in {}.",
      my_name, my_country, my_home
  );
  println!("together : {:?}", together) ;

  
  // # into() function 
  // rust can make a &str into a lot of things. 따라서 String type 명시 필요
  let my_string: String = "Try to make this a String".into() ;
  println!("mystring : {:?}", my_string) ; 

  

  
//  Chapter 15 : const and static
    // let 뿐만 아니라 const, static 으로도 변수 선언 가능
  
    /* let : immutable default 지만, 진짜 변수라 메모리 주소가 존재. 
              Rust에서 shadowing은 let 변수만 가능*/
    /* const : 반드시 컴파일 타임에 값이 확정 & 타입 명시 (함수호출 값 할당 불가)
               값 자체가 코드에 박혀서 인라인됨 >> 실제 주소 갖지 않을 수도 있음
               define 상수처럼 사용하는 값을 지정할 때 사용 */
    /* static : 프로그램 전체 공유되는 전역 변수 (전역 메모리 저장, 데이터 영역에 위치) */
  
    const PI: f64 = 3.14159 ;
    let c = 2.0 * PI * 5.0 ;
    println!("{}", c) ; 

    static _GREETING: &str = "Hello, world!" ;
    static mut COUNTER: i32 = 0 ; 
    // unsafe 블록 (mutable static : 메모리 안전 보장 X)

    unsafe {
        COUNTER += 1 ;
        println!("{}", COUNTER) ;
    }

  
//  Chapter 16, 17 : reference +A >> Mutable references

    let country = String::from("Austria") ;
    let ref_one = &country ;  
    // ref_one type : &String, == reference to a String
  
    println!("{}", ref_one) ;
    let _country = return_str() ; 


    // reference : *, &
    // * is called "dereferencing". 
    /*Rust rules for mutable and immutable references

    Rule 1:  only immutable references, you can have as many as you want
    Rule 2:  mutable reference >> you can only have one.
    you can't have an immutable reference and a mutable reference together also.

    This is because mutable references "can change the data!"
    Rust 는 데이터에 대한 여러 곳의 간섭에 엄격한 경향이 강하다.
    problems if you change the data when other references are reading it 에 주의하기 때문.
    */
  
    let mut num1 = 8;
    let num_ref = &mut num1;
    *num_ref += 10; // Use * to change the i32 value.
    println!("reference applied : {}", num1); // reference 이므로 값이 반영됨
  
    let num2 = 800;
    let triple_ref= &&&num2;
    println!("Second_number = triple_ref? {:?}", num2 == ***triple_ref); // true
    println!("second_ref value is : {:?}", *triple_ref) ; 
  
  /*
  triple_ref : &&&i32
  *triple_ref : &&i32 → 참조의 참조
  **triple_ref : &i32 → num2를 가리키는 참조
  ***triple_ref : i32 → 실제 값
  */


  
  let mut number = 10;
  let number_ref = &number; // immutable ref : &T
  // let number_change = &mut number; // mutable ref!
  // *number_change += 10; 
  println!("{}", number_ref); 


  // 컴파일러가 변수 사용 범위를 기준으로 라이프타임을 종료로 간주
  
  let number_change = &mut number; 
  *number_change += 10;
  let number_ref = &number; // number_ref 재선언
  println!("{}", number_ref); 


  // Shadowing again 
  /* Rust에서 shadowing은 이름만 덮어쓰기
     실제 데이터가 사라짐(X) 컴파일러가 새 이름을 이전 이름 위에 붙인 것처럼 취급(O)
     country_ref는 여전히 첫 번째 String 값을 가리키는 참조로 유지될 수 있음*/
  
  let country = String::from("Austria"); 
  let country_ref = &country;
  let country = 8;
  println!("{}, {}", country_ref, country);
  

    
}

fn return_str() {
    let country = String::from("Austria") ; 
    let _country_ref = &country ; 
    //country_ref 
  
  // : country 가 스코프 벗어나며 메모리 해제됨
  // contry_ref 는 dangling reference. 
  
  /* country_ref는 country를 빌림(borrow)
     소유권 없음, 단순히 스택/힙 데이터를 가리키는 포인터
     참조 대상(country) 소유권이 함수 안에서만 있음 → 함수 종료 후 drop
     내부 생성 변수를 참조로 반환하는 건 소유권이 함수 밖으로 이동하지 않기 때문에 불가능

     Rust 에서는 함수 리턴값이라도 참조의 생명주기가 반환하는 쪽보다 짧으면 안 되고,
     참조가 항상 유효해야 함 (참조 대상이 살아 있어야 함)*/
}
