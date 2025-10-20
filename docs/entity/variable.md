# Entity: Variable

Variables are named storage locations in memory that can hold different values over time. In Rust, variables are immutable by default, but can be made mutable with the mut keyword.

## Supported Patterns

### name: VariableDefinition
### Syntax: VariableDefinition
```rust
let variable_name = value;
let mut mutable_variable = value;
let variable_name: Type = value;
```

### Examples

#### Basic Variable Declaration
```rust
fn main() {
    let x = 5;
    let y: i32 = 10;
    let z = "Hello, world!";
    
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

```yaml
name: BasicVariableDeclaration
entity:
  type: Variable
  extra: false
  items:
  - type: Variable
    qualified: test_basic_variable.main.x
    name: x
    loc: '2:9'
  - type: Variable
    qualified: test_basic_variable.main.y
    name: y
    loc: '3:9'
  - type: Variable
    qualified: test_basic_variable.main.z
    name: z
    loc: '4:9'
```

#### Mutable Variable
```rust
fn main() {
    let mut counter = 0;
    println!("Counter: {}", counter);
    
    counter += 1;
    println!("Counter: {}", counter);
    
    counter = 10;
    println!("Counter: {}", counter);
}
```

```yaml
name: MutableVariable
entity:
  type: Variable
  extra: false
  items:
  - type: Variable
    qualified: test_mutable_variable.main.counter
    name: counter
    loc: '2:13'
```

#### Variable Shadowing
```rust
fn main() {
    let x = 5;
    let x = x + 1;  // x is now 6
    let x = x * 2;  // x is now 12
    
    println!("The value of x is: {}", x);
    
    // Shadowing with different type
    let spaces = "   ";
    let spaces = spaces.len();  // Now spaces is usize
    
    println!("Spaces: {}", spaces);
}
```

```yaml
name: VariableShadowing
entity:
  type: Variable
  extra: false
  items:
  - type: Variable
    qualified: test_variable_shadowing.main.x
    name: x
    loc: '2:9'
  - type: Variable
    qualified: test_variable_shadowing.main.spaces
    name: spaces
    loc: '10:9'
```

#### Variable in Pattern Matching
```rust
fn main() {
    let some_value = Some(5);
    
    match some_value {
        Some(x) => println!("Got a value: {}", x),
        None => println!("Got nothing"),
    }
    
    let (a, b, c) = (1, 2, 3);
    println!("a: {}, b: {}, c: {}", a, b, c);
    
    let Point { x, y } = Point { x: 1, y: 2 };
    println!("x: {}, y: {}", x, y);
}

struct Point {
    x: i32,
    y: i32,
}
```

```yaml
name: VariableInPatternMatching
entity:
  type: Variable
  extra: false
  items:
  - type: Variable
    qualified: test_variable_pattern_matching.main.some_value
    name: some_value
    loc: '2:9'
  - type: Variable
    qualified: test_variable_pattern_matching.main.x
    name: x
    loc: '5:14'
  - type: Variable
    qualified: test_variable_pattern_matching.main.a
    name: a
    loc: '9:10'
  - type: Variable
    qualified: test_variable_pattern_matching.main.b
    name: b
    loc: '9:13'
  - type: Variable
    qualified: test_variable_pattern_matching.main.c
    name: c
    loc: '9:16'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isMutable | Indicates whether the variable is mutable. | boolean | false |
| isStatic | Indicates whether the variable is a static (global) variable. | boolean | false |