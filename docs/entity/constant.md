# Entity: Constant

Constants are immutable values that must be computed at compile time and defined at declaration. They are similar to immutable static variables but with some differences in how they're used and optimized.

## Supported Patterns

### name: ConstantDefinition
### Syntax: ConstantDefinition
```rust
const CONSTANT_NAME: Type = value;
```

### Examples

#### Basic Constant Definition
```rust
const MAX_POINTS: u32 = 100_000;
const DEFAULT_NAME: &str = "Unknown";
```

```yaml
name: BasicConstantDefinition
entity:
  type: Constant
  extra: false
  items:
  - type: Constant
    qualified: test_basic_constant.MAX_POINTS
    name: MAX_POINTS
    loc: '1:7'
  - type: Constant
    qualified: test_basic_constant.DEFAULT_NAME
    name: DEFAULT_NAME
    loc: '2:7'
```

#### Constant with Expression
```rust
const HOUR_IN_SECONDS: u32 = 60 * 60;
const PI_APPROX: f64 = 22.0 / 7.0;
const IS_ENABLED: bool = true;
```

```yaml
name: ConstantWithExpression
entity:
  type: Constant
  extra: false
  items:
  - type: Constant
    qualified: test_constant_expression.HOUR_IN_SECONDS
    name: HOUR_IN_SECONDS
    loc: '1:7'
  - type: Constant
    qualified: test_constant_expression.PI_APPROX
    name: PI_APPROX
    loc: '2:7'
  - type: Constant
    qualified: test_constant_expression.IS_ENABLED
    name: IS_ENABLED
    loc: '3:7'
```

#### Module Constant
```rust
//// test_module_constant.rs
mod config {
    pub const DATABASE_URL: &str = "localhost:5432";
    const PRIVATE_KEY: &str = "secret";
}

fn main() {
    println!("Database URL: {}", config::DATABASE_URL);
}
```

```yaml
name: ModuleConstant
entity:
  type: Constant
  extra: false
  items:
  - type: Constant
    qualified: test_module_constant.config.DATABASE_URL
    name: DATABASE_URL
    loc: '2:11'
  - type: Constant
    qualified: test_module_constant.config.PRIVATE_KEY
    name: PRIVATE_KEY
    loc: '3:11'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
| isPublic | Indicates whether the constant is publicly accessible. | boolean | false |