//共计 22个实体 30个关系
pub trait Summary //接口 定义
{
    fn summarize(&self) -> String; //方法 定义
}
pub struct Post {
    //结构体 定义
    pub title: String,   // 标题   //变量 定义
    pub author: String,  // 作者 //变量 定义
    pub content: String, // 内容 //变量 定义
}

impl Summary for Post {
    //实现
    fn summarize(&self) -> String {
        //方法 定义
        format!("文章{}, 作者是{}", self.title, self.author) //usevar
    }
}

pub struct Weibo {
    //结构体 定义
    pub username: String, //变量 定义
    pub content: String,  //变量 定义
}

impl Summary for Weibo {
    //实现
    fn summarize(&self) -> String {
        //方法 定义
        format!("{}发表了微博{}", self.username, self.content) //usevar
    }
}
// 父trait示例：定义基本的打印功能
trait Printable {
    ////接口 定义
    fn print(&self); //方法 定义
}

// 定义父trait，并继承自Printable
trait Debuggable: Printable {
    //接口 定义 继承
    fn debug(&self); //方法 定义
}
// 定义trait A
trait A {
    //接口 定义
    fn func_a(&self); //方法 定义
}

// 定义trait B
trait B {
    //接口 定义
    fn func_b(&self); //方法 定义
}

// 定义父trait，并继承自A和B
trait C: A + B {
    //接口 定义 2个继承
    fn func_c(&self); //方法 定义
}
