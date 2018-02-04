# partial_application
This crate contains the `partial!` macro which allows partial application of a function.
Calling `partial!(some_fn(arg0, _, arg2, _))` will return the closure
`|x1, x3| some_fn(arg0, x1, arg1, x3)`.
The function call parentheses are optional:
`partial!(some_fn arg0, _, arg2, _)`

Move closures can be created by adding `move` in front of the function: `partial!(move ..)`

```rust
#[macro_use]
extern crate partial_application;

fn foo(a: i32, b: i32, c: i32, d: i32, mul: i32, off: i32) -> i32 {
    (a + b*b + c.pow(3) + d.pow(4)) * mul - off
}

fn main() {
    let bar = partial!( foo(_, _, 10, 42, 10, 10) );
    assert_eq!(
        foo(15, 15, 10, 42, 10, 10),
        bar(15, 15)
    );
}
```
