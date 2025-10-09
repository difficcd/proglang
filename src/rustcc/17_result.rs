

use std::io ; 

fn main()
    {
        // EasyRust Chapter 32 Option&Result 참조
        //     Option : 정상종료 했는데 없는 경우 처리
        
        let mut input = String::new() ;
        
        io::stdin().read_line(&mut input).expect("read_line") ;
        // enter 까지 한 줄 입력 (얘도 리턴값이 Result, 대부분이 Result return함) 
        // Result 이므로, let~= 을 빼버리면 warning 이 나오게 된다
        // input reference mutable borrow

        // expect 는 read_line 이 Result 로서 Error 를 내지 않을 것임을 보장한다는 것
        // 즉, 예외 처리되지 않아도 되는 안전한 입력임을 알려주는 것임

        /*  let r0 : Result<usize, _> = io::stdin().read_line(&mut input) ;  */
        

        let r : Result<i32, _> = input.trim().parse() ;
        // Result는 i32 이거나 _(어떤 타입도 가능한) enum 타입이라는 뜻
        // 정확히는, 리턴값대로  _ → std::num::ParseIntError 로 추론됨

        let r : Result<i32, std::num::ParseIntError> = input.trim().parse() ;
        // Result는 i32 이거나 ParseIntError라는 에러값인 enum 타입
        // Exception 대신에, return 값을 특수 형태 enum인 Result 로 받음!

        // trim : tap, enter, space 를 제거해줌
        // parse : i32 로 받게 parsing 해달라는 것 (참고로 빌리는 것임)

        /* 중요한 점은, parse() 리턴값이 단순한 i32가 아니라 
        Exception 처리 포함했다고 볼 수 있는 꼴의 Result 출력값이라는 점으로 
        타입을 선언할 때 필수적으로 Result<> enum 으로 해주어야 한다 */


        println!("\n{:?}\n", r) ;
        // Ok(123)
        // digit 이 아닌 input : ParseIntError {kind: InvalidDigit}
        // OV : ParseIntError {kind: PosOverflow} (음수는 NegOverflow)

        match r {
            Ok(n) => println!("\n{}\n", n),
            Err(_) => println!("\nError\n"),
        }
        // 깔끔한 결과 출력은 match 사용하면 됨


        let mut input2 = String::new() ;
        io::stdin().read_line(&mut input2).expect("read_line");
        // enter 까지 한 줄 입력 (얘도 리턴값이 Result)

        let l : i32 = input2.trim().parse().unwrap() ;
        println!("\n{}\n", l);
        // unwrap() 은 i32 외의 입력이 들어오면 패닉(런타임 에러와 비슷)
        // 

    }
