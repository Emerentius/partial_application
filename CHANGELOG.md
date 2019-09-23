# 0.1.0 (2018-02-27)
Initial release.
Allows defining partial functions with the syntax `partial!(some_fn some_expr, _, 3)` or `partial!(some_fn(some_expr, _, 3))` with parentheses. However, `some_fn` can only be a single identifier. Paths like `some_module::foo` do not work.

# 0.2.0 (2019-09-23)
* Allow functions containing paths as well.
* Change syntax to `partial!(move? ("," | ";" | "=>") comma_separated_arg_list)`  
  example: `partial!(foo => some_expr, _, 3)`  
  This change is necessitated by allowing paths to functions as no other tokens may follow an `expr` token in rust macros.
