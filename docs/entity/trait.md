# Entity: Trait

Traits are collections of methods that can be implemented by different types, similar to interfaces in other languages. They define shared behavior that can be used across multiple types, enabling polymorphism and code reuse.

## Supported Patterns

### name: TraitDefinition
### Syntax: TraitDefinition
```rust
trait trait_name {
    fn method_name(&self) -> ReturnType;
    fn associated_function() -> ReturnType;
    fn default_method(&self) -> ReturnType {
        // default implementation
    }
}
```

### Examples

#### Basic Trait Definition
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}
```

```yaml
name: BasicTraitDefinition
entity:
  type: Trait
  extra: false
  items:
  - type: Trait
    qualified: test_basic_trait.Drawable
    name: Drawable
    loc: '1:7'
```

#### Trait with Associated Function
```rust
trait Animal {
    fn new(name: String) -> Self;
    fn name(&self) -> &String;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn new(name: String) -> Dog {
        Dog { name }
    }
    
    fn name(&self) -> &String {
        &self.name
    }
    
    fn talk(&self) {
        println!("{} says woof!", self.name());
    }
}
```

```yaml
name: TraitWithAssociatedFunction
entity:
  type: Trait
  extra: false
  items:
  - type: Trait
    qualified: test_trait_associated_function.Animal
    name: Animal
    loc: '1:7'
```

#### Trait with Default Implementation
```rust
trait Summary {
    fn summarize(&self) -> String;
    
    fn summarize_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
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
    
    // 使用默认实现的 summarize_author
}
```

```yaml
name: TraitWithDefaultImplementation
entity:
  type: Trait
  extra: false
  items:
  - type: Trait
    qualified: test_trait_default.Summary
    name: Summary
    loc: '1:7'
```

#### Trait Object
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

fn render_all(shapes: Vec<Box<dyn Drawable>>) {
    for shape in shapes {
        shape.draw();
    }
}
```

```yaml
name: TraitObject
entity:
  type: Trait
  extra: false
  items:
  - type: Trait
    qualified: test_trait_object.Drawable
    name: Drawable
    loc: '1:7'
```

#### Inherited Trait
```rust
trait Shape {
    fn area(&self) -> f64;
}

trait ColoredShape: Shape {
    fn color(&self) -> String;
    
    fn describe(&self) -> String {
        format!("A {} shape with area {}", self.color(), self.area())
    }
}

struct ColoredRectangle {
    width: f64,
    height: f64,
    color: String,
}

impl Shape for ColoredRectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl ColoredShape for ColoredRectangle {
    fn color(&self) -> String {
        self.color.clone()
    }
}
```

```yaml
name: InheritedTrait
entity:
  type: Trait
  extra: false
  items:
  - type: Trait
    qualified: test_inherited_trait.Shape
    name: Shape
    loc: '1:7'
  - type: Trait
    qualified: test_inherited_trait.ColoredShape
    name: ColoredShape
    loc: '5:7'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isGeneric | Indicates whether the trait is generic. | boolean | false |
| isUnsafe | Indicates whether the trait is unsafe. | boolean | false |