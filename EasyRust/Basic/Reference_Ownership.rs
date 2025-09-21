


fn print_val(x: &i32) {
    println!("{}", x);
}


fn add_one(x: &mut i32) {
    *x += 1;
}



fn main(){

  // Reference 
    
      let mut x = 5;
      let r1 = &x;
      let r2 = &x; // 여러 불변 참조 가능
      // let r3 = &mut x; // 불변 참조가 있는 상태에서 가변 참조 불가!
    
    
      let x = 10;
      let r = &x;
      println!("{}", *r); // 10, *r로 원본 값 접근
  
      let mut x = 20;
      let r = &mut x;
      *r += 5;
      println!("{}", x); // 25
  
      let x = 10;
      print_val(&x); // &x → 불변 참조
  
      let mut x = 5;
      add_one(&mut x);
      println!("{}", x); // 6
      
  
    // Ownership
    
      let s1 = String::from("hello"); // s1이 owner
      let s2 = s1; // s1의 값이 s2로 move 됨, s1 더 이상 사용 불가
      // *** Primitive 타입(i32, bool 등)은 Copy trait : move가 아니라 복사
    
      println!("{}", s2); // ok
      // println!("{}", s1); // error


      // 참조는 소유권과 무관!
      // 가변이든 불변이든 소유권은 원본값이 가짐



}
