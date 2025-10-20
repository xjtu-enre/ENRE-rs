// 测试修改后的CHA算法

// 基础trait
trait Animal {
    fn speak(&self);
    fn age(&self) -> u32 {
        0
    }
}

// 继承trait
trait DogTrait: Animal {
    fn breed(&self) -> &str;
}

// 结构体定义
struct Dog;
struct Cat;

// 为结构体实现基础trait
impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }

    fn age(&self) -> u32 {
        5
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }

    // 使用默认实现age()
}

// 为结构体实现继承trait
impl DogTrait for Dog {
    fn breed(&self) -> &str {
        "Labrador"
    }
}

// 使用trait object的函数
fn call_animal_speak(a: &dyn Animal) {
    a.speak();
}

fn call_dog_breed(d: &dyn DogTrait) {
    println!("Breed: {}", d.breed());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    call_animal_speak(&dog);
    call_animal_speak(&cat);

    call_dog_breed(&dog);
}
