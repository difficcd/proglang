

// ===== self & Self 설명 =====
struct Person {
    name: String,
    // struct 인스턴스 생성 :
    //    let p = Person {name: String::from("name"),};
    //    사용은 p.name...
}

impl Person {
    // 1 ) Self = Person 이라는 타입 그 자체, 바꿔도 돌아감
    fn new(name: String) -> Self {   // 여기서 Self == Person
        Self { name }                // 여기서도 Self == Person
    }

    // 2 ) self = 객체 그 자신 (instance) 
    fn consume(self) {         // self: 값 자체를 소유 (move)
        println!("bye, {}", self.name);
    }

    fn borrow(&self) {         // self: 불변 참조
        println!("hi, {}", self.name);
    }

    fn borrow_mut(&mut self) { // self: 가변 참조
        self.name.push_str("!");
    }
}





#[derive(Debug)]
pub struct Animal { // Struct : 데이터 묶음 (변수의 틀)
    pub age: u8, // pub 필드는 외부참조가능 (구조체 자체 pub필수)
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType { // Enum : 여러 상태 중 하나
    Cat,
    Dog,
}

impl Animal {  // Impl : 메서드/기능 붙이기 (class 메서드 정의 유사)
    // impl & struct : class 와 비슷하지만 상속 등이 없음
    fn new(age: u8, animal_type: AnimalType) -> Self {
        Self {
            age,
            animal_type,
        }
    }
    
    fn change_to_dog(&mut self) {
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("It's a dog"),
            AnimalType::Cat => println!("It's a cat"),
        }
    }
}



// EX 2 
enum Mood {
    Good,
    Bad,
    Sleepy,
}

impl Mood { // 단순 enum + impl 예시
    fn check(&self) {
        match self {
            Mood::Good => println!("good!"),
            Mood::Bad => println!("not feeling so good"),
            Mood::Sleepy => println!("Need sleep NOW"),
        }
    }
}


fn main() {
    let mut new_animal = Animal::new(10,AnimalType::Cat);

    // Animal type인 new_animal 은 impl-Animal 의 fn 사용가능
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();

    // EX2
    let my_mood = Mood::Sleepy;
    my_mood.check();
}
