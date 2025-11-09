fn main () {
    let vec1 = vec![1, 2, 3] ;
    let vec2 = vec![1, 2, 3, 1, 4] ;

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2)) ;
    println!("4 in vec1: {}", vec1.into_iter().any(|x| x == 4)) ;

    println!("find 1 in vec1: {:?}", vec2.into_iter().find(|&x| x == 1)) ;

}
