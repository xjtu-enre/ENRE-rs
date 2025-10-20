//总计 实体17个 关系18个
type Kilometers = i32; //类型别名 定义
type Thunk = Box<dyn Fn() + Send + 'static>; //类型别名 定义

fn main() //函数 定义
{
    let x: Kilometers = 5; //变量 定义
    let f: Thunk = Box::new(|| println!("hi")); //变量 闭包 2个定义
}
mod networking {
    //模块 定义
    type Port = u16; //类型别名 定义
    type Host = String; //类型别名 定义

    pub struct Server {
        //结构体 定义
        host: Host, //变量 定义
        port: Port, //变量 定义
    }
}
pub type PublicAlias = std::collections::HashMap<String, i32>; //类型别名 定义
type PrivateAlias = Vec<String>; //类型别名 定义

pub struct MyStruct {
    //结构体 定义
    pub data: PublicAlias,      //变量 定义
    private_data: PrivateAlias, //变量 定义
}
