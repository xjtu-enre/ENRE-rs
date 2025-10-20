//共计实体25   关系26
struct User
//结构体 定义
{
    //字段结构体
    active: bool,       //变量 定义
    username: String,   //变量 定义
    email: String,      //变量 定义
    sign_in_count: u64, //变量 定义
}
fn a() //函数 定义
{
    let user1 = User {
        //变量 定义
        //call关系
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
//元组结构体
struct Color(i32, i32, i32); //结构体 定义
struct Point(i32, i32, i32); //结构体 定义
//单元结构体
struct AlwaysEqual; //结构体 定义
// 函数体内定义结构体
fn outer() //函数 定义
{
    struct Inner; // 只在 outer 函数内可见//结构体 定义
    let _ = Inner; //变量 定义
}

// impl 块内部，再嵌套函数体里的结构体
struct S; //结构体 定义
impl S {
    //impl
    fn helper() {
        //关联函数 定义
        struct HelperCtx; // 只在 S::helper 函数内可见//结构体 定义
    }
}

fn main() //函数 定义
{
    outer(); //call关系
    S::helper(); //call关系
}
