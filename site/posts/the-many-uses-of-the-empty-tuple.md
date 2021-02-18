---
title: The Many Uses Of The Empty Tuple
author: Ryan James Spencer
date: 2021-02-10T21:12:41.065637462+00:00
tags:
  - rust
summary: >-
    For newcomers to Rust unfamiliar with the empty tuple (`()`), it can be a
    confusing type; what's the point of this thing and how can I use it
    idiomatically rather than having to endure its presence? Here are some common
    patterns where unit actually plays a helpful role in both guiding program
    construction, helping with readability, and even reducing unnecessary memory
    allocations.
---

For newcomers to Rust unfamiliar with the empty tuple (`()`), it can be a
confusing type; what's the point of this thing and how can I use it
idiomatically rather than having to endure its presence? Here are some common
patterns where unit actually plays a helpful role in both guiding program
construction, helping with readability, and even reducing unnecessary memory
allocations.

### A Primer

_If you feel comfortable with unit or don't really care about the nuances, feel
free to skip this section, but it is short and recommended as it informs the
tips below._

The empty tuple (`()`), sometimes affectionately, and historically, referred to
as `unit`, as I will call it for the rest of this article, describes the type of
something that _does_ have a value, but a value you don't care about. The empty
tuple is perfect for these situations because it can never carry any information
with it, but it is different to the `never` (`!`) type in Rust because the
`never` type designates the type of things that can never be constructed (hence
the name). Thus, functions that may never return are technically `!`, but
functions that perform some sort of effect are `()`. The `void` keyword fills a
similar role to unit in C-derived languages, but unit has the advantage of being
usable in type annotations, which we'll see come in handy later.

For starters, `()` is both the type and the value of the type. Sometimes unit is
implicit, such as when you write a function with no return type, or when you
slap a semicolon on an expression. In Rust, a great many things are expressions,
which means they have values and therefore have types. For example, assigning to
an assignment, such as in the case `let x = let y = 12;` gives us `y = 12` and
`x = ()` as an assignment expression itself has type unit. For each example we
don't care about the value and only care about the action that took place (the
action of the function invoked or the expression that was run, the act of
assigning a value to a variable name, and so on.)

### Why Use An If-Let?

`if-let` let's us combine both the niceties of pattern matching with a `match`
statement, without having to be explicit about fall-through cases. If you are
pattern matching only to perform some action at the end, you can be more concise
and simply turn this:

```
fn side_effect() {
  println!("a side effect");
}

let connection = socket.accept();

match connection {
  Ok(_) => side_effect(),
  _ => (),
}
```

Into this:

```
fn side_effect() {
  println!("a side effect");
}

let connection = socket.accept();

if let Ok(_) = connection {
  side_effect();
}
```

Given that we used a semicolon on `side_effect` it would have compiled just as
fine if we had a return value from `side_effect`, unless you use `Result` which
is marked as `must_use` in the compiler, forcing you to deal with the errors
that may crop up. In that case, if you truly wanted to ignore a return value of
a function, you could do `let _ = side_effect();` instead.

### Clarifying The Presence And Reason Of Why Things Failed

One common mistake that newbies will make is to avoid error handling with
`Result` in favor of simply having functions that return `()` and panicking via
`expect` or `unwrap` et. al. `()` tends to designate that things are _not_ going
to blow up at runtime, and so this type is actually the wrong thing to signal to
other peers. Rust has a type called `!` or "never" that implies that something
may never return or fail ("infallible").

A simple flow chart for choosing a type for error handling could go something
like this:

1. Nothing obvious in the code path in question is going to fail, including via
   panics, e.g. with `unwrap`, `expect`, etc.: use `()`

2. I know this might fail and I
   1. Want to know why: use `Result`
   2. Only care about the presence or absence of something: use `Option`

3. I know this is going to live forever and never return (like a socket
   connection): use `!`

Although I don't commonly see the third case used as often as the prevalence of
the first and second cases, I do think it is useful to signal to others that
something is going to loop indefinitely, replace the current process with
`exec`, and so on. It currently requires a crate attribute, which may be enough
to keep you away from using it until it lands on stable without the need for the
annotation.

### The Traverse Trick

There is a common pattern in Haskell called `traverse`. A way to think about it
is like turning a collection of things inside out. For example, if we have a
`Vec<Result<T, E>>` we can 'traverse' on this collection, treating it's values
as inputs to a function, and turn it into a `Result<Vec<T>, E>` instead. This is
wildly useful, and you can extend this pattern for your own types and
collections, too, but one common use case is a function that returns `Result<T,
E>` that we want to run over several elements:

```
struct Error;

fn may_fail(x: i32) -> Result<i32, Error> {
  Ok(x)
}

fn main() {
  let inputs = vec![1, 2, 3];
  let outputs: Result<Vec<i32>, Error> = inputs.into_iter().map(may_fail).collect();
}
```

What if we didn't want the outputs? What if all we wanted to do was to run
`may_fail` for the effects it produces? We could change this around:

```
struct Error;

fn may_fail(x: i32) -> Result<i32, Error> {
  Ok(x)
}

fn main() {
  let inputs = vec![1, 2, 3];
  Result<Vec<i32>, Error> =
  inputs.into_iter().map(|i| { may_fail(i).map(|_| ()) }).collect::<Result<Vec<()>, Error>>();
}
```

But now we are allocating a vector just to fill in all the units. Let's fix
that:

```
struct Error;

fn may_fail(x: i32) -> Result<i32, Error> {
  Ok(x)
}

fn main() {
  let inputs = vec![1, 2, 3];
  Result<Vec<i32>, Error> =
  inputs.into_iter().map(|i| { may_fail(i); }).collect::<Result<(), Error>>();
}
```

This version is specialized as it will never allocate; no collection is being
created and each `()` type can be optimized away by the compiler as having no
bearing on program semantics.

You can similarly do this for Option, and, as mentioned, can implement the same
trick for your own custom types.

### Figuring Out Types With Invalid Annotations

This one is particularly helpful if you are not using something like
`rust-analyzer` or the like. If you prefer to simply run the compiler in a loop,
such as with `cargo watch`, you can get immediate feedback on the type of
something by assigning the value to an invalid type, such as:

```
main() {
  let x: () = mystery();
}
```

You just want to use some type of annotation you are absolutely sure this thing
is _not_, and _most_ things are _not_ unit. If unit doesn't work, you can switch
it up to other unlikely things: `u128`, `!` (requires crate attribute), and on
and on. If you know the thing is a collection, try a scalar value. Usually it
doesn't take much guesswork to get the compiler to spit something out, but you
will wind up with something like the following:

```
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:6:17
  |
6 |     let x: () = mystery();
  |            --   ^^^^^^^^^ expected `()`, found struct `BTreeSet`
  |            |
  |            expected due to this
  |
  = note: expected unit type `()`
                found struct `BTreeSet<i32>`
```

Above we see what we claimed was the real type and what the compiler inferred or
realized is the real type. You can flip this trick on the head, too. With
generics it can be easy for types to become things you didn't quite intend
simply by how they got used in other contexts, hence it can be helpful to
sprinkle around annotations in code to be really clear on precisely what final
shape(s) you are expecting to deal with.
