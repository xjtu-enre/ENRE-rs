# Relation: Modify

An entity modifies the value or state of another entity.

## Supported Patterns

### name: Modify
### Semantic:

### Examples

#### Variable Value Modification
```rust
//// test_variable_modification.rs
fn main() {
    let mut x = 5;
    x = 10;  // 修改变量x的值
    
    let mut y = 20;
    y += 5;  // 修改变量y的值
    
    let mut z = 30;
    z *= 2;  // 修改变量z的值
}
```

```yaml
name: VariableValueModification
relation:
  items:
  - type: Modify
    to: Variable:'test_variable_modification.main.x'
    loc: '3:4'
    from: Function:'test_variable_modification.main'
  - type: Modify
    to: Variable:'test_variable_modification.main.y'
    loc: '6:4'
    from: Function:'test_variable_modification.main'
  - type: Modify
    to: Variable:'test_variable_modification.main.z'
    loc: '9:4'
    from: Function:'test_variable_modification.main'
```
```

#### Array/Vector Element Modification
```rust
//// test_array_vector_modification.rs
fn main() {
    // 数组元素修改
    let mut arr = [1, 2, 3, 4, 5];
    arr[0] = 10;  // 修改数组元素
    arr[2] += 5;  // 修改数组元素
    
    // 向量元素修改
    let mut vec = vec![1, 2, 3, 4, 5];
    vec[0] = 10;  // 修改向量元素
    vec[2] *= 2;  // 修改向量元素
    
}
```

```yaml
name: ArrayVectorModification
relation:
  items:
  - type: Modify
    to: Variable:'test_array_vector_modification.main.arr'
    loc: '4:4'
    from: Function:'test_array_vector_modification.main'
  - type: Modify
    to: Variable:'test_array_vector_modification.main.arr'
    loc: '5:4'
    from: Function:'test_array_vector_modification.main'
  - type: Modify
    to: Variable:'test_array_vector_modification.main.vec'
    loc: '9:4'
    from: Function:'test_array_vector_modification.main'
  - type: Modify
    to: Variable:'test_array_vector_modification.main.vec'
    loc: '10:4'
    from: Function:'test_array_vector_modification.main'
  - type: Modify
    to: Variable:'test_array_vector_modification.main.vec'
    loc: '13:4'
    from: Function:'test_array_vector_modification.main'
  - type: Modify
    to: Variable:'test_array_vector_modification.main.vec'
    loc: '14:4'
    from: Function:'test_array_vector_modification.main'
```

#### Mutable Reference Modification
```rust
//// test_mutable_reference_modification.rs
fn modify_value(x: &mut i32) {
    *x = 42;  // 通过可变引用修改值
    *x += 1;  // 通过可变引用修改值
}

fn main() {
    let mut value = 0;
    modify_value(&mut value);  // 传递可变引用
}
```

```yaml
name: MutableReferenceModification
relation:
  items:
  - type: Modify
    to: Variable:'test_mutable_reference_modification.modify_value.x'
    loc: '2:5'
    from: Function:'test_mutable_reference_modification.modify_value'
  - type: Modify
    to: Variable:'test_mutable_reference_modification.modify_value.x'
    loc: '3:5'
    from: Function:'test_mutable_reference_modification.modify_value'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |