

fn main() {

    // Chapter 26 : Loops
        let mut counter = 0;
        loop {
            counter +=1; 
            println!("The counter is now: {}", counter);
            if counter == 5 { 
                break;
            }
        } println!();
    
        // label loop 에 대해서는 13_controlflow.rs 참고(rustcc)
    
        let mut counter = 0;
        while counter<5 {
            counter +=1 ;
            println!("counter : {}", counter);
        }println!();
    
        
        for num in 0..3{
            println!("{}", num);  // 0 1 2 
            // python 과 비슷하지만, 들여쓰기가 아닌 block 기준
        }println!(); 
    
        
        for num in 0..=3{
            println!("{}", num);  // 0 1 2 3 (<=)
        }println!(); 
    
        
        for _ in 0..3{
            println!("repeat : 3 times");
            // num 으로 해도 되지만, 그런 경우 warning 이 나옴
            // _변수명으로 하면 warning 안나옴 (선언과 동일)
        }println!();
    
    
        let mut counter = 5;
        let my_number = loop { // let 변수 = loop {} 도 가능!
            counter +=1;
            if counter % 53 == 3 {
                break counter;
            }
        };
        println!("{}", my_number); // 56

        // 튜플 활용 loop 예제
        let first = (200, 0, 0);
        let second = (50, 50, 50);
        let third = (200, 50, 0);
    
        match_colours(first);
        match_colours(second);
        match_colours(third);

}


fn match_colours(rbg: (i32, i32, i32)) {
    println!("{} red, {} blue, {} green:", rbg.0, rbg.1, rbg.2);
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")];
    
    let mut all_have_at_least_10 = true; 
    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false; 
            println!("Not much {}.", item.1)
        }
    }
    if all_have_at_least_10 { 
        println!("Each colour has at least 10.")
    }
    println!(); 
}



