---
title: "Bindings Are Cheap: Managing Rightward Drift"
author: Ryan James Spencer
date: 2020-02-20T10:34:34.578114198+00:00
tags:
  - rust
summary: >-
  It can be easy to fall into the habit of using match and if let everywhere
  but you soon may find yourself with heavily nested code. Rightward drift is a
  pain to decipher in any language. The good news is that you can easily manage
  rightward drift in Rust using a few techniques.
---

How do you avoid deeply nested `if let` or `match` statements when you're first
coming to Rust? Rightward drift is a pain to decipher in any language, but the
good news is you can manage rightward drift in Rust with a few techniques and
some mental shifting. Maybe this is the code you're writing which has a lot of
if-let chaining:

```
if let Some(x) = some_func() {
    // do stuff with x
    if let Some(y) = some_func2() {
        // do other stuff with y
        if let Some(z) = some_func3() {
          // and so on
        } else {
          reticulating_splines()
        }
    } else {
        engage_thrusters()
    }
} else {
    launch_the_missiles()
}
```

In Rust, everything is an expression, and every expression has a value. For
control flow, that means all branches must return values of the same type. If
you look at the code above you ought to see that whole thing as `()`, assuming
the functions in the `else` blocks above return `()`. When I look at the above
code snippet I think "this code is always meant to succeed but with different
results on the types of success". This code is always mapping `Some` and `None`
to `()`, which doesn't tell the caller much besides "I did something."

**A `None` implies the absence of something. If we want more information for
_why_ the data we want isn't there we can use the `Err` variant on `Result`**.
The intent with the `try` operator is to always allow a way to express this
'failure' back to the caller when it first happens; we should not assume we can
go ahead safely with the subsequent code and return from the current function.

A style I like to recommend to people is known by some as "newspaper
article" style. Because Rust is an expression-oriented language which means we can
`let`-bind to anything! Using the `try` (`?`) operator means we can write
our fix as:

```
let x = some_func()
  .or_else(|| { launch_the_missiles(); None } )?;
let y = some_func2()
  .or_else(|| { engage_thrusters(); None } )?;
let z = some_func3()
  .or_else(|| { reticulating_splines(); None } )?;
// and so on.
```

If we wanted to only give the caller the sense that nothing bad happened,
we could wrap the whole thing in a block and discard the result (NB. the
semicolon at the end of the block):

```
fn top_level() {
    fn go<T>() -> Option<T> {
        let x = some_func()
          .or_else(|| { launch_the_missiles(); None } )?;
        let y = some_func2()
          .or_else(|| { engage_thrusters(); None } )?;
        let z = some_func3()
          .or_else(|| { reticulating_splines(); None } )?;
        // and so on.
    }
    go(); // throw away the result for the caller.
}
```

**But this is weird**. Giving callers control is at the crux of good error
handling, especially when it comes to something as powerful as error handling
with values!

What I absolutely love about the rampant `let`-bindings approach is that it
provides a lot of flexibility for modification; with `let` bindings we can
remove or modify the offending assignments exactly.

Rust also lets us shadow variables and with its move semantics we can avoid
unexpected allocations, hence we can do things like expressing
data as it changes throughout various steps:

```
struct Json {
  property: i64,
}

struct Error {
  SerdeError(serde::Error),
  IoError(std::fs::IoError),
}

fn update_json() -> Result<(), Error> {
  let json = include_str!("../some.json");
  let json: Json = serde_json::from_str(&json);
  json.property = 42;
  let json = serde_json::to_string(&json);
  fs::write("../some.json", json)?;
}
```

Use `let` bindings liberally and you'll make your code easier to modify and
read. If you have custom types you've written yourself you might,

* one day be able to write an implementation for the `Try` trait yourself when it stabilizes (its currently experimental)
* take a cue from `Option` and `Result` and write similar combinators that let you get at internal data for your type
* merely wrap things in `Option` and `Result` and use the bevy of methods they expose
