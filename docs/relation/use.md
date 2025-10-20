# Relation: Use

Using the use keyword to import modules, types, functions, or constants into the current scope.

## Supported Patterns

### name: Use
### Semantic:

### Examples

#### Basic Module Use
```rust
//// test_basic_use.rs
mod module_a {
    pub fn function_a() {
        // ...
    }
    
    pub struct StructA;
}

use module_a::function_a;
use module_a::StructA;

fn main() {
    function_a();
    let _s = StructA;
}
```

```yaml
name: BasicModuleUse
relation:
  items:
  - type: Use
    to: Function:'test_basic_use.module_a.function_a'
    loc: '10:4'
    from: Module:'test_basic_use'
  - type: Use
    to: Struct:'test_basic_use.module_a.StructA'
    loc: '11:4'
    from: Module:'test_basic_use'
```

#### Selective Use
```rust
//// test_selective_use.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
    
    pub const PI: f64 = 3.14159;
}

use math::{add, PI};

fn main() {
    let result = add(2, 3);
    println!("Pi: {}", PI);
}
```

```yaml
name: SelectiveUse
relation:
  items:
  - type: Use
    to: Function:'test_selective_use.math.add'
    loc: '12:11'
    from: Module:'test_selective_use'
  - type: Use
    to: Constant:'test_selective_use.math.PI'
    loc: '12:16'
    from: Module:'test_selective_use'
```

#### Renaming Use
```rust
//// test_renaming_use.rs
mod graphics {
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }
    
    pub fn draw_point(p: Point) {
        // ...
    }
}

use graphics::Point as GPoint;
use graphics::draw_point as draw;

fn main() {
    let p = GPoint { x: 1.0, y: 2.0 };
    draw(p);
}
```

```yaml
name: RenamingUse
relation:
  items:
  - type: Use
    to: Struct:'test_renaming_use.graphics.Point'
    loc: '11:19'
    from: Module:'test_renaming_use'
  - type: Use
    to: Function:'test_renaming_use.graphics.draw_point'
    loc: '12:24'
    from: Module:'test_renaming_use'
```

#### Nested Module Use
```rust
//// test_nested_use.rs
mod outer {
    pub mod inner {
        pub fn function() {
            // ...
        }
        
        pub mod deeper {
            pub struct Data;
        }
    }
}

use outer::inner::function;
use outer::inner::deeper::Data;

fn main() {
    function();
    let _d = Data;
}
```

```yaml
name: NestedModuleUse
relation:
  items:
  - type: Use
    to: Function:'test_nested_use.outer.inner.function'
    loc: '12:4'
    from: Module:'test_nested_use'
  - type: Use
    to: Struct:'test_nested_use.outer.inner.deeper.Data'
    loc: '13:4'
    from: Module:'test_nested_use'
```

#### Glob Use
```rust
//// test_glob_use.rs
mod utils {
    pub fn func_a() {}
    pub fn func_b() {}
    pub const CONSTANT: i32 = 42;
    pub struct StructA;
}

use utils::*;

fn main() {
    func_a();
    func_b();
    let _s = StructA;
    println!("Constant: {}", CONSTANT);
}
```

```yaml
name: GlobUse
relation:
  items:
  - type: Use
    to: Module:'test_glob_use.utils'
    loc: '9:4'
    from: Module:'test_glob_use'
```
## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |