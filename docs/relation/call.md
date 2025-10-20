# Relation: Call

Calling a callable object indicates a call dependency.

## Supported Patterns

### name: Call
### Semantic:

### Examples

#### Global Function Call
```rust
//// test_global_function_call.rs

fn func1() {
    func1();  // Recursive call
    return;
}

fn main() {
    func1();  // Call from main
}
```

```yaml
name: GlobalFunctionCall
relation:
  items:
  - type: Call
    to: Function:'test_global_function_call.func1'
    loc: '3:4'
    from: Function:'test_global_function_call.func1'
  - type: Call
    to: Function:'test_global_function_call.func1'
    loc: '8:4'
    from: Function:'test_global_function_call.main'
```

#### Method Call
```rust
//// test_method_call.rs

struct ClassA;

impl ClassA {
    fn method(&self) {
        // ...
    }
}

struct ClassB;

impl ClassB {
    fn method(&self) {
        // ...
    }
}

fn main() {
    let instance = ClassA;
    instance.method();  // Method call
}
```

```yaml
name: MethodCall
relation:
  items:
  - type: Call
    to: Function:'test_method_call.ClassA.method'
    loc: '18:13'
    from: Function:'test_method_call.main'
```

#### Local Function Call
```rust
//// test_local_call.rs

fn func() {
    fn inner() {
        fn inner_inner() {
            func();  // Call to outer function
        }
        func();  // Call to outer function
        inner_inner();  // Call to nested function
    }
    inner();  // Call to nested function
}
```

```yaml
name: LocalFunctionCall
relation:
  items:
  - type: Call
    to: Function:'test_local_call.func'
    loc: '5:12'
    from: Function:'test_local_call.func.inner.inner_inner'
  - type: Call
    to: Function:'test_local_call.func'
    loc: '7:8'
    from: Function:'test_local_call.func.inner'
  - type: Call
    to: Function:'test_local_call.func.inner.inner_inner'
    loc: '8:8'
    from: Function:'test_local_call.func.inner'
  - type: Call
    to: Function:'test_local_call.func.inner'
    loc: '10:4'
    from: Function:'test_local_call.func'
```

#### First Order Function Call
```rust
//// test_first_order_func_call.rs
fn foo() {
    // ...
}

fn acceptor(f: fn()) {
    f();  // Call function parameter
}

fn main() {
    acceptor(foo);  // Pass function as parameter
}
```

```yaml
name: FirstOrderFunctionCall
relation:
  items:
  - type: Call
    to: Variable:'test_first_order_func_call.acceptor.f'
    from: Function:'test_first_order_func_call.acceptor'
    loc: '5:4'
  - type: Call
    to: Function:'test_first_order_func_call.foo'
    from: Function:'test_first_order_func_call.acceptor'
    loc: '5:4'
  - type: Call
    to: Function:'test_first_order_func_call.acceptor'
    from: Function:'test_first_order_func_call.main'
    loc: '9:4'
```

#### Closure Call
```rust
//// test_closure_call.rs
fn main() {
    let closure = |x: i32| x + 1;
    let result = closure(5);  // Call closure
    
    let stored_closure = closure;
    let result2 = stored_closure(10);  // Call stored closure
}
```

```yaml
name: ClosureCall
relation:
  items:
  - type: Call
    to: Variable:'test_closure_call.main.closure'
    from: Function:'test_closure_call.main'
    loc: '3:17'
  - type: Call
    to: Variable:'test_closure_call.main.stored_closure'
    from: Function:'test_closure_call.main'
    loc: '6:18'
```

#### Trait Method Call
```rust
//// test_trait_method_call.rs
trait Drawable {
    fn draw(&self);
}

struct Circle;

impl Drawable for Circle {
    fn draw(&self) {
        // ...
    }
}

fn main() {
    let circle = Circle;
    circle.draw();  // Trait method call
}
```

```yaml
name: TraitMethodCall
relation:
  items:
  - type: Call
    to: Function:'test_trait_method_call.Circle.draw'
    loc: '14:11'
    from: Function:'test_trait_method_call.main'
```

## Properties

| Name | Description | Type | Default |
|------|-------------|------|---------|
|      |             |      |         |