# partial_application
The `partial!` macro allows for partial application of a function.  

`partial!(some_fn => arg0, _, arg2, _)` returns the closure `|x1, x3| some_fn(arg0, x1, arg2, x3)`.  
Move closures are created by adding `move` in front of the function: `partial!(move ..)`

```rust
use partial_application::partial;

// When you're using the 2015 edition of Rust, you need to import the macro like this
#[macro_use]
extern crate partial_application;

fn foo(a: i32, b: i32, c: i32, d: i32, mul: i32, off: i32) -> i32 {
    (a + b*b + c.pow(3) + d.pow(4)) * mul - off
}

fn main() {
    let bar = partial!(foo => _, _, 10, 42, 10, 10);
    assert_eq!(
        foo(15, 15, 10, 42, 10, 10),
        bar(15, 15)
    );
}
```
