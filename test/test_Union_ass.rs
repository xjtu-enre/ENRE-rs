//总计  实体25个 关系34个
union MyUnion {
    //union 定义
    i: i32,  //变量 定义
    f: f32,  //变量 定义
    c: char, //变量 定义
}

fn main() {
    //函数 定义
    let u = MyUnion { i: 10 }; //变量 定义
    // Only unsafe access is allowed
    unsafe {
        println!("Integer: {}", u.i); //usevar
        println!("Float: {}", u.f); //usevar
        println!("Char: {}", u.c as u8); //usevar importalias
    }
}
union ComplexUnion {
    //union 定义
    array: [i32; 4],             //变量 定义
    tuple: (i32, i32, i32, i32), //变量 定义
    struct_val: [i32; 4],        //变量 定义
}

fn test() {
    //函数 定义
    let u = ComplexUnion {
        //变量 定义
        array: [1, 2, 3, 4],
    };

    unsafe {
        println!("Array: {:?}", u.array); //usevar
        println!("Tuple: {:?}", u.tuple); //usevar
        println!("Struct: {:?}", u.struct_val); //usevar
    }
}
struct Point {
    //结构体 定义
    x: f64, //变量 定义
    y: f64, //变量 定义
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        //Associatedfunction 2个变量 3个定义
        Point { x, y } //usevar
    }

    fn origin() -> Point {
        //Associatedfunction 定义
        Point { x: 0.0, y: 0.0 }
    }
}
struct Circle {
    //结构体 定义
    radius: f64, //变量 定义
}
impl Circle {
    const PI: f64 = 3.14159; //Associatedconst 定义
    const DEFAULT_RADIUS: f64 = 1.0; //Associatedconst 定义

    fn area(&self) -> f64 {
        //Associatedfunction 定义
        Self::PI * self.radius * self.radius
    }
}
