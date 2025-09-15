

fn main() {

// Chapter 22 : Tuples
    // Tuples in Rust use (). We have seen many empty tuples already : main()
    // non-return function : you actually return an empty tuple, ()
    
    let tuple = ("str", 0, vec!['a'], 'b', [1,2], 0.1);
    println!("\n{:?}", tuple.1+1); // prints 1 (0+1)

    let str_vec = vec!["one", "two", "three"];
    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{:?}",(a,b,c)); // prints two

    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{:?}",variable);
    // let variable = str_vec[2]; 대신 사용하는 이유 
    //    >> 앞의 값은 무시하고 세 번째 값만 쓴다는 의도를 명확히 한 것

    
// Chapter 23 : Control flow
    // Control flow means telling your code what to do in different situations

  // (1) if 
    let my_number = 5 ;
    if my_number % 2 == 1 && my_number > 0 { 
        println!("It's a positive odd number");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }

    
  // match(case..)
    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
        // _ 가 필수적으로 들어가야 함 (예외처리)
    }

    let my_number = 5;
    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,

        // 당연하게도 타입이 잘 맞아야함
        // ex : _ => "string" (X)
    };
    println!("{}", second_number); // return 값 사용도 가능
    

    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }

    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) 
            if married == false => println!("married:n, {} children", children),
        (children, married) 
            if children == 0 && married == true => println!("Married & no children"),
        _ => println!("Married? {}. Number of children: {}.", married, children),
    }

    
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);


    // 변수 선언 시 : let _ if 는 못 씀
    let my_number = 10;
    if my_number == 10 {
        let some_variable = 8;
        println!("{}", some_variable);
    } 
    else {
        let some_variable = "Something else";
        println!("{}", some_variable);
    }


    // 패턴 매칭 사용
    match_number(50);
    match_number(13);
    match_number(4);

}



fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }

}


fn match_number(input: i32) {
    match input {
    number @ 4 => println!("{} unlucky number in China", number),
    number @ 13 => println!("{} unlucky in North America, lucky in Italy", number),
    _ => println!("Looks like a normal number"),
    }
    
    // You can also use @ to give a name to the value of a match expression
    // @ : 패턴 매칭 + 값 바인딩 
}




