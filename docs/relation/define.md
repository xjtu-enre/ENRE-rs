# Relation: Define

Specifying the location where entities (functions, structs, enums, constants, modules, etc.) are defined.

## Supported Patterns

### name: Define
### Semantic:

### Examples

#### Function Definition
```rust
//// test_function_definition.rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(3, 4);
    println!("Result: {}", result);
}
```

```yaml
name: FunctionDefinition
relation:
  items:
  - type: Define
    to: Function:'test_function_definition.add'
    loc: '1:4'
    from: Module:'test_function_definition'
```

#### Struct Definition
```rust
//// test_struct_definition.rs
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn main() {
    let p = Point::new(1.0, 2.0);
}
```

```yaml
name: StructDefinition
relation:
  items:
  - type: Define
    to: Struct:'test_struct_definition.Point'
    loc: '1:8'
    from: Module:'test_struct_definition'
```

#### Enum Definition
```rust
//// test_enum_definition.rs
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let favorite_color = Color::Red;
}
```

```yaml
name: EnumDefinition
relation:
  items:
  - type: Define
    to: Enum:'test_enum_definition.Color'
    loc: '1:6'
    from: Module:'test_enum_definition'
```

#### Constant Definition
```rust
//// test_constant_definition.rs
const MAX_SIZE: usize = 100;
const PI: f64 = 3.14159;

fn main() {
    let buffer = [0; MAX_SIZE];
    println!("Pi: {}", PI);
}
```

```yaml
name: ConstantDefinition
relation:
  items:
  - type: Define
    to: Constant:'test_constant_definition.MAX_SIZE'
    loc: '1:7'
    from: Module:'test_constant_definition'
  - type: Define
    to: Constant:'test_constant_definition.PI'
    loc: '2:7'
    from: Module:'test_constant_definition'
```

#### Module Definition
```rust
//// test_module_definition.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let sum = math::add(5, 3);
    let difference = math::subtract(5, 3);
}
```

```yaml
name: ModuleDefinition
relation:
  items:
  - type: Define
    to: Module:'test_module_definition.math'
    loc: '1:5'
    from: Module:'test_module_definition'
```

#### Trait Definition
```rust
//// test_trait_definition.rs
trait Drawable {
    fn draw(&self);
}

struct Circle;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

fn main() {
    let c = Circle;
    c.draw();
}
```

```yaml
name: TraitDefinition
relation:
  items:
  - type: Define
    to: Trait:'test_trait_definition.Drawable'
    loc: '1:7'
    from: Module:'test_trait_definition'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |