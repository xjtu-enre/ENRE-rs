# Relation: Contain

Indicating that a file contains a module, or a module contains another module.

## Supported Patterns

### name: Contain
### Semantic:

### Examples

#### File Contains Module
```rust
//// test_file_contain_module.rs
mod my_module {
    pub fn function() {
        // ...
    }
}

fn main() {
    my_module::function();
}
```

```yaml
name: FileContainsModule
relation:
  items:
  - type: Contain
    to: Module:'test_file_contain_module.my_module'
    loc: '2:1'
    from: Module:'test_file_contain_module'
```

#### Module Contains Submodule
```rust
//// test_module_contain_submodule.rs
mod outer {
    pub mod inner {
        pub fn function() {
            // ...
        }
    }
    
    pub fn outer_function() {
        inner::function();
    }
}

fn main() {
    outer::inner::function();
}
```

```yaml
name: ModuleContainsSubmodule
relation:
  items:
  - type: Contain
    to: Module:'test_module_contain_submodule.outer.inner'
    loc: '3:9'
    from: Module:'test_module_contain_submodule.outer'
```

#### Nested Module Containment
```rust
//// test_nested_module_contain.rs
mod level1 {
    pub mod level2 {
        pub mod level3 {
            pub fn deep_function() {
                // ...
            }
        }
    }
    
    pub fn level1_function() {
        level2::level3::deep_function();
    }
}

fn main() {
    level1::level2::level3::deep_function();
}
```

```yaml
name: NestedModuleContainment
relation:
  items:
  - type: Contain
    to: Module:'test_nested_module_contain.level1.level2'
    loc: '3:9'
    from: Module:'test_nested_module_contain.level1'
  - type: Contain
    to: Module:'test_nested_module_contain.level1.level2.level3'
    loc: '4:17'
    from: Module:'test_nested_module_contain.level1.level2'
```

#### File Module Containment
```rust
//// test_file_module_contain.rs
// main.rs
mod database;
mod network;

fn main() {
    database::connect();
    network::send_data();
}
```

```rust
//// test_file_module_contain_database.rs
// database.rs
pub fn connect() {
    // ...
}
```

```rust
//// test_file_module_contain_network.rs
// network.rs
pub fn send_data() {
    // ...
}
```

```yaml
name: FileModuleContainment
relation:
  items:
  - type: Contain
    to: Module:'test_file_module_contain.database'
    loc: '2:1'
    from: Module:'test_file_module_contain'
  - type: Contain
    to: Module:'test_file_module_contain.network'
    loc: '3:1'
    from: Module:'test_file_module_contain'
```

#### Directory Module Containment
```rust
//// test_directory_module_contain.rs
// main.rs
mod graphics;

fn main() {
    graphics::render::draw();
}
```

```rust
//// test_directory_module_contain_graphics.rs
// graphics/mod.rs
pub mod render;

pub fn init() {
    // ...
}
```

```rust
//// test_directory_module_contain_graphics_render.rs
// graphics/render.rs
pub fn draw() {
    // ...
}
```

```yaml
name: DirectoryModuleContainment
relation:
  items:
  - type: Contain
    to: Module:'test_directory_module_contain.graphics'
    loc: '2:1'
    from: Module:'test_directory_module_contain'
  - type: Contain
    to: Module:'test_directory_module_contain.graphics.render'
    loc: '2:1'
    from: Module:'test_directory_module_contain.graphics'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |