---
title: Idiomatic Argument Passing in Rust
author: Ryan James Spencer
date: 2020-02-13T20:05:17.823423765+00:00
tags:
  - rust
image: idiomatic-argument-passing-in-rust.jpg
summary: >-
  If you're coming from a language that supports automatically taking references
  to arguments you may wonder why Rust can't do the same. Rust is all about giving
  developers a better control of the memory layout of the data in their programs.
hero_font_color: light
---

If you're coming from a language that supports automatically taking references
to arguments you may wonder why Rust can't do the same. Rust is all about giving
developers a better control of the memory layout of the data in their programs.
Since Rust has the notion of ownership, we don't have to worry about large
objects being copied into a function when the arguments to a function are owned.
Instead, they are moved (pass-by-move), and when I first started writing Rust I
assumed the idiom was to always use owned types for function arguments. For
clarity, we call something an "owned" typed when it isn't behind a reference
(`&`). When an argument is behind a reference, we call that a "borrowed" type.

**tl;dr**
_Idiomatic Rust functions ought to borrow arguments unless a function needs to
completely own an argument for ergonomics (say, method chaining) or allocation
(the caller won't need to re-use the data, perhaps)._

What's the case against owned types for function arguments as the default? All
data in Rust must have an owner and that owner is a variable. Function arguments
are variables. This means that when you give a function an owned type, you force
a caller to give away ownership of the data it has allocated and probably wanted
to use further down the line. If the function doesn't give back the value, it is
lost to the caller, and the memory will be de-allocated at the end of the call's
scope. Often callers _do_ want to keep ownership of the values they pass into
functions.

Immutable borrows let functions decide if they want to make selective
allocations but that does mean a function may be allocating when the caller may
want to know all allocations upfront. Owned types are a good fit for this as it
is the caller's responsibility to allocate and give up ownership to the function
for its use. _Alternatively_, if a function wants to make a change (mutate) an
argument, it will be clear to the caller that data may change signaled by adding
`mut` after the `&`. The common practice in C is to take pointers to
non-primitive values. This is done so large objects don't get copied on each
function call. However, with this approach of using raw pointers there is no way
to clarify when a pointer is simply going to read data and when it is going to
change it. With borrowed types in Rust we get this clarity at the syntactic
level.

**Idiomatic Rust functions borrow arguments unless it truly needs to own the
values or they are primitives.** Rust copies primitive values as they are part
of the `Copy` trait. And this isn't to say you should _never_ take owned
arguments. The [builder
pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html)
explicitly takes ownership and gives it back at each method call, allowing us to
chain together calls prior to a `let` assignment. If we used `&mut self` instead
we'd need to first assign the value with `let mut` and make the calls
separately.

This leads us to an interesting example: How would we write the inside of this
function?

```
fn thin_air() -> &Vec<i32> {
    unimplemented!()
}
```

We could try to allocate and take a reference to the allocation?

```
fn thin_air() -> &Vec<i32> {
    &vec![]
}
```

But the borrow checker will refuse this program because our `Vec` only exists
for the scope of `thin_air` and if we held a reference after the point it was
dropped (its memory is freed) we'd be holding a pointer to garbage which is not
safe to read or write to. Thus, if we want to return a borrowed type, we must
also take a borrowed type or something that holds a borrowed type.

```
struct<'a> Data {
  integers: &'a Vec<i32>
}

fn thin_air(data: Data) -> &Vec<i32> {
  data.integers
}
```

To recap, Rust cares about memory safety and layout a fair amount and puts the
work on the programmer to decide when references to arguments should be taken.
Choosing immutable borrows by default means you won't cause any unintended
consequences besides maybe some stray allocations. If you want to change the
content that the caller owns and, hence, has allocated, switch to a mutable
borrow. Lastly, if you know the caller won't need the argument anymore or if it
wants to return an owned type in exchange of the passed in argument(s), the
function ought to take ownership.
