# Entity: Function

Functions are reusable blocks of code that accept parameters and return a value. They are the primary way to organize and structure code in Rust, allowing for modular and maintainable programs.

## Supported Patterns

### name: FunctionDefinition
### Syntax: FunctionDefinition
```rust
fn function_name(parameter1: Type1, parameter2: Type2) -> ReturnType {
    // function body
    return value;
}
```

### Examples

#### Basic Function Definition
```rust
fn greet() {
    println!("Hello, world!");
}

fn add_one(x: i32) -> i32 {
    x + 1
}
```

```yaml
name: BasicFunctionDefinition
entity:
  type: Function
  extra: false
  items:
  - type: Function
    qualified: test_basic_function.greet
    name: greet
    loc: '1:4'
  - type: Function
    qualified: test_basic_function.add_one
    name: add_one
    loc: '5:4'
```

#### Function with Parameters and Return Value
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: f64, y: f64) -> f64 {
    x * y
}
```

```yaml
name: FunctionWithParameters
entity:
  type: Function
  extra: false
  items:
  - type: Function
    qualified: test_function_params.add
    name: add
    loc: '1:4'
  - type: Function
    qualified: test_function_params.multiply
    name: multiply
    loc: '5:4'
```

#### Method Definition
```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

```yaml
name: MethodDefinition
entity:
  type: Function
  extra: false
  items:
  - type: Function
    qualified: test_method.Rectangle.area
    name: area
    loc: '7:8'
  - type: Function
    qualified: test_method.Rectangle.can_hold
    name: can_hold
    loc: '11:8'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isMethod | Indicates whether the function is a method (has self parameter). | boolean | false |
| isGeneric | Indicates whether the function is generic. | boolean | false |
| isPublic | Indicates whether the function is publicly accessible. | boolean | false |