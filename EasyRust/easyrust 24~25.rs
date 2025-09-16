

// 1  unit struct : doesn't have anything
struct _FileDirectory;

// 2  tuple struct
// = unnamed struct : only need to write the types, not the field names
struct Colour(u8, u8, u8);


// 3 named struct 
struct _SizeAndColour {
    size: u32,
    colour: Colour, // And we put it in our new named struct
}

struct Country {
    population: u32,
    capital: String,
    _leader_name: String
}



// 25 : Enum!
enum ThingsInTheSky {
    Sun,
    Stars,
}

enum Shape {
    Circle(f64),       // 반지름
    Rectangle(f64, f64), // 가로, 세로
    Triangle(f64, f64, f64), // 세 변 길이
}


fn main() {

// Chapter 24 : Structs
    // The name of a struct should be : compiler check !
    //  >> in UpperCamelCase (capital letter for each word, no spaces)
    
    let my_colour = Colour(50, 0, 50);
    println!("The second part of the colour is: {}", my_colour.1);


    let population = 500_000;
    let capital = String::from("Elista");
    let _leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population: population,
        capital: capital,
        _leader_name: _leader_name,
    };


    
    println!("population : {}", kalmykia.population);
    println!("capital : {}", kalmykia.capital);
    
    let population = 500_000;
    let capital = String::from("Elista");
    let _leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population,
        capital,
        _leader_name,
    };

    println!("population : {}", kalmykia.population);
    println!("capital : {}", kalmykia.capital);
        
// Chapter 25 : Enums : 서로 관련 있는 상수들의 집합을 나타내는 자료형
    // https://dhghomon.github.io/easy_rust/Chapter_25.html

    let time = 8; // it's 8 o'clock
    let skystate = create_skystate(time); 
    check_skystate(&skystate); 

    /*Use a struct when you want one thing AND another thing.
      Use an enum when you want one thing OR another thing.*/

    // enum : 여러 선택지 중에 고름. 하나 고르면 그 타입만 사용 가능!
    // struct 는 여러 타입의 변수가 한 블록을 이루는 것

    let c = Shape::Circle(2.0);
    match c {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("Rectangle {} x {}", w, h),
        _ => println!("not"),
    }

    
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}
    
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
        // match 는 쉼표도 허용함
    }

    
}



