
// EX1 
struct Person { 
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

// EX2
struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
        Self {
            name,
            name_before,
            population,
            date_founded,
        }
    }
}

fn process_city_values(city: &City) {
    let City {
        name,
        name_before,
        population,
        date_founded,
    } = city;
        // now we have the values to use separately
    let two_names = vec![name, name_before];
    println!("The city's two names are {:?}", two_names);
}


fn main() {
  // Chapter 28 : Destructuring
    // EX1
    let name = Person { 
        name: "name".to_string(), 
        real_name: "real_name".to_string(),
        //&static str(불변) >> string(가변)
        height: 170,
        happiness: false
    };

    let Person { 
        name: a,
        real_name: b,
        height: c,
        happiness: d
    } = name; 
    // 구조체 분해 : 구조체 → 필드 단위로 쪼개서 새 변수에 담은 것

    println!("name: {}  real_name: {}", a, b) ;
    println!("height: {}  happiness : {}",c, d);


    // EX2
    let tallinn = City::new("Tallinn".to_string(), "Reval".to_string(), 426_538, 1219);
    process_city_values(&tallinn); // 참조로 보내기

    
}
