//共计 实体40个 关系87个
// 测试CHA算法的实现

// 基础trait定义
trait Drawable {
    // Entity: Trait, Relation: Define
    fn draw(&self); // Entity: Method, Relation: Define
    fn area(&self) -> f64; // Entity: Method, Relation: Define
}

// 带有默认实现的trait方法
trait Printable {
    // Entity: Trait, Relation: Define
    fn print(&self) {
        // Entity: Method with default implementation, Relation: Define
        println!("Printing...");
    }

    fn format(&self) -> String {
        // Entity: Method with default implementation, Relation: Define
        "Default format".to_string()
    }
}

// 继承关系：ColoredDrawable继承Drawable
trait ColoredDrawable: Drawable {
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

struct Triangle {
    // Entity: Struct, Relation: Define
    base: f64,   // Entity: Variable, Relation: Define
    height: f64, // Entity: Variable, Relation: Define
}

// 为结构体实现Drawable trait
impl Drawable for Circle {
    // Entity: Implementation, Relation: Impl
    fn draw(&self) {
        // Entity: Method, Relation: Define
        println!("Drawing a circle");
    }

    fn area(&self) -> f64 {
        // Entity: Method, Relation: Define
        std::f64::consts::PI * self.radius * self.radius // Entity: Variable, Relation: UseVar
    }
}

impl Drawable for Rectangle {
    // Entity: Implementation, Relation: Impl
    fn draw(&self) {
        // Entity: Method, Relation: Define
        println!("Drawing a rectangle");
    }

    fn area(&self) -> f64 {
        // Entity: Method, Relation: Define
        self.width * self.height //  Relation: UseVar
    }
}

impl Drawable for Triangle {
    // Entity: Implementation, Relation: Impl
    fn draw(&self) {
        // Entity: Method, Relation: Define
        println!("Drawing a triangle");
    }

    fn area(&self) -> f64 {
        // Entity: Method, Relation: Define
        0.5 * self.base * self.height //  Relation: UseVar
    }
}

// 为结构体实现ColoredDrawable trait
impl ColoredDrawable for Circle {
    // Entity: Implementation, Relation: Impl
    fn color(&self) -> &str {
        // Entity: Method, Relation: Define
        "red" // Entity: Constant, Relation: Use
    }
}

impl ColoredDrawable for Rectangle {
    // Entity: Implementation, Relation: Impl
    fn color(&self) -> &str {
        // Entity: Method, Relation: Define
        "blue" // Entity: Constant, Relation: Use
    }
}

// 为结构体实现Printable trait
impl Printable for Circle {
    // Entity: Implementation, Relation: Impl
    fn print(&self) {
        // Entity: Method, Relation: Define
        println!("Printing circle with area: {}", self.area()); // Entity: Function call, Relation: Call
    }
}

impl Printable for Rectangle {
    // Entity: Implementation, Relation: Impl
    // 使用默认实现
}

// 使用trait object的函数
fn render(shape: &dyn Drawable) {
    // Entity: Function, Relation: Define
    shape.draw(); // Entity: Method call, Relation: Call
    println!("Area: {}", shape.area()); // Entity: Function call, Relation: Call
}

fn render_boxed(shape: Box<dyn Drawable>) {
    // Entity: Function, Relation: Define
    shape.draw(); // Entity: Method call, Relation: Call
    println!("Area: {}", shape.area()); // Entity: Function call, Relation: Call
}

fn print_item(item: &dyn Printable) {
    // Entity: Function, Relation: Define
    item.print(); // Entity: Method call, Relation: Call
    println!("Formatted: {}", item.format()); // Entity: Function call, Relation: Call
}

fn render_colored(shape: &dyn ColoredDrawable) {
    // Entity: Function, Relation: Define
    shape.draw(); // Entity: Method call, Relation: Call
    println!("Color: {}", shape.color()); // Entity: Function call, Relation: Call
}

// 测试CHA算法的主函数
fn main() {
    // Entity: Function, Relation: Define
    let circle = Circle { radius: 5.0 }; // Entity: Variable, Relation: Define
    let rectangle = Rectangle {
        // Entity: Variable, Relation: Define
        width: 10.0, // Entity: Field, Relation: Define
        height: 8.0, // Entity: Field, Relation: Define
    };
    let triangle = Triangle {
        // Entity: Variable, Relation: Define
        base: 6.0,   // Entity: Field, Relation: Define
        height: 4.0, // Entity: Field, Relation: Define
    };

    // 直接调用（不涉及CHA）
    circle.draw(); // Entity: Method call, Relation: Call
    rectangle.draw(); // Entity: Method call, Relation: Call
    triangle.draw(); // Entity: Method call, Relation: Call

    // 通过trait object调用（涉及CHA）
    render(&circle); // Entity: Function call, Relation: Call
    render(&rectangle); // Entity: Function call, Relation: Call
    render(&triangle); // Entity: Function call, Relation: Call

    // 通过Box<dyn Trait>调用（涉及CHA）
    render_boxed(Box::new(circle)); // Entity: Function call, Relation: Call
    render_boxed(Box::new(rectangle)); // Entity: Function call, Relation: Call
    render_boxed(Box::new(triangle)); // Entity: Function call, Relation: Call

    // 测试继承关系的CHA
    let colored_circle: Box<dyn ColoredDrawable> = Box::new(circle); // Entity: Variable, Relation: Define
    render_colored(&*colored_circle); // Entity: Function call, Relation: Call

    // 测试默认实现
    let printable_circle: Box<dyn Printable> = Box::new(circle); // Entity: Variable, Relation: Define
    let printable_rectangle: Box<dyn Printable> = Box::new(rectangle); // Entity: Variable, Relation: Define
    print_item(&*printable_circle); // Entity: Function call, Relation: Call
    print_item(&*printable_rectangle); // Entity: Function call, Relation: Call
}
