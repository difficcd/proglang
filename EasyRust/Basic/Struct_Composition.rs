

//Engine
struct Engine {
    horsepower: u32,
}

impl Engine {
    fn start(&self) {
        println!("엔진 가동! {} 마력", self.horsepower);
    }
}

// Car
struct Car {
    brand: String,
    engine: Engine, // Car는 Engine을 포함(Composition)
}

impl Car {
    fn drive(&self) {
        self.engine.start(); // 포함했으므로 impl도 공유함
        println!("{} 자동차가 출발합니다!", self.brand);
    }
}

fn main() {
    let engine = Engine { horsepower: 150 };
    let car = Car {
        brand: String::from("Hyundai"),
        engine,
    };

    car.drive();
}
