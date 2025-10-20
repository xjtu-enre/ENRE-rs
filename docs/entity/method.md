# Entity: Method

Methods provide concrete method definitions for traits or structs. They are defined within impl blocks and contain the actual code that implements the behavior defined in traits or adds functionality to structs.

## Supported Patterns

### name: MethodDefinition
### Syntax: MethodDefinition
```rust
impl struct_name {
    fn method_name(&self, parameters) -> ReturnType {
        // method
    }
}

impl trait_name for struct_name {
    fn method_name(&self, parameters) -> ReturnType {
        // method
    }
}
```

### Examples

#### Struct Method
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
    
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

```yaml
name: StructMethod
entity:
  type: Method
  extra: false
  items:
  - type: Method
    qualified: test_struct_method.Rectangle
    name: Rectangle
    loc: '7:1'
```

#### Trait Method
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}
```

```yaml
name: TraitMethod
entity:
  type: Method
  extra: false
  items:
  - type: Method
    qualified: test_trait_method.Drawable.for.Circle
    name: Circle
    loc: '9:1'
```

#### Associated Function Method
```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
    
    fn child(name: String) -> Person {
        Person { name, age: 0 }
    }
}
```

```yaml
name: AssociatedFunctionMethod
entity:
  type: Method
  extra: false
  items:
  - type: Method
    qualified: test_associated_function_method.Person
    name: Person
    loc: '6:1'
```

#### Default Method Override
```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

```yaml
name: DefaultMethodOverride
entity:
  type: Method
  extra: false
  items:
  - type: Method
    qualified: test_default_override.Summary.for.NewsArticle
    name: NewsArticle
    loc: '9:1'
```
## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isTraitImpl | Indicates whether the method is for a trait. | boolean | false |
| isGeneric | Indicates whether the method is generic. | boolean | false |
| isUnsafe | Indicates whether the method is unsafe. | boolean | false |