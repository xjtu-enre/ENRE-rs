//共计 实体22个 关系40个
//普通函数
fn greet() //函数 定义
{
    println!("你好，锈蚀世界！");
}
//带参数
fn greet_person(name: &str) //函数 参数 2个定义
{
    println!("你好，{}！欢迎来到锈蚀世界！", name); //usevar
}
//多个复杂参数
fn make_salad(greens: &str, veggies: Vec<&str>, dressing: Option<&str>)
//函数 3个参数 4个定义
{
    println!("制作一份{}沙拉", greens); //usevar
    for veggie in veggies {
        //变量 定义
        println!("添加{}", veggie); //usevar
    }
    match dressing {
        Some(d) => println!("淋上{}酱", d), //变量 定义 usevar
        None => println!("不加酱料"),
    }
}
//高阶函数
fn apply_function<F>(f: F, value: i32) -> i32
//函数 2个参数 3个定义
where
    F: Fn(i32) -> i32,
{
    f(value) //call
}
fn main() //函数 定义
{
    let double = |x| x * 2; //变量 闭包 2定义
    let result = apply_function(double, 5); //变量 call usevar
    println!("Result: {}", result); // usevar
}
fn main2() //函数 定义
{
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; //变量 定义
    let result: i32 = numbers //变量 定义
        .iter()
        .filter(|&&x| x % 2 == 0) //闭包 定义
        .map(|&x| x * x) //闭包 定义
        .sum();
    println!("Sum of squares of even numbers: {}", result); //usevar
}
