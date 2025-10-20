# Entity: Static Item

Static items are similar to constants but have a global storage lifetime and can be modified during program execution (with unsafe). They are stored in the program's data segment and have a fixed memory address.

## Supported Patterns

### name: StaticItemDefinition
### Syntax: StaticItemDefinition
```rust
static STATIC_NAME: Type = value;
static mut MUTABLE_STATIC: Type = value;
```

### Examples

#### Basic Static Item
```rust
static GLOBAL_COUNTER: i32 = 0;
static APPLICATION_NAME: &str = "My Application";

fn main() {
    println!("Counter: {}", GLOBAL_COUNTER);
    println!("App Name: {}", APPLICATION_NAME);
}
```

```yaml
name: BasicStaticItem
entity:
  type: StaticItem
  extra: false
  items:
  - type: StaticItem
    qualified: test_basic_static.GLOBAL_COUNTER
    name: GLOBAL_COUNTER
    loc: '1:8'
  - type: StaticItem
    qualified: test_basic_static.APPLICATION_NAME
    name: APPLICATION_NAME
    loc: '2:8'
```

#### Mutable Static Item
```rust
static mut ERROR_COUNT: i32 = 0;

fn record_error() {
    unsafe {
        ERROR_COUNT += 1;
        println!("Error count: {}", ERROR_COUNT);
    }
}

fn main() {
    record_error();
    record_error();
    
    unsafe {
        println!("Final error count: {}", ERROR_COUNT);
    }
}
```

```yaml
name: MutableStaticItem
entity:
  type: StaticItem
  extra: false
  items:
  - type: StaticItem
    qualified: test_mutable_static.ERROR_COUNT
    name: ERROR_COUNT
    loc: '1:12'
```

#### Static Reference
```rust
static HELLO_WORLD: &str = "Hello, World!";
static mut LAST_ERROR: &str = "No error";

fn main() {
    println!("{}", HELLO_WORLD);
    
    unsafe {
        LAST_ERROR = "File not found";
        println!("Last error: {}", LAST_ERROR);
    }
}
```

```yaml
name: StaticReference
entity:
  type: StaticItem
  extra: false
  items:
  - type: StaticItem
    qualified: test_static_reference.HELLO_WORLD
    name: HELLO_WORLD
    loc: '1:8'
  - type: StaticItem
    qualified: test_static_reference.LAST_ERROR
    name: LAST_ERROR
    loc: '2:12'
```
#### Module Static Item
```rust
mod config {
    pub static DATABASE_URL: &str = "localhost:5432";
    static mut CONNECTION_COUNT: i32 = 0;
    
    pub fn get_connection_count() -> i32 {
        unsafe { CONNECTION_COUNT }
    }
    
    pub fn increment_connections() {
        unsafe { CONNECTION_COUNT += 1; }
    }
}

fn main() {
    println!("Database URL: {}", config::DATABASE_URL);
    config::increment_connections();
    println!("Connections: {}", config::get_connection_count());
}
```

```yaml
name: ModuleStaticItem
entity:
  type: StaticItem
  extra: false
  items:
  - type: StaticItem
    qualified: test_module_static.config.DATABASE_URL
    name: DATABASE_URL
    loc: '2:12'
  - type: StaticItem
    qualified: test_module_static.config.CONNECTION_COUNT
    name: CONNECTION_COUNT
    loc: '3:16'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isMutable | Indicates whether the static item is mutable. | boolean | false |
| isPublic | Indicates whether the static item is publicly accessible. | boolean | false |