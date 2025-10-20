# Relation: UseVar

Using variables, where an entity uses a variable within its scope, which could be a local variable, field, or parameter.

## Supported Patterns

### name: UseVar
### Semantic:

### Examples

#### Local Variable Usage
```rust
//// test_local_var_usage.rs
fn main() {
    let x = 5;
    let y = 10;
    let z = x + y;  // 使用局部变量x和y
    
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

```yaml
name: LocalVariableUsage
relation:
  items:
  - type: UseVar
    to: Variable:'test_local_var_usage.main.x'
    loc: '4:13'
    from: Function:'test_local_var_usage.main'
  - type: UseVar
    to: Variable:'test_local_var_usage.main.y'
    loc: '4:17'
    from: Function:'test_local_var_usage.main'
  - type: UseVar
    to: Variable:'test_local_var_usage.main.x'
    loc: '6:34'
    from: Function:'test_local_var_usage.main'
  - type: UseVar
    to: Variable:'test_local_var_usage.main.y'
    loc: '6:37'
    from: Function:'test_local_var_usage.main'
  - type: UseVar
    to: Variable:'test_local_var_usage.main.z'
    loc: '6:40'
    from: Function:'test_local_var_usage.main'
```

#### Function Parameter Usage
```rust
//// test_parameter_usage.rs
fn add(a: i32, b: i32) -> i32 {
    a + b  // 使用参数a和b
}

fn main() {
    let result = add(3, 4);  // 使用参数3和4
    println!("Result: {}", result);
}
```

```yaml
name: FunctionParameterUsage
relation:
  items:
  - type: UseVar
    to: Variable:'test_parameter_usage.add.a'
    loc: '2:5'
    from: Function:'test_parameter_usage.add'
  - type: UseVar
    to: Variable:'test_parameter_usage.add.b'
    loc: '2:9'
    from: Function:'test_parameter_usage.add'
  - type: UseVar
    to: Variable:'test_parameter_usage.main.result'
    loc: '6:20'
    from: Function:'test_parameter_usage.main'
```

#### Struct Field Usage
```rust
//// test_struct_field_usage.rs
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()  // 使用字段x和y
    }
    
    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;  // 使用字段x
        self.y += dy;  // 使用字段y
    }
}

fn main() {
    let mut p = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", p.distance_from_origin());
    p.move_by(1.0, 1.0);
    println!("New coordinates: ({}, {})", p.x, p.y);  // 使用字段x和y
}
```

```yaml
name: StructFieldUsage
relation:
  items:
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.x'
    loc: '8:15'
    from: Function:'test_struct_field_usage.Point.distance_from_origin'
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.x'
    loc: '8:25'
    from: Function:'test_struct_field_usage.Point.distance_from_origin'
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.y'
    loc: '8:33'
    from: Function:'test_struct_field_usage.Point.distance_from_origin'
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.y'
    loc: '8:43'
    from: Function:'test_struct_field_usage.Point.distance_from_origin'
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.x'
    loc: '12:9'
    from: Function:'test_struct_field_usage.Point.move_by'
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.y'
    loc: '13:9'
    from: Function:'test_struct_field_usage.Point.move_by'
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.x'
    loc: '20:45'
    from: Function:'test_struct_field_usage.main'
  - type: UseVar
    to: Variable:'test_struct_field_usage.Point.y'
    loc: '20:50'
    from: Function:'test_struct_field_usage.main'
```

#### Static Variable Usage
```rust
//// test_static_var_usage.rs
static mut COUNTER: i32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;  // 使用静态变量COUNTER
        println!("Counter: {}", COUNTER);
    }
}

fn get_counter() -> i32 {
    unsafe {
        COUNTER  // 使用静态变量COUNTER
    }
}

fn main() {
    increment_counter();
    increment_counter();
    println!("Final counter: {}", get_counter());
}
```

```yaml
name: StaticVariableUsage
relation:
  items:
  - type: UseVar
    to: StaticItem:'test_static_var_usage.COUNTER'
    loc: '5:9'
    from: Function:'test_static_var_usage.increment_counter'
  - type: UseVar
    to: StaticItem:'test_static_var_usage.COUNTER'
    loc: '6:28'
    from: Function:'test_static_var_usage.increment_counter'
  - type: UseVar
    to: StaticItem:'test_static_var_usage.COUNTER'
    loc: '12:9'
    from: Function:'test_static_var_usage.get_counter'
```

#### Constant Usage
```rust
//// test_constant_usage.rs
const MAX_SIZE: usize = 100;
const PI: f64 = 3.14159;

fn calculate_circumference(radius: f64) -> f64 {
    2.0 * PI * radius  // 使用常量PI
}

fn create_buffer() -> [u8; MAX_SIZE] {
    [0; MAX_SIZE]  // 使用常量MAX_SIZE
}

fn main() {
    let circumference = calculate_circumference(5.0);
    let buffer = create_buffer();
    println!("Circumference: {}", circumference);
    println!("Buffer size: {}", buffer.len());
}
```

```yaml
name: ConstantUsage
relation:
  items:
  - type: UseVar
    to: Constant:'test_constant_usage.PI'
    loc: '5:14'
    from: Function:'test_constant_usage.calculate_circumference'
  - type: UseVar
    to: Constant:'test_constant_usage.MAX_SIZE'
    loc: '9:22'
    from: Function:'test_constant_usage.create_buffer'
  - type: UseVar
    to: Constant:'test_constant_usage.MAX_SIZE'
    loc: '10:9'
    from: Function:'test_constant_usage.create_buffer'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |