#![no_std]

//! The `partial!` macro allows for partial application of a function.
//!
//! `partial!(some_fn => arg0, _, arg2, _)` returns the closure `|x1, x3| some_fn(arg0, x1, arg2, x3)`. <br>
//! Move closures are created by adding `move` in front of the function: `partial!(move ..)`
//!
//! ```rust
//! #[macro_use]
//! extern crate partial_application;
//!
//! fn foo(a: i32, b: i32, c: i32, d: i32, mul: i32, off: i32) -> i32 {
//!     (a + b*b + c.pow(3) + d.pow(4)) * mul - off
//! }
//!
//! fn main() {
//!     let bar = partial!(foo => _, _, 10, _, 10, 10);
//!     assert_eq!(
//!         foo(15, 15, 10, 42, 10, 10),
//!         bar(15, 15,     42)
//!     );
//! }
//! ```
//!
//! The expressions used to fix an argument are reevaluated on every call of the new function because of the straightforward translation behind the macro. <br>
//! ```rust
//! # #[macro_use]
//! # extern crate partial_application;
//! # fn main() {
//! #
//! fn identity(x: u32) -> u32 { x }
//!
//! let mut n = 0;
//! let mut f = partial!(identity => { n += 1; n});
//! assert_eq!(f(), 1);
//! assert_eq!(f(), 2);
//! # }
//! ```
//! Pre-compute arguments to be fixed in a local variable, if their creation is expensive or has unwanted side-effects.
//!
//! You can also use a comma (`,`) or semicolon (`;`) instead of the arrow (`=>`).
//! This strange syntax choice is due to limitations imposed on us by the macro system.
//! No other tokens may follow the expression token for the function.

/// The macro that creates a wrapping closure for a partially applied function
///
/// Syntax: `partial!(move? fn_name ("=>" | "," | ";") comma_separated_arg_list)`
///         `=>`, `,` and `;` are completely equivalent and act only as separator between
///         function and arguments.
///
/// Function arguments are either expressions or `_` <br>
/// `_` arguments have to be supplied on each call. They forward from the resulting closure into the function. <br>
/// Expressions are hardcoded into the function call. <br>
/// `partial!(foo => _)` => `|a| foo(a);` <br>
/// `partial!(foo => 2)` => `|| foo(2);`
///
/// Prepending `move` to the `fn_name` creates a move closure. Trailing commas are permitted.
#[macro_export]
macro_rules! partial {
    // The macro works with 3 lists
    // 1. closure args $cl_arg(s)
    //    The argument identifiers for the closure
    // 2. fn args      $fn_arg(s)
    //    The argument identifiers and forwarded expressions for the fn
    //
    //    Arg idents are passed around for hygiene reasons and to keep track
    //    of their number
    //
    // 3. the macro arguments $m_args
    //    A list of expressions and the forwarding sign '_'
    //    from which the former two lists are built up
    //
    // Until $m_args is empty, an element is popped off its front
    // and the appropiate pieces are pushed to cl_args and/or fn_args
    //
    // The fn ident and the move closure "boolean" (either "move" or "()")
    // are simpyl passed through during list processing inside $pt (pass-through)

    // exhausted macro arguments, create closure
    (@inner [(() $id:expr) ($($cl_arg:ident),*) ($($fn_arg:expr),*)] ()) => {
        |$($cl_arg),*| $id($($fn_arg),*);
    };
    // with move
    (@inner [(move $id:expr) ($($cl_arg:ident),*) ($($fn_arg:expr),*)] ()) => {
        move |$($cl_arg),*| $id($($fn_arg),*);
    };

    // process forwarder '_' ,
    (@inner [$pt:tt ($($cl_arg:ident),*) ($($fn_arg:expr),*)] (_ , $($m_arg:tt)*) ) => {
        partial!(
            @inner [$pt ($($cl_arg,)* a) ($($fn_arg,)* a)] ($($m_arg)*)
        )
    };
    // last forwarder (if no trailing comma)
    (@inner [$pt:tt ($($cl_arg:ident),*) ($($fn_arg:expr),*)] (_) ) => {
        partial!(
            @inner [$pt ($($cl_arg,)* a) ($($fn_arg,)* a)] ()
        )
    };

    // process given expr
    (@inner [$pt:tt $cl_args:tt ($($fn_arg:expr),*)] ($e:expr , $($m_arg:tt)*) ) => {
        partial!(
            @inner [$pt $cl_args ($($fn_arg,)* $e)] ($($m_arg)*)
        )
    };
    // last expr (if no trailing comma)
    (@inner [$pt:tt $cl_args:tt ($($fn_arg:expr),*)] ($e:expr) ) => {
        partial!(
            @inner [$pt $cl_args ($($fn_arg,)* $e)] ()
        )
    };

    // entry points
    // ordered to match eagerly
    // move
    (move $id:expr , $($args:tt)*) => {
        partial!(@inner [(move $id) () ()] ($($args)*))
    };
    (move $id:expr ; $($args:tt)*) => {
        partial!(@inner [(move $id) () ()] ($($args)*))
    };
    (move $id:expr => $($args:tt)*) => {
        partial!(@inner [(move $id) () ()] ($($args)*))
    };

    // no move
    ($id:expr , $($args:tt)*) => {
        partial!(@inner [(() $id) () ()] ($($args)*))
    };
    ($id:expr ; $($args:tt)*) => {
        partial!(@inner [(() $id) () ()] ($($args)*))
    };
    ($id:expr => $($args:tt)*) => {
        partial!(@inner [(() $id) () ()] ($($args)*))
    };
}

