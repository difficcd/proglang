


fn times_two(number: i32) -> i32 {
    number * 2
}


fn r#return() -> u8 { // 함수명 중복도 r#로 사용가능
    println!("Here is your number.");
    8
}


fn main() {
    println!("smallest i8 : {} , biggest i8 : {}.", i8::MIN, i8::MAX); // -128~127

    // === Mutability : 변수 고정값 ===
    // immutable by default (let a, a=.. >> x!)
    let a = 0;
    println!("{}", a);
    {
        let a = 1; // shadowing (덮어쓰기)
        println!("{}", a);
    }
    println!("{}", a); // print 0 (블럭 안 갱신이므로)

    let mut mutable = 0;
    // 선언 시에 mut 해주면 mutable
    // 변경하지 않으면 컴파일러가 오류

    mutable = 1; // mutable 변수라도 같은 i32 로 변경해주어야 함
    println!("{}", mutable);

    // === Memory + reference ===
    // reference : rust 에서의 pointer 사용

    let num = 10;
    let _r1 = &num;
    let _r2 = &_r1;
    let _r5 = &&&&&num;
    //  모두 다른 타입, 비교 불가. &&&&i32, &&i32...

    println!("{:p}", _r1); // reference pointer address 출력
    println!("{:x}", 9000); // hexa 9000 => 2328
    println!("{:b}", 10); // bit 1010,,
                          // octal : o

    let final_number = {
        let y = 10;
        let x = 9; // x starts at 9
        let x_twice = times_two(x); // second name for x
        let x_twice_and_y = x_twice + y; // third name for x
        x_twice_and_y
        // too bad we didn't have shadowing - we could have just used x.
    };
    println!("The number is now: {}", final_number);

    //fancy printing
    print!("escape : \\ \n");
    print!(r#"C:\users:\"#); // raw test 작성법
    println!(r#####"this is ####"#####); // #의 개수+1만큼 # 붙여줌 >> 구분

    let mut r#mut = 1; // 지정단어를 사용가능하게 해줌
    println!("this is mut : {}", r#mut);

    println!(
        "\n
a
b
c"
    ); // 줄바꿈 자동인식 & 탭 인식

    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!(
        "This is {1} {2}, son of {0} {2}.",
        father_name, son_name, family_name
    );



    let my_number = r#return();
    println!("{}", my_number);
  
    println!("{:?}", b"This will look like numbers"); 
    // byte string. b를 앞에 붙여주면 &str 을 char 로 다룰 수 있음
    
    println!("{:?}", br##"This will "#"."##); 
    // b, r 같이 사용 가능 (이스케이프 필요시)

    println!("{:X}", 'い' as u32); // u32 casting & hexa 출력


    // 쉬운 포맷형 글자 출력
    let letter = "O";
    println!("{:o^10}", letter); 
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // 30 길이 안에 title, - 중앙 정렬
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); 
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);

}
