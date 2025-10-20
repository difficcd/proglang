


fn main() {

// Chapter 20 : Collection types : array
  
    let _seasons = ["Spring", "Summer", "Autumn", "Winter"]; // Type : [&str; 4]
    let _seasons2 = ["Spring", "Fall", "Autumn"]; // Type : [&str; 3]

    let my_array = ["a"; 10]; // make 10 size array initialized "a" 
    println!("{:?}", my_array);
    println!("\n");

    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Index ranges are exclusive (they do not include the last number)
    let arr1 = &array_of_ten[2..5];   println!("{:?}", arr1); // 3 4 5 
    let arr2 = &array_of_ten[1..];   println!("{:?}", arr2);  // 2~*
    let arr3 = &array_of_ten[..5];   println!("{:?}", arr3); // 1~5
    let arr4 = &array_of_ten[..];   println!("{:?}", arr4); // 1~*

  
// Chapter 21 : Vectors
  
    //  Arrays are faster*  with less functionality
    // and vectors are slower*  with more functionality. 
  
    // two main ways to declare a vector*
    let name1 = String::from("Windy");
    let name2 = String::from("Class");

    // let mut vec = Vec::new();    >> compiler doesn't know the type of vec!
  
    let mut vec : Vec<String> = Vec::new(); // declare mutable

    // push : same as C++'s vector expression
    vec.push(name1);
    vec.push(name2); 
    println!("{:?}", vec);

    let mut pair_vec : Vec<(i32, i32)> = Vec::new();
    pair_vec.push((1,2)); 
    pair_vec.push((3,4)); 
    println!("{:?}", pair_vec);

    let mut _1d_vec1 : Vec<i32> = Vec::new();
    let mut _1d_vec2 : Vec<i32> = Vec::new();
    let mut _2d_vec : Vec<Vec<i32>> = Vec::new();
    _1d_vec1.push(1); _1d_vec1.push(2); // [1,2]
    _1d_vec2.push(0); // [0]
    _2d_vec.push(_1d_vec1);
    _2d_vec.push(_1d_vec2);
    println!("{:?}", _2d_vec); // [[1,2], [0]]
  
    /* 
       C++ ) vector<String> vec;
       Rust ) let mut vec : Vec<String> = Vec::new();

       C++ ) vector<pair<int,int>> vec;
       Rust ) let mut vec : Vec<(i32, i32)> = Vec::new();

       C++ ) vector<vector<int>> vec;
       Rust ) let mut vec : Vec<Vec<i32>> = Vec::new();
    */

    let mut _vec = vec![8, 10, 10]; // easu way to create Vector (i32)
    _vec.push(1);
    println!("{:?}", _vec);
    
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let vec1 = &vec_of_ten[2..5];   println!("{:?}", vec1); // 2 3 4
    let vec2 = &vec_of_ten[1..];   println!("{:?}", vec2);  // 1~*
    let vec3 = &vec_of_ten[..5];   println!("{:?}", vec3); // 0~5
    let vec4 = &vec_of_ten[..];   println!("{:?}", vec4); // 0~*

    // A vec has a capacity :  We'll use a method called .capacity() 
    //       to look at the capacity of a vector as we add items to it.

      
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
  
    num_vec.push('a');
    println!("{}", num_vec.capacity()); // 1 element: prints 4
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a'); 
    println!("{}", num_vec.capacity()); // 4 elements: still prints 4
    num_vec.push('a'); 
    println!("{}", num_vec.capacity()); // prints 8

    // So this vector has two reallocations: 0 to 4, and 4 to 8.
    // we can use some methods to make Vector faster. 

    let mut num_vec = Vec::with_capacity(8); // Give it capacity 8
    num_vec.push('a'); 
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); 
    num_vec.push('a');
    num_vec.push('a'); 
    num_vec.push('a'); 
    println!("{}", num_vec.capacity()); // Still 8


    // can use .into() to make a &str into a String,,
    // >> can also use it to make an array into a Vec

    // let my_vec: Vec<u8> = [1, 2, 3];    >> error : i32
    // Vec<String> <- [i32; 3]    >> error : i32 <=> u,i
    // 1,2,3,... : 컴파일러가 자동으로 i32 로 인식함. <_>도 마찬가지
    let my_vec1: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into();

    println!("{:?}", my_vec1);
    println!("{:?}", my_vec2);
  
    for i in my_vec1 {
      print!("{} ", i) ;
    } // C++ style (iterator)
    println!("") ;
    
}
