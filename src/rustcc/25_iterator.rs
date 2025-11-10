fn main () {
    let vec1 = vec![1, 2, 3] ;
    let vec2 = vec![1, 2, 3, 1, 4] ;

    // iter() : 모든 요소 순회
    // any(*) : 해당 조건 *에 맞는 요소를 찾으면 true리턴
    // find(*) : 해당 조건 *에 맞는 요소를 찾으면 해당 조건에 맞는 원소리턴
    // 참고로 iter(): 빌림  into_iter(): 값 자체 이동(move) 혹은 빌림(borrow)
    
    //   # |&x| x == 2 이런거 전체가 closure, 헷갈리면 안됨
    
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2)) ; // &x: 벡터라 참조
    println!("4 in vec1: {}", vec1.into_iter().any(|x| x == 4)) ;

    println!("find 1 in vec1: {:?}", vec2.into_iter().find(|&x| x == 1)) ;


    /* [이 작업을 왜 closure로 하는가? closure의 의의는?] => 안전성, 추상화 
    => 추상화 :: .any() 나 .find() 같은 함수는 “조건”을 모르고도 동작해야 함
    => 배열 참조 오류가 많이 나니까, 사용자에게 맡기는 대신 PL차원에서
       이런 함수를 제공할테니 함수 범위에서 안전하게, 하이레벨(추상화)로 접근하도록 함
       하지만 이러면 직접적인 조건문을 사용하기 힘든데, 여기서 closure를 사용하면
       추상화하여 안전히 참조하는 동시에 조건문도 사용할 수 있음
       : int 연결리스트 구현은 쉬운데 double 연결리스트 구현은 어려운 것과 같은 문제를
         해결하기 위해서는 PL이 제공하는 타입적인 부분들에 스스로의 코드로 수정가능하도록
         만들 필요가 있음
    */

    /* [연결리스트에 대한 비유 부연설명]
    typedef struct IntNode { int data; struct IntNode* next; } IntNode;
    typedef struct DoubleNode { double data; struct DoubleNode* next; } DoubleNode;
    typedef struct PersonNode { struct Person data; struct PersonNode* next; } PersonNode;
      : C에서는 타입이 컴파일타임에 고정되어 유지보수 힘들고 코드 재사용 안됨 (타입고정의 문제)
      : 여기서의 문제의 본질은 타입을 데이터로 다룰 수 없다는것 >> closure, generic, callback 필요
      : closure는 여기서 타입마다 다른 데이터 연산방법을 "외부에서 주입" 가능하도록 만들어 줌
        코드를 던질 수 있기 때문에, 
    */
}
