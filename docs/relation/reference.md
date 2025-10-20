# Relation: Reference

An entity references another entity through a reference (&) or mutable reference (&mut).

## Supported Patterns

### name: Reference
### Semantic:

### Examples

#### Immutable Reference
```rust
//// test_immutable_reference.rs
fn main() {
    let x = 5;
    let y = &x;  // 不可变引用
    
    println!("x: {}, y: {}", x, *y);
    
    let z = &x;  // 另一个不可变引用
    println!("z: {}", *z);
}
```

```yaml
name: ImmutableReference
relation:
  items:
  - type: Reference
    to: Variable:'test_immutable_reference.main.x'
    loc: '3:13'
    from: Function:'test_immutable_reference.main'
  - type: Reference
    to: Variable:'test_immutable_reference.main.x'
    loc: '7:13'
    from: Function:'test_immutable_reference.main'
```

#### Mutable Reference
```rust
//// test_mutable_reference.rs
fn main() {
    let mut x = 5;
    let y = &mut x;  // 可变引用
    
    *y += 1;  // 通过可变引用修改值
    println!("x: {}", x);
    
    let z = &mut x;  // 另一个可变引用
    *z *= 2;  // 通过可变引用修改值
    println!("x: {}", x);
}
```

```yaml
name: MutableReference
relation:
  items:
  - type: Reference
    to: Variable:'test_mutable_reference.main.x'
    loc: '3:13'
    from: Function:'test_mutable_reference.main'
  - type: Reference
    to: Variable:'test_mutable_reference.main.x'
    loc: '8:13'
    from: Function:'test_mutable_reference.main'
```

#### Function Parameter Reference
```rust
//// test_function_parameter_reference.rs
fn calculate_length(s: &String) -> usize {
    s.len()  // 使用不可变引用
}

fn change_string(s: &mut String) {
    s.push_str(", world!");  // 通过可变引用修改
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 传递不可变引用
    println!("The length of '{}' is {}.", s1, len);
    
    let mut s2 = String::from("hello");
    change_string(&mut s2);  // 传递可变引用
    println!("Modified string: {}", s2);
}
```

```yaml
name: FunctionParameterReference
relation:
  items:
  - type: Reference
    to: Variable:'test_function_parameter_reference.calculate_length.s'
    loc: '2:20'
    from: Function:'test_function_parameter_reference.calculate_length'
  - type: Reference
    to: Variable:'test_function_parameter_reference.change_string.s'
    loc: '5:21'
    from: Function:'test_function_parameter_reference.change_string'
  - type: Reference
    to: Variable:'test_function_parameter_reference.main.s1'
    loc: '10:31'
    from: Function:'test_function_parameter_reference.main'
  - type: Reference
    to: Variable:'test_function_parameter_reference.main.s2'
    loc: '14:20'
    from: Function:'test_function_parameter_reference.main'
```

#### Struct Field Reference
```rust
//// test_struct_field_reference.rs
struct Point {
    x: f64,
    y: f64,
}

fn distance_from_origin(p: &Point) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()  // 通过引用访问字段
}

fn move_point(p: &mut Point, dx: f64, dy: f64) {
    p.x += dx;  // 通过可变引用修改字段
    p.y += dy;  // 通过可变引用修改字段
}

fn main() {
    let mut point = Point { x: 3.0, y: 4.0 };
    let dist = distance_from_origin(&point);  // 传递不可变引用
    println!("Distance: {}", dist);
    
    move_point(&mut point, 1.0, 1.0);  // 传递可变引用
    println!("New coordinates: ({}, {})", point.x, point.y);
}
```

```yaml
name: StructFieldReference
relation:
  items:
  - type: Reference
    to: Variable:'test_struct_field_reference.distance_from_origin.p'
    loc: '5:27'
    from: Function:'test_struct_field_reference.distance_from_origin'
  - type: Reference
    to: Variable:'test_struct_field_reference.move_point.p'
    loc: '9:17'
    from: Function:'test_struct_field_reference.move_point'
  - type: Reference
    to: Variable:'test_struct_field_reference.main.point'
    loc: '14:34'
    from: Function:'test_struct_field_reference.main'
  - type: Reference
    to: Variable:'test_struct_field_reference.main.point'
    loc: '18:16'
    from: Function:'test_struct_field_reference.main'
```

#### Array/Vector Element Reference
```rust
//// test_array_vector_reference.rs
fn main() {
    // 数组元素引用
    let arr = [1, 2, 3, 4, 5];
    let first = &arr[0];  // 不可变引用数组元素
    let second = &arr[1];  // 不可变引用数组元素
    
    println!("First: {}, Second: {}", *first, *second);
    
    // 向量元素引用
    let mut vec = vec![1, 2, 3, 4, 5];
    let first_mut = &mut vec[0];  // 可变引用向量元素
    *first_mut += 10;  // 通过可变引用修改
    
    let first_imm = &vec[0];  // 不可变引用向量元素
    println!("Modified first: {}", *first_imm);
}
```

```yaml
name: ArrayVectorReference
relation:
  items:
  - type: Reference
    to: Variable:'test_array_vector_reference.main.arr'
    loc: '4:17'
    from: Function:'test_array_vector_reference.main'
  - type: Reference
    to: Variable:'test_array_vector_reference.main.arr'
    loc: '5:19'
    from: Function:'test_array_vector_reference.main'
  - type: Reference
    to: Variable:'test_array_vector_reference.main.vec'
    loc: '11:21'
    from: Function:'test_array_vector_reference.main'
  - type: Reference
    to: Variable:'test_array_vector_reference.main.vec'
    loc: '15:19'
    from: Function:'test_array_vector_reference.main'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |