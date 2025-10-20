# Entity: Union

Unions are data structures that allow multiple types to share the same memory location. In Rust, unions provide safe access to their fields through explicit tagging and unsafe operations, preventing undefined behavior.

## Supported Patterns

### name: UnionDefinition
### Syntax: UnionDefinition
```rust
union union_name {
    field1: Type1,
    field2: Type2,
    // ...
}
```

### Examples

#### Basic Union Definition
```rust
union MyUnion {
    i: i32,
    f: f32,
    c: char,
}

fn main() {
    let u = MyUnion { i: 10 };
    // Only unsafe access is allowed
    unsafe {
        println!("Integer: {}", u.i);
        println!("Float: {}", u.f);
        println!("Char: {}", u.c as u8);
    }
}
```

```yaml
name: BasicUnionDefinition
entity:
  type: Union
  extra: false
  items:
  - type: Union
    qualified: test_basic_union.MyUnion
    name: MyUnion
    loc: '1:7'
```


#### Generic Union
```rust
union GenericUnion<T: Copy> {
    value: T,
    bytes: [u8; std::mem::size_of::<T>()],
}

fn main() {
    let u32_union = GenericUnion { value: 0x12345678u32 };
    let f64_union = GenericUnion { value: 3.14159f64 };
    
    unsafe {
        println!("u32 value: 0x{:x}", u32_union.value);
        println!("u32 bytes: {:?}", u32_union.bytes);
        
        println!("f64 value: {}", f64_union.value);
        println!("f64 bytes: {:?}", f64_union.bytes);
    }
}
```

```yaml
name: GenericUnion
entity:
  type: Union
  extra: false
  items:
  - type: Union
    qualified: test_generic_union.GenericUnion
    name: GenericUnion
    loc: '1:7'
```

#### Union with Complex Types
```rust
union ComplexUnion {
    array: [i32; 4],
    tuple: (i32, i32, i32, i32),
    struct_val: [i32; 4], // Same layout as array
}

fn main() {
    let u = ComplexUnion { array: [1, 2, 3, 4] };
    
    unsafe {
        println!("Array: {:?}", u.array);
        println!("Tuple: {:?}", u.tuple);
        println!("Struct: {:?}", u.struct_val);
    }
}
```

```yaml
name: UnionWithComplexTypes
entity:
  type: Union
  extra: false
  items:
  - type: Union
    qualified: test_union_complex_types.ComplexUnion
    name: ComplexUnion
    loc: '1:7'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isGeneric | Indicates whether the union is generic. | boolean | false |