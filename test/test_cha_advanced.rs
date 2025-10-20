//共计 实体49个 关系78个
// 更复杂的CHA算法测试用例

// 基础trait
trait Shape {
    // Entity: Trait, Relation: Define
    fn area(&self) -> f64; // Entity: Method, Relation: Define
}

// 继承trait
trait Drawable: Shape {
    // Entity: Trait, Relation: Define, Inherit
    fn draw(&self); // Entity: Method, Relation: Define
}

// 进一步继承的trait
trait Printable: Drawable {
    // Entity: Trait, Relation: Define, Inherit
    fn print(&self) {
        // Entity: Method with default implementation, Relation: Define
        self.draw(); // Relation: Call
        println!("Area: {}", self.area()); //  Relation: Call
    }
}

// 具体实现trait
trait Colored: Printable {
    // Entity: Trait, Relation: Define, Inherit
    fn color(&self) -> &str; // Entity: Method, Relation: Define
}

// 结构体定义
struct Circle {
    // Entity: Struct, Relation: Define
    radius: f64, // Entity:Variable, Relation: Define
}

struct Rectangle {
    // Entity: Struct, Relation: Define
    width: f64,  // Entity: Variable, Relation: Define
    height: f64, // Entity: Variable, Relation: Define
}

struct ColoredCircle {
    // Entity: Struct, Relation: Define
    radius: f64,   // Entity:Variable, Relation: Define
    color: String, // Entity:Variable, Relation: Define
}

// 实现基础trait
impl Shape for Circle {
    // Entity: Implementation, Relation: Impl
    fn area(&self) -> f64 {
        // Entity: Method, Relation: Define
        std::f64::consts::PI * self.radius * self.radius // Entity: Variable , Relation: UseVar
    }
}

impl Shape for Rectangle {
    // Entity: Implementation, Relation: Impl
    fn area(&self) -> f64 {
        // Entity: Method, Relation: Define
        self.width * self.height // Entity: Variable , Relation: UseVar
    }
}

impl Shape for ColoredCircle {
    // Entity: Implementation, Relation: Impl
    fn area(&self) -> f64 {
        // Entity: Method, Relation: Define
        std::f64::consts::PI * self.radius * self.radius // Entity: Variable usage, Relation: UseVar
    }
}

// 实现继承trait
impl Drawable for Circle {
    // Entity: Implementation, Relation: Impl
    fn draw(&self) {
        // Entity: Method, Relation: Define
        println!("Drawing a circle"); //  Relation: Call
    }
}

impl Drawable for Rectangle {
    // Entity: Implementation, Relation: Impl
    fn draw(&self) {
        // Entity: Method, Relation: Define
        println!("Drawing a rectangle"); //  Relation: Call
    }
}

impl Drawable for ColoredCircle {
    // Entity: Implementation, Relation: Impl
    fn draw(&self) {
        // Entity: Method, Relation: Define
        println!("Drawing a colored circle"); //  Relation: Call
    }
}

// 实现进一步继承的trait
impl Printable for Circle {
    // Entity: Implementation, Relation: Impl
    // 使用默认实现
}

impl Printable for Rectangle {
    // Entity: Implementation, Relation: Impl
    fn print(&self) {
        // Entity: Method, Relation: Define
        self.draw(); //  Relation: Call
        println!("Rectangle area: {}", self.area()); //  Relation: Call
    }
}

// 实现具体trait
impl Colored for ColoredCircle {
    // Entity: Implementation, Relation: Impl
    fn color(&self) -> &str {
        // Entity: Method, Relation: Define
        &self.color //   Relation: UseVar
    }
}

// 使用trait object的函数
fn process_shape(shape: &dyn Shape) {
    // Entity: Function, Relation: Define
    println!("Area: {}", shape.area()); //  Relation: Call
}

fn render_drawable(drawable: &dyn Drawable) {
    // Entity: Function, Relation: Define
    drawable.draw(); // Relation: Call
    println!("Area: {}", drawable.area()); //  Relation: Call
}

fn print_printable(printable: &dyn Printable) {
    // Entity: Function, Relation: Define
    printable.print(); // Entity: Method call, Relation: Call
}

fn show_colored(colored: &dyn Colored) {
    // Entity: Function, Relation: Define
    colored.draw(); //  Relation: Call
    println!("Color: {}", colored.color()); //  Relation: Call
    colored.print(); // E Relation: Call
}

// 测试CHA算法的主函数
fn main() {
    // Entity: Function, Relation: Define
    let circle = Circle { radius: 5.0 }; // Entity: Variable, Relation: Define
    let rectangle = Rectangle {
        // Entity: Variable, Relation: Define
        width: 10.0,
        height: 8.0,
    };
    let colored_circle = ColoredCircle {
        // Entity: Variable, Relation: Define
        radius: 3.0,
        color: "red".to_string(),
    };

    // 通过基础trait object调用
    process_shape(&circle); //  Relation: Call
    process_shape(&rectangle); //  Relation: Call
    process_shape(&colored_circle); //  Relation: Call

    // 通过继承trait object调用
    render_drawable(&circle); //  Relation: Call
    render_drawable(&rectangle); //  Relation: Call
    render_drawable(&colored_circle); //  Relation: Call

    // 通过进一步继承trait object调用
    print_printable(&circle); //  Relation: Call
    print_printable(&rectangle); // Relation: Call
    print_printable(&colored_circle); //  Relation: Call

    // 通过具体trait object调用
    show_colored(&colored_circle); //  Relation: Call

    // 混合使用
    let shapes: Vec<Box<dyn Shape>> = vec![
        // Entity: Variable, Relation: Define
        Box::new(circle),         //  Relation: Call
        Box::new(rectangle),      //  Relation: Call
        Box::new(colored_circle), //  Relation: Call
    ];

    for shape in &shapes {
        // Entity: Variable, Relation: UseVar
        process_shape(&**shape); //  Relation: Call
    }

    let drawables: Vec<Box<dyn Drawable>> = vec![
        // Entity: Variable, Relation: Define
        Box::new(Circle { radius: 2.0 }), //  Relation: Call
        Box::new(Rectangle {
            //  Relation: Call
            width: 3.0,
            height: 4.0,
        }),
        Box::new(ColoredCircle {
            //  Relation: Call
            radius: 1.0,
            color: "blue".to_string(),
        }),
    ];

    for drawable in &drawables {
        // Entity: Variable, Relation: UseVar
        render_drawable(&**drawable); //  Relation: Call
    }
}
