
// CP 31 : Option / Result 
// Result 는 proglang-src-rustcc-17_result.rs 에 정리해 두었으니 참고


fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 { 
        None
    } else {
        Some(value[4])
    }

    // Option 은 Result와 비슷한 Error 방지 목적의 타입
    // 조건적으로 출력 두 개 중에 하나를 택할 수 있다
    //          => None, Some(value)

    /* Option은 이렇게 생겼음
    enum Option<T> {
        None,
        Some(T),
    }
    */
}

fn handle_option(my_option: Vec<Option<i32>>) {
  for item in my_option {
    match item {
      Some(number) => println!("Found a {}!", number),
      None => println!("Found a None!"),
    }
  }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", take_fifth(new_vec.clone())) ;  // 
    println!("{:?}", take_fifth(bigger_vec.clone())) ;


     let mut option_vec = Vec::new();
    
    option_vec.push(take_fifth(new_vec.clone())); 
    option_vec.push(take_fifth(bigger_vec.clone())); 

    handle_option(option_vec);
    
}

