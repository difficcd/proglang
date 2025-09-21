
// CP 29
struct Item1 {
    number: u8,
}
impl Item1 {
    fn compare_number (&self, other_number: u8) { 
        print!(" {} == {} 2?  >>  ", self.number, other_number);
        println!("{}", self.number == other_number);
    }
}


#[derive(Debug)] //Display 는 불가함 (아주 기본 타입인거만 출력해주기에..)
struct Item2 {
  number: u8,
}
impl Item2 {
  fn compare_number (&self, other_number: u8) { 
      print!(" {} == {} 2?  >>  ", self.number, other_number);
      println!("{}", self.number == other_number);
  }
}

// CP 30
// {} 는 Display trait 필요하므로 제네릭 사용 불가
// {:?} 도 Debug trait 필요하므로 구현이 안된 제네릭에서는 사용불가
// trait : trait A + impl A for Item >> 해당 fn 필수 구현
//  *i32, i64 처럼 단순히 {} {:?} 사용가능한 애들은 이미 구현된 것
//   타입을 새로 만드는 제네릭에서는 따로 trait 구현이 필요함

use std::fmt::Debug; // 기본적으로는 위와 같지만 얠 쓰면 사용가능!!
use std::fmt::Display;

use std::cmp::PartialOrd; // 얘는 부분적 비교 지원해주는애 (>,<=, ...)
// i32 등 기본 숫자 타입은 PartialOrd 가능함. 제네릭 타입은 따로 구현..


fn return_number1(number: i32) -> i32 {
    println!("Here is your number : {:?}", number);
    number
}
fn return_number2<T: Display>(number: T) -> T { // <T: Display> 이런식으로 해야함
    println!("Here is your number : {}", number); //Thx to std::fmt::Display..
    number
}
fn return_number3<Mytype: Debug>(number: Mytype) -> Mytype {
    println!("Here is your number : {:?}", number); //Thx to std::fmt::Debug..
    number
}


fn main() {

  // Chapter 29 : Reference & dot operator
      let my_number = 9;
      let reference = &my_number; // &integer type
      println!("{}", &my_number == reference); //&빼면 type불일치
  
    
      let item1 = Item1 { // item 은 Item 객체(instance)
          number: 8,
      };
      let reference_number = &item1.number; 
      println!("{}", reference_number == &8); // type불일치
      println!("{}", item1.number);
      println!("{:?}", item1.number);
      // println!("{:?}",item1);  item1은 derive 안했기에 안됨 ###
  

      let item2 = Item2 { // item 은 Item 객체(instance)
          number: 8,
      };
      println!("{:?}", item2); // thx to derive(Debug)!!  ###
      // println!("{}", item2); // Display 는 기본타입만 지원, item2도 이건 불가능
       

      let reference_item = &item1;
      let reference_item_two = &reference_item; 
      item1.compare_number(8); 
      reference_item.compare_number(8); 
      reference_item_two.compare_number(8); 
      // Rust 의 dot operator 는 컴파일러가 내부적으로 
      // 타입이 메서드에 맞지 않아도 &, &mut, * 등으로 자동 역참조 시동
      // 따라서 위의 모든 호출에서도 에러가 안 남! (자동 역참조!)



  
  // Chapter 30 : Generics
      let n1 = return_number1(5); println!("{}", n1);
      let n2 = return_number2(3.14); println!("{}", n2);
      let n3 = return_number3(item2); println!("{:?}", n3.number);
      // return_num3 에다가 item1 을 넣으면 작동 안됨! (debug trait X)
  
      // C++ 의 Template 와 비슷 (그래서 Generic 인 것)
      // Rust 는 Generics + Trait 을 통해 OOP틱한 느낌을 줌
      // 단, 상속 기반 OOP는 불안정/예측불가 높이므로 채용하지 않음
      // Rust 는 Runtime 비용 없이 OOP 스타일 제공!

      // PartialOrd*
      compare_and_display1("Listen up!", 'a', 'b');
      compare_and_display1("Listen up!", "a", "b");
      compare_and_display1("Listen up!", 10.2, 0.01);
      // char 비교 : 유니코드 코드포인트로 비교. 'a' : 97, 'b' : 98
      // String 비교 : 사전순!!! 비교 / 바이트 단위u8로 UTF-8순서대로 비교

      compare_and_display2(10, 1, 2); // T U 라도 같을 수 있음 (where만 잘 따지기)
      // 단, T T 로 같은데 다른 Type 보내는 건 오류가 맞음!
  

}

fn compare_and_display1<T: Display, U: Display + PartialOrd>(statement: T, n1: U, n2: U) {
    println!("{}  {} > {}?  : {}", statement, n1, n2, n1 > n2);
    // U, U 끼리 부분적 크기 비교가 가능하다.
    // PartialOrd 가 알아서 맞춰서 비교해주는 것!
  // U 자리에 들어오는 게 숫자가 아니어도 알아서 비교해줌.
}

fn compare_and_display2<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
  // Using where is a good idea when you have many generic types.
  // where 을 사용하면, 더러웠던 선언이 굉장히 깔끔해짐! 
{
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

