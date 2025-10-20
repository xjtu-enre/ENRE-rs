# Entity: Module

Modules provide a means of organizing code and controlling visibility. Creating a new module creates a new namespace, allowing items to be grouped logically and their visibility to be controlled with pub keyword.

## Supported Patterns

### name: ModuleDefinition
### Syntax: ModuleDefinition

### Examples
```rust
mod module_name {
    // items
}
```

#### Global Module Definition
```rust
//// test_global_module.rs
mod my_module {
    pub fn public_function() {
        // ...
    }
    
    fn private_function() {
        // ...
    }
}
```

```yaml
name: GlobalModuleDefinition
entity:
  type: Module
  extra: false
  items:
  - type: Module
    qualified: test_global_module.my_module
    name: my_module
    loc: '1:5'
```

#### Nested Module Definition
```rust
//// test_nested_module.rs
mod outer {
    pub mod inner {
        pub fn function() {
            // ...
        }
    }
    
    fn private_mod() {
        // ...
    }
}

fn function() {
    mod local_module {
        // ...
    }
}
```

```yaml
name: NestedModuleDefinition
entity:
  type: Module
  extra: false
  items:
  - type: Module
    qualified: test_nested_module.outer
    name: outer
    loc: '1:5'
  - type: Module
    qualified: test_nested_module.outer.inner
    name: inner
    loc: '2:9'
  - type: Module
    qualified: test_nested_module.function.local_module
    name: local_module
    loc: '12:9'
```

#### Module in Separate File
```rust
//// test_module_file.rs
mod database;  // 在 database.rs 中定义
mod network {
    mod tcp;   // 在 network/tcp.rs 中定义
}
```

```yaml
name: ModuleInSeparateFile
entity:
  type: Module
  extra: false
  items:
  - type: Module
    qualified: test_module_file.database
    name: database
    loc: '1:5'
  - type: Module
    qualified: test_module_file.network
    name: network
    loc: '2:5'
  - type: Module
    qualified: test_module_file.network.tcp
    name: tcp
    loc: '3:9'
```

#### Abstract Module Pattern (Traits)
```rust
//// test_abstract_module.rs
mod drawable {
    pub trait Drawable {
        fn draw(&self);
    }
}

mod shapes {
    use crate::drawable::Drawable;
    
    pub struct Circle {
        radius: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) {
            // ...
        }
    }
}
```

```yaml
name: AbstractModulePattern
entity:
  type: Module
  extra: false
  items:
  - type: Module
    qualified: test_abstract_module.drawable
    name: drawable
    loc: '1:5'
  - type: Module
    qualified: test_abstract_module.shapes
    name: shapes
    loc: '8:5'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isPublic | Indicates whether the module is publicly accessible. | boolean | false |