# Entity: Associated Item

Associated items are items associated with a type, such as methods or constants, typically defined within impl blocks. They provide a way to group functionality with the types they operate on.

## Supported Patterns

### name: AssociatedItemDefinition
### Syntax: AssociatedItemDefinition
```rust
impl type_name {
    const ASSOCIATED_CONST: Type = value;
    type AssociatedType = Type;
    fn associated_function(parameters) -> ReturnType {
        // implementation
    }
}
```

### Examples

#### Associated Function
```rust
//// test_associated_function.rs
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

fn main() {
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::origin();
}
```

```yaml
name: AssociatedFunction
entity:
  type: AssociatedItem
  extra: false
  items:
  - type: AssociatedItem
    qualified: test_associated_function.Point.new
    name: new
    loc: '8:8'
  - type: AssociatedItem
    qualified: test_associated_function.Point.origin
    name: origin
    loc: '12:8'
```

#### Associated Constant
```rust
//// test_associated_constant.rs
struct Circle {
    radius: f64,
}

impl Circle {
    const PI: f64 = 3.14159;
    const DEFAULT_RADIUS: f64 = 1.0;
    
    fn area(&self) -> f64 {
        Self::PI * self.radius * self.radius
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    println!("Area: {}", circle.area());
    println!("PI: {}", Circle::PI);
}
```

```yaml
name: AssociatedConstant
entity:
  type: AssociatedItem
  extra: false
  items:
  - type: AssociatedItem
    qualified: test_associated_constant.Circle.PI
    name: PI
    loc: '7:11'
  - type: AssociatedItem
    qualified: test_associated_constant.Circle.DEFAULT_RADIUS
    name: DEFAULT_RADIUS
    loc: '8:11'
```

#### Associated Type
```rust
//// test_associated_type.rs
trait Graph {
    type Node;
    type Edge;
    
    fn has_edge(&self, &Self::Node, &Self::Node) -> bool;
    fn edges(&self, &Self::Node) -> Vec<Self::Edge>;
}

struct MyGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl Graph for MyGraph {
    type Node = String;
    type Edge = (usize, usize);
    
    fn has_edge(&self, a: &Self::Node, b: &Self::Node) -> bool {
        // implementation
        false
    }
    
    fn edges(&self, node: &Self::Node) -> Vec<Self::Edge> {
        // implementation
        Vec::new()
    }
}
```

```yaml
name: AssociatedType
entity:
  type: AssociatedItem
  extra: false
  items:
  - type: AssociatedItem
    qualified: test_associated_type.Graph.Node
    name: Node
    loc: '2:10'
  - type: AssociatedItem
    qualified: test_associated_type.Graph.Edge
    name: Edge
    loc: '3:10'
  - type: AssociatedItem
    qualified: test_associated_type.MyGraph.Node
    name: Node
    loc: '15:10'
  - type: AssociatedItem
    qualified: test_associated_type.MyGraph.Edge
    name: Edge
    loc: '16:10'
```
## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isGeneric | Indicates whether the associated item is generic. | boolean | false |
| isDefault | Indicates whether the associated item has a default implementation. | boolean | false |