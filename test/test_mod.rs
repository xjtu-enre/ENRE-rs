//共计 实体20个 关系20个
//单模块
mod utils {
    //模块 定义
    pub fn greet() //函数 定义
    {
        println!("Hello from utils!");
    }
}
mod utils2; //模块 定义
//模块嵌套
mod front_of_house {
    //模块 定义
    mod hosting {
        //模块 定义
        fn add_to_waitlist() {} //函数 定义

        fn seat_at_table() {} //函数 定义
    }

    mod serving {
        fn take_order() {} //函数 定义

        fn serve_order() {} //函数 定义

        fn take_payment() {} //函数 定义
    }
} //  函数体内部嵌套模块
fn outer_fn() {
    //函数 定义
    mod inside_fn {
        //模块 定义
        pub const MSG: &str = "hello from fn"; //常量 定义
    }
    let _ = inside_fn::MSG; //变量 定义 
}
struct S; //结构体 定义

// 关联函数内部再嵌模块
fn helper() //函数 定义
{
    mod inside_fn {
        //模块 定义
        pub fn work() {} //函数 定义
    }
    inside_fn::work(); //call
}
