//总计 实体:29个 关系38个
struct Point {
    //结构体 定义
    x: f64, //变量 定义
    y: f64, //变量 定义
}

impl Point {
    fn origin() -> Point {
        //函数 定义
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        //函数 2个变量 3个定义
        Point { x: x, y: y } //usevar
    }
}

struct Rectangle {
    //结构体 定义
    p1: Point, //变量 定义
    p2: Point, //变量 定义
}

impl Rectangle {
    fn area(&self) -> f64 {
        //方法 定义
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        //方法 定义
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    fn translate(&mut self, x: f64, y: f64) {
        //方法 2个变量 3个定义
        self.p1.x += x; //usevar
        self.p2.x += x; //usevar

        self.p1.y += y; //usevar
        self.p2.y += y; //usevar
    }
}
trait Drawable {
    //接口 定义
    fn draw(&self); //方法 定义
}

struct Circle {
    //结构体 定义
    radius: f64, //变量 定义
}

impl Drawable for Circle {
    fn draw(&self) {
        //方法 定义 实现
        println!("Drawing circle with radius {}", self.radius); //usevar
    }
}
struct Rectangle {
    //结构体 定义
    width: f64,  //变量 定义
    height: f64, //变量 定义
}

impl Rectangle {
    fn area(&self) -> f64 {
        //方法 定义
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        //方法  参数 2个定义
        self.width > other.width && self.height > other.height //usevar
    }

    fn square(size: f64) -> Rectangle {
        //函数 参数 两个定义
        Rectangle {
            width: size, //usevar
            height: size,
        }
    }
}
