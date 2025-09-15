

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
        
// Chapter 25 : Enums
    // https://dhghomon.github.io/easy_rust/Chapter_25.html
    
}




