//共计25个实体 30个关系
//常量
fn test() //函数实体  定义
{
    const USER_LIMIT: i32 = 100; // 声明一个整型常量  定义
    const PI: f32 = 3.14; //常量实体  定义
}
//static变量
static NUM: i32 = 100; //不可变static变量 定义
static mut NUM2: i32 = 100; //可变static变量 定义

fn test2() //函数实体 定义
{
    // 声明并未初始化的变量
    let x: i32; //变量 定义

    // 声明并初始化变量
    let y = 5; //不可变变量 定义
    let mut z = 10; // z 是可变变量 定义
    z = 20; // modify
    let d = 5; //变量 定义
    let d = 6; // modify
    {
        let d = 9; // 在这个内部作用域中，d的值为 9
    }
    let (a, mut b): (bool, bool) = (true, false); //变量a、b  两次定义
    // a = true,不可变; b = false，可变
    let [x, y2, z2]: [i32; 3] = [1, 2, 3]; //3个变量 3次定义
    struct Point
//结构体 定义
    {
        x: i32, //变量 定义
        y: i32, //变量 定义
    }
    let Point { x, mut y } = Point { x: 1, y: 2 }; ////变量 定义
    let &(a3, b3) = &(1, 2); //引用模式 //2变量 2定义
    let ((a2, b2), Point2 { x2, mut y2 }) = ((1, 2), Point2 { x2: 3, y2: 4 }); //混合嵌套+
}
