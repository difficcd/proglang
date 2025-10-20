

// 주석1
/* 주석2 */

// 관습으로 인해, 세미콜론은 한 칸 띄우고 작성

use std::mem::size_of;

fn number() -> i32 { // skinny arrow : 반환값을 i32 로 지정
    8 // no ";" is the value it returns
}

fn multiply(number_one: i32, number_two: i32) { // skinny arrow 는 반환값이 존재할 때만 사용
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
    
    // 반환값은 return result ;  대신 세미콜론 없이 반환값명만 적어준다. 
    // return result ; (X), reulst (o)
}


fn main() {
    
    let num = 100 ; 
    // 정수는 기본 i32 로 인식한다. (범용성)
    
    // i:8~128 (int, long.. 간소화)  isize : PC 에따라다름
    // u:8~128 (unsigned .. 간소화)  usize : PC 에따라다름0 
    // 다른 타입의 연산은 불가능하다.
    
    let x : i8 = 10 ; // 사용하지 않으면 unused warning
    let _x : i16 = 10 ; // unused warning 무시
    let _한글변수 : i16 = num + _x ; // 한글명 변수 사용가능
    
    let _casting = x + _x as i8 ; 
    //  casting = simple type change, as type 으로 쉽게 가능함
    
    
    // casting & char
    let _c = 'a' as u8 ; // C와 동일하게 ASCII 따름 (-> 97 나옴)
    
    // {} 쓰고 변수 안넣으면 컴파일러가 에러로 취급함
    println!("{}", _c) ; 
    println!("Hello, world!") ; 
    // string 은 무조건 "", char는 ''. C랑똑같음 UTF8 씀

    println!("char : {} bytes", size_of::<char>()) ;  // 4 byte
    
    println!("len of a  : {}", "a".len()) ; // len() == byte 수 (완전히 길이는 아님)
    println!("len of 國 : {}", "國".len()) ; // 복잡할 수록 필요 byte 수 증가 
    
    let slice = "國國國" ;
    println!("len of abcde {}", "abcde".len()) ; // 요소 증가 : byte 수 증가  
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    // 문자열의 len() 은, 다른 언어와 달리 .chars().count() 함수를 통해 얻을 수 있음 
    
    
    // Type inference 
    let num = 10u8 ;  // num:i8 = 10 과 동일
    let big_num = 100_000_000_i32 ;  // _는 가독성을 위해 사용가능
    println!("{}, {}", num, big_num) ;
    
    let my_float_1 = 5. ;
    let my_float_2 : f32 = 8.5; 
    println!("{}, {}", my_float_1, my_float_2) ;
    
    
    // Println! (!:macro = function that writes code)
    println!("Hello, world number {}!", number()) ; // 함수 호출 (당연히 main 위에 선언)
    
    
    // 여러 가지 변수 print 방식
    
    // 1 {별명}
    let my_city = "Seoul" ;
    println!( "The city is {city}", 
        city = my_city 
    ) ;
    
    // 2 numbering
    println!( "The city is {0}",  my_city ) ;
    
    // 3 {실제 이름}
    println!("The city is {my_city}") ; 
    
   
    // 함수 호출, 매개변수 보내기는 다른 코드들과 차이점이 없음
    multiply(8, 9) ; 
    let some_number = 10 ;
    let some_other_number = 2 ;
    multiply(some_number, some_other_number) ;
    
    let my_number = {
        let second_number = 8 ;
            second_number + 9 
            // 함수처럼, mynumber에는 17이 저장됨
    };

    print!("My number is: {}\n", my_number); 
    // print!, println! Java 와 유사함
    // 개행문자 \n 사용하면 ln 과 동일한 기능
    
    let my_number = {
        let second_number = 8 ; 
        second_number + 9 ; 
    };

    println!("My number is: {:?}", my_number) ; // my_number is ()
    // {:?} 은 빈 값도 받는 변수 자리
    // let doesnt_print = (); 꼴이면 {:?} 를 써야 한다.

    
}

