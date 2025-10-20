# Entity: Type Alias

Type aliases provide new names for existing types, improving code readability and simplicity. They are especially useful for complex types or when a more descriptive name would make the code clearer.

## Supported Patterns

### name: TypeAliasDefinition
### Syntax: TypeAliasDefinition
```rust
type alias_name = existing_type;
```

### Examples

#### Basic Type Alias
```rust
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: Kilometers = 5;
    let f: Thunk = Box::new(|| println!("hi"));
}
```

```yaml
name: BasicTypeAlias
entity:
  type: TypeAlias
  extra: false
  items:
  - type: TypeAlias
    qualified: test_basic_type_alias.Kilometers
    name: Kilometers
    loc: '1:6'
  - type: TypeAlias
    qualified: test_basic_type_alias.Thunk
    name: Thunk
    loc: '2:6'
```

#### Complex Type Alias
```rust
type Result<T> = std::result::Result<T, MyError>;
type MyMap = std::collections::HashMap<String, Vec<i32>>;

enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}
```

```yaml
name: ComplexTypeAlias
entity:
  type: TypeAlias
  extra: false
  items:
  - type: TypeAlias
    qualified: test_complex_type_alias.Result
    name: Result
    loc: '1:6'
  - type: TypeAlias
    qualified: test_complex_type_alias.MyMap
    name: MyMap
    loc: '2:6'
```


#### Module Type Alias
```rust
mod networking {
    type Port = u16;
    type Host = String;
    
    pub struct Server {
        host: Host,
        port: Port,
    }
}

fn main() {
    let server = networking::Server {
        host: String::from("localhost"),
        port: 8080,
    };
}
```

```yaml
name: ModuleTypeAlias
entity:
  type: TypeAlias
  extra: false
  items:
  - type: TypeAlias
    qualified: test_module_type_alias.networking.Port
    name: Port
    loc: '2:10'
  - type: TypeAlias
    qualified: test_module_type_alias.networking.Host
    name: Host
    loc: '3:10'
```

#### Public Type Alias
```rust
pub type PublicAlias = std::collections::HashMap<String, i32>;
type PrivateAlias = Vec<String>;

pub struct MyStruct {
    pub data: PublicAlias,
    private_data: PrivateAlias,
}
```

```yaml
name: PublicTypeAlias
entity:
  type: TypeAlias
  extra: false
  items:
  - type: TypeAlias
    qualified: test_public_type_alias.PublicAlias
    name: PublicAlias
    loc: '1:10'
  - type: TypeAlias
    qualified: test_public_type_alias.PrivateAlias
    name: PrivateAlias
    loc: '2:6'
```
## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isGeneric | Indicates whether the type alias is generic. | boolean | false |
| isPublic | Indicates whether the type alias is publicly accessible. | boolean | false |