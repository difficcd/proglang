

// https://dhghomon.github.io/easy_rust/Chapter_20.html


fn print_country_1(country_name: String) {
    println!("{}", country_name);
}

fn print_country_2(country_name: String) -> String {
    println!("{}", country_name);
    country_name // return it here
}

fn print_country_3(country_name: &String) {
    println!("{}", country_name);
}


fn main(){
//  Chapter 18 : Giving reference to functions

    /* concept of owner : References are very useful for functions. 
       The rule of values : a value can only have one owner.*/

    let country = String::from("Austria");
    print_country_1(country); // ownership 이전
    // print_country(country); not works

    let country = String::from("Austria");
    let country = print_country_2(country);
    print_country_2(country);
    // country를 move로 넘김 → 이전 country 변수는 더 이상 유효하지 않음. 
    // 함수가 String을 다시 리턴 → 그 반환값을 **새로운 country 변수(쉐도잉)**에 바인딩
    // 즉, 소유권이 함수 → 호출자에게 다시 돌아온 것.

    let country = String::from("Austria");
    print_country_3(&country); 
    print_country_3(&country); 
    // 참조 type 으로 보내는 경우 소유권의 이동이 없다
    // can look at it, but I will keep it


    // example of a function that uses a mutable variable
    let mut country = String::from("Austria");
    add_hungary_1(&mut country);


    // immutable variable 가공하는 방법
    let country = String::from("Austria"); 
    adds_hungary_2(country);



    
//  Chapter 19 : Copy types

    /* copy types : These simple types are all on the stack, 
       and the compiler knows their size. 
       That means that they are very easy to copy
       So you don't need to worry about ownership for these types. 
       These simple types include: integers, floats, booleans, and char.*/

    // Trait Implementations

    let my_number = 8;
    prints_number(my_number); 
    prints_number(my_number);
    // i32 : Copy trait 구현, 매개변수값은 복사된 값 
    
    // *String 은 Copy 가 아니기에(heap data) Copy가 아닌 move (ownership change)
    
    let country = String::from("Kiribati");
    print_country_1(country.clone()); 
    print_country_1(country.clone()); 
    print_country_1(country);
    // 복사본(clone) 을 보내면 String 다회 호출 가능
    // if the String is very large, .clone() can use a lot of memory
    // So using & for a reference is faster, if you can. 

    
    let mut my_string = String::new();
    for _ in 0..50 { // 반복문은 python 과 비슷 range(0,50) >> 0..50
        my_string.push_str("Here are some more words "); // push words
        get_length_1(my_string.clone()); 
    } // That's 50 clones

    let mut my_string = String::new();
    for _ in 0..50 { 
        my_string.push_str("Here are some more words "); // push words
        get_length_2(&my_string); 
    } // Instead of 50 clones, it's zero.



    // ** Variables without values (for, code block..)
    
    let my_number;
    {
        let number = {
            57
        };
        my_number = loop_then_return(number);
    }
    println!("{}", my_number);


    let number;
    {
        number = 100;
    }
    println!("{}", number);


    // 9.9 수업 내용 보충
    
    'outer: for i in 0..5 {          // 바깥쪽 루프에 'outer 라벨
        for j in 0..5 {
            println!("i = {}, j = {}", i, j);
            if i == 2 && j == 3 {
                println!("조건 충족 → 바깥 루프까지 break!");
                break 'outer;        // 바깥쪽 루프까지 즉시 탈출
            }
        }
    }
}


fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 { // 조건식에 괄호작성 X
            break;
        }
    }
    counter
}


fn get_length_1(input: String) { // Takes ownership of a String
    println!("It's {} words long.", input.split_whitespace().count()); // splits to count the number of words
}

fn get_length_2(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}



fn prints_number(number: i32) { // returning X 
    println!("{}", number);
} 

fn add_hungary_1(country: &mut String) { 
    // mut 인 경우 &mut 을 꼭 붙여주어야 함
    // mutable String  >> push
    country.push_str("-Hungary");
    println!("Now it says: {}", country);
}

fn adds_hungary_2(mut country: String) { 
    // 받는 시점에 mutable String 으로 받음
    /*mut country is not a reference: adds_hungary owns country now. 
    (Remember, it takes String and not &String). 
    The moment "you call adds_hungary, it becomes the full owner"
    country has nothing to do with String::from("Austria") anymore. 
    So adds_hungary_2 can take country as mutable & perfectly safe */
    
    country.push_str("-Hungary");
    println!("{}", country);
}

