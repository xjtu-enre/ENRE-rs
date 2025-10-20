# Entity: Struct

Structs provide a means of creating custom data types by grouping related data together. They are similar to classes in other languages but without inheritance. Structs can have methods implemented via impl blocks.

## Supported Patterns

### name: StructDefinition
### Syntax: StructDefinition
```rust
struct struct_name {
    field1: Type1,
    field2: Type2,
    // ...
}
```

### Examples

#### Basic Struct Definition
```rust
struct Person {
    name: String,
    age: u32,
}

struct Point {
    x: f64,
    y: f64,
}
```

```yaml
name: BasicStructDefinition
entity:
  type: Struct
  extra: false
  items:
  - type: Struct
    qualified: test_basic_struct.Person
    name: Person
    loc: '1:8'
  - type: Struct
    qualified: test_basic_struct.Point
    name: Point
    loc: '6:8'
```

#### Tuple Struct Definition
```rust
struct Color(u8, u8, u8);
struct Point(f64, f64);
```

```yaml
name: TupleStructDefinition
entity:
  type: Struct
  extra: false
  items:
  - type: Struct
    qualified: test_tuple_struct.Color
    name: Color
    loc: '1:8'
  - type: Struct
    qualified: test_tuple_struct.Point
    name: Point
    loc: '2:8'
```

#### Unit Struct Definition
```rust
struct Empty;
struct Null;
```

```yaml
name: UnitStructDefinition
entity:
  type: Struct
  extra: false
  items:
  - type: Struct
    qualified: test_unit_struct.Empty
    name: Empty
    loc: '1:8'
  - type: Struct
    qualified: test_unit_struct.Null
    name: Null
    loc: '2:8'
```

#### Struct with Methods
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
}
```

```yaml
name: StructWithMethods
entity:
  type: Struct
  extra: false
  items:
  - type: Struct
    qualified: test_struct_methods.Rectangle
    name: Rectangle
    loc: '1:8'
```

#### Nested Struct Definition
```rust
struct Address {
    street: String,
    city: String,
}

struct Person {
    name: String,
    address: Address,  // 嵌套结构体
}
```

```yaml
name: NestedStructDefinition
entity:
  type: Struct
  extra: false
  items:
  - type: Struct
    qualified: test_nested_struct.Address
    name: Address
    loc: '1:8'
  - type: Struct
    qualified: test_nested_struct.Person
    name: Person
    loc: '6:8'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isGeneric | Indicates whether the struct is generic. | boolean | false |