

fn main() {
  let a : i32 = 42 ;
  
  let mut b1 : Box<i32> = Box::new(a) ;  // i32, move(x) copy(o)
  let mut b2 : Box<i32> = Box::new(a) ; 
  // b 는 Box type (**주 특징 : mutable)
  /* !! 단, 간접적인 접근을 해주는 reference b가 mut 인것과 
        Box<> 가 mut 인 건 완전히 다른 문제 
        Box<> 는 본래 mutable 한 type 이므로 mut 이 필요없음  */
  
  // Box<i32> 는 i32 가지는 heap 공간에 대한 reference
  // b : Box 는 쉽게 보면 b 를 가리키는 reference
  
  
  println!("{a}") ;      // println + : {a}로 쓸 수 있음
  println!("{}", b1) ;    // {b}도 가능 
  
  // b 라는 애는 사실 포인터인데, Debug 출력포맷이 아니면 reference

  *b1 = *b1 + 1 ;
  println!("{b1}") ;
  // b가 가리키는 값 update (mut 없으면 오류)
  
  let c : Box<i32> = b1 ;  
  // 여기서 Box의 owner가 move됨 (reference 소유권 move)
  // println!("{b1}") ;       Box 에 대한 소유권이 없어 재사용 불가
  println!("{c}") ; 

  // 위와 같은 상황에서 b를 계속 사용하려면? 
  // Box를 b를 통해 사용하도록 함 
  //     : Box heap공간 주소가 아니라 b2를 통해서 Box접근 가능

  
  let c : &mut Box<i32> = &mut b2 ;  
  // println!("{b2}") ;
  println!("{c}") ; 
  
  // b1, b2은 Box<i32> type 의 variable = heap 에 대한 reference 
  // c는 그 reference 에 대한 reference

  **c = 63 ;      
  println!("{c}") ;

  // b2, Box<i32> 에 mut을 붙여줘야만 c가 본래값을 바꿀 수 있음
  // c 자체의 값을 바꾸는 것이 아니므로, c는 mut 필요 없다 
  //    (가리키는 ref를 바꾸는 것이 아니기 때문. dereference value 를 바꿀 뿐)
  

}



