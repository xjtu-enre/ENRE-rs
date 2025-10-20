//共计 实体39个 关系41个
//最简单定义
enum Book {
    // Entity: Enum, Relation: Define
    Papery,     // Entity: Variant Relation: Define
    Electronic, // Entity: Variant Relation: Define
}
//加元组属性描述
enum Book2 {
    // Entity: Enum, Relation: Define
    Papery2(u32),        // Entity: Variant Variable Relation: Define
    Electronic2(String), // Entity: Variant Variable Relation: Define
}
//结构体语法
enum Book3 {
    // Entity: Enum, Relation: Define
    Papery3 { index: u32 }, // Entity: Variant with struct Variable Relation: Define
    Electronic3 { url: String }, // Entity: Variant with struct Variable Relation: Define
}
//Option 枚举类
enum Option<T> {
    // Entity: Enum  Relation: Define
    Some(T), // Entity: Variant with generic Relation: Define
    None,    // Entity: Variant Relation: Define
}
fn main() {
    // Entity: Function, Relation: Define
    let opt = Option::Some("Hello"); // Entity: Variable, Relation: Define, call
}
//混合
enum WebEvent {
    // Entity: Enum, Relation: Define
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,   // Entity: Variant Relation: Define
    PageUnload, // Entity: Variant Relation: Define
    // 或者一个元组结构体，
    KeyPress(char), // Entity: Variant Variable Relation: Define
    Paste(String),  // Entity: Variant Variable Relation: Define
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 }, // Entity: Variant Variable Variable Relation: Define
}
//模块嵌套
mod net {
    // Entity: Module, Relation: Define
    enum ConnectionState {
        // Entity: Enum, Relation: Define, Contain
        Connecting, // Entity: Variant Relation: Define
        Connected,  // Entity: Variant Relation: Define
        Closed,     // Entity: Variant Relation: Define
    }
}
//函数体内
fn work() {
    // Entity: Function, Relation: Define
    enum LocalCmd {
        // Entity: Enum, Relation: Define
        Inc, // Entity: Variant Relation: Define
        Dec, // Entity: Variant Relation: Define
    }
    let a = LocalCmd::Inc; // Entity: Variable, Relation: Define
}
