# Relation: Inherit

Inheritance, where a trait can inherit from another trait.

## Supported Patterns

### name: Inherit
### Semantic:

### Examples

#### Trait Inherit Trait
```rust
//// test_trait_inherit.rs
trait Animal {
    fn speak(&self);
}

trait Dog: Animal {
    fn bark(&self);
}

struct Labrador;

impl Animal for Labrador {
    fn speak(&self) {
        println!("Labrador speaks");
    }
}

impl Dog for Labrador {
    fn bark(&self) {
        println!("Labrador barks");
    }
}

fn main() {
    let dog = Labrador;
    dog.speak();
    dog.bark();
}
```

```yaml
name: TraitInheritTrait
relation:
  items:
  - type: Inherit
    to: Trait:'test_trait_inherit.Animal'
    loc: '5:10'
    from: Trait:'test_trait_inherit.Dog'
```

#### Multiple Trait Inheritance
```rust
//// test_multiple_trait_inherit.rs
trait Walk {
    fn walk(&self);
}

trait Swim {
    fn swim(&self);
}

trait Amphibian: Walk + Swim {
    fn live_on_land_and_water(&self);
}

struct Frog;

impl Walk for Frog {
    fn walk(&self) {
        println!("Frog walks");
    }
}

impl Swim for Frog {
    fn swim(&self) {
        println!("Frog swims");
    }
}

impl Amphibian for Frog {
    fn live_on_land_and_water(&self) {
        println!("Frog lives on land and water");
    }
}

fn main() {
    let frog = Frog;
    frog.walk();
    frog.swim();
    frog.live_on_land_and_water();
}
```

```yaml
name: MultipleTraitInheritance
relation:
  items:
  - type: Inherit
    to: Trait:'test_multiple_trait_inherit.Walk'
    loc: '7:17'
    from: Trait:'test_multiple_trait_inherit.Amphibian'
  - type: Inherit
    to: Trait:'test_multiple_trait_inherit.Swim'
    loc: '7:24'
    from: Trait:'test_multiple_trait_inherit.Amphibian'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |