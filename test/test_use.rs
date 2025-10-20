//共计 实体29个 关系47个
mod module_a {
    //模块 定义
    pub fn function_a() {} //函数 定义

    pub struct StructA; //结构体 定义
}

use module_a::StructA; //Basic Module Use  use
use module_a::function_a; //use

fn test() //函数 定义
{
    function_a(); //call
    let _s = StructA; //变量 定义
}
mod math {
    //模块 定义
    pub fn add(a: i32, b: i32) -> i32 {
        //函数 2个参数 3个定义
        a + b //usevar
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        //函数 2个参数 3个定义
        a - b //usevar
    }

    pub const PI: f64 = 3.14159; //常量 定义
}

use math::{PI, add}; //Selective Use 2个use

fn test2() //函数 定义
{
    let result = add(2, 3); //变量 定义 call
    println!("Pi: {}", PI); //usevar
}
mod graphics {
    //模块 定义
    pub struct Point {
        //结构体 定义
        pub x: f64, //变量 定义
        pub y: f64, //变量 定义
    }

    pub fn draw_point(p: Point) //函数 参数 2个定义
    {
        // ...
    }
}

use graphics::Point as GPoint; //Renaming Use  ImportAlias use
use graphics::draw_point as draw; //ImportAlias use
mod utils {
    //模块 定义
    pub fn func_a() {} //函数 定义
    pub fn func_b() {} //函数 定义
    pub const CONSTANT: i32 = 42; //常量 定义
    pub struct StructA; //结构体 定义
}

use utils::*; //global 4个use
