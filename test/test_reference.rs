//共计11个实体 22个关系
//不可变引用
fn test() //函数实体 定义
{
    let mut x = 5; //变量 定义
    let y = &x; //变量   y引用x 定义

    println!("x: {}", x); //使用x
    println!("y: {}", y); //使用y
}
//可变引用
fn main() //函数实体 定义
{
    let mut x = 5; //变量 定义
    let y = &mut x; //变量   可变引用 定义

    *y += 1; //通过y修改x(modify) 使用y

    println!("x: {}", x); //使用x
    println!("y: {}", y); //使用y
}
fn main1() //函数实体 定义
{
    // 数组元素引用
    let arr = [1, 2, 3, 4, 5]; //变量arr 定义
    let first = &arr[0]; // 变量first 不可变引用数组元素 定义
    let second = &arr[1]; //变量second  不可变引用数组元素 定义

    println!("First: {}, Second: {}", *first, *second); //使用变量first、变量second
}
