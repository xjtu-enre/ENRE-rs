# Entity: Enum

Enums provide a means of creating custom data types by defining a set of possible discrete values. Each variant can optionally carry other data. Enums are useful for representing data that can be one of several variants.

## Supported Patterns

### name: EnumDefinition
### Syntax: EnumDefinition
```rust
enum enum_name {
    Variant1,
    Variant2(data_type),
    Variant3 { field1: Type1, field2: Type2 },
    // ...
}
```

### Examples

#### Basic Enum Definition
```rust
enum Direction {
    North,
    South,
    East,
    West,
}

enum Status {
    Pending,
    Approved,
    Rejected,
}
```

```yaml
name: BasicEnumDefinition
entity:
  type: Enum
  extra: false
  items:
  - type: Enum
    qualified: test_basic_enum.Direction
    name: Direction
    loc: '1:6'
  - type: Enum
    qualified: test_basic_enum.Status
    name: Status
    loc: '8:6'
```

#### Enum with Associated Data
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

```yaml
name: EnumWithAssociatedData
entity:
  type: Enum
  extra: false
  items:
  - type: Enum
    qualified: test_enum_associated_data.Message
    name: Message
    loc: '1:6'
```

#### Enum with Explicit Values
```rust
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}
```

```yaml
name: EnumWithExplicitValues
entity:
  type: Enum
  extra: false
  items:
  - type: Enum
    qualified: test_enum_explicit_values.HttpStatus
    name: HttpStatus
    loc: '1:6'
```

#### Enum with Methods
```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}
```

```yaml
name: EnumWithMethods
entity:
  type: Enum
  extra: false
  items:
  - type: Enum
    qualified: test_enum_methods.Shape
    name: Shape
    loc: '1:6'
```

#### Nested Enum Definition
```rust
enum Outer {
    Variant1,
    InnerEnum(enum Inner {
        A,
        B,
    }),
}
```

```yaml
name: NestedEnumDefinition
entity:
  type: Enum
  extra: false
  items:
  - type: Enum
    qualified: test_nested_enum.Outer
    name: Outer
    loc: '1:6'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isGeneric | Indicates whether the enum is generic. | boolean | false |