#[cfg(test)]
mod test {
    // compile time check for maximum arity
    // 60 with default recursion limit
    #[allow(unused)]
    #[rustfmt::skip]
    fn high_arity(
        _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (),
        _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (),
        _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (),
        _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (),
        _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (), _: (),
        _: (), _: ()
    ) {
        let c = partial!(high_arity,
            (), (), (), (),     (), (), (), (),     (), (), (), (),
            (), (), (), (),     (), (), (), (),     (), (), (), (),
            (), (), (), (),     (), (), (), (),     (), (), (), (),
            (), (), (), (),     (), (), (), (),     (), (), (), (),
            (), (), (), (),     (), (), (), (),     (), (), (), (),
            (), ()
        );
    }

    #[test]
    fn argument_order() {
        // non-commutative arguments
        // wrong forwarding order will result in error
        fn foo(a: u32, b: u32) -> u32 {
            100 + a - b
        }

        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(foo(i, j), partial!(foo => i, _)(j));
            }
        }
    }

    #[test]
    // tests preservation of argument order in a more complex setting
    fn interspersed_expr_and_forwarders() {
        fn foo(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> u8 {
            fn shift(b: bool, n: usize) -> u8 {
                (b as u8) << n
            }
            // in reverse so a is most significant
            // resulting number will be abcdef
            // where each letter represents a bit
            [f, e, d, c, b, a]
                .iter()
                .cloned()
                .enumerate()
                .fold(0, |acc, (n, arg)| acc | shift(arg, n))
        }

        let reduced_foo = partial!(foo => true, _, _, true, true, _);
        assert_eq!(reduced_foo(false, false, false), 0b100110);
    }
}

// moving a !Copy type forces FnOnce
// which should fail to compile on reuse
#[allow(unused)]
/// ```compile_fail
/// #[macro_use] extern crate partial_application;
/// fn main() {
///     struct Foo(u32);
///     let sub = |a: u32, b: Foo| a - b.0;
///
///     let f = Foo(5);
///     let sub5 = partial!(move sub => _, f);
///
///     sub5(5);
///     sub5(5);
/// }
/// ```
struct MoveCompileFail;

#[cfg(test)]
#[allow(unused)]
// compile check
fn syntax_check() {
    partial!(::core::option::Option::<i32>::is_some => _ );

    #[derive(Clone)]
    struct NoCopy;

    fn foo(_: u8, _: u8, _: u8, _: u8, _: u8, _: u8, _: NoCopy) {}

    let a = (NoCopy,);
    let b = (5,);
    let five: fn() -> u8 = || 5;

    let num = 10;

    // test various forms of expressions
    // and trailing commas for forwarders and expressions
    partial!(foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), _);
    partial!(foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), _,);
    partial!(foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), a.clone().0,);
    partial!(foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), a.clone().0);

    partial!(move foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), _);
    partial!(move foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), _,);
    let s = a.clone();
    partial!(move foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), s.clone().0,);
    let s = a.clone();
    partial!(move foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), s.clone().0);
    let s = a;
    partial!(move foo => 2, _, num, {stringify!(boo); 2}, b.0, five(), s.0);
}
