---
title: Bindings Are Cheap: Managing Rightward Drift
author: Ryan James Spencer
date:
tags:
  - rust
summary: >-
---

It can be easy to fall into the habit of using `match` and `if let` everywhere
but you soon may find yourself with heavily nested code. Rightward drift is a
pain to decipher in any language. The good news is that you can easily manage
rightward drift in Rust using a few techniques. Maybe this is the code you're
writing which has a lot of if-let chaining:

```
if let Some(x) = some_func() {
    // do stuff with x
    if let Some(y) = some_func2() {
        // do other stuff with y
        if let Some(z) = some_func3() {
        // and so on
```

You could use a monadic style to fix this, and it _is_ nice:

```
some_func().and_then(|x| {
  // do stuff with x
  some_func2()
}).and_then(|y| {
  // do stuff with y
  some_func3()
}).and_then(|z| {
  // and so on
})
```

But this doesn't include the possible `else` clauses mentioned in our original example.
To get that back but use something other than `match` or `if-let` we could try:

```
some_func().and_then(|x| {
  // do stuff with x
  some_func2()
    .or_else(|| { launch_the_missiles(); Some(1) })
}).and_then(|y| {
  // do stuff with y
  some_func3()
    .or_else(|| { engage_thrusters(); Some(2) })
}).and_then(|z| {
  // and so on
}).or_else(|| { reticulating_splines(); Some(3) })
```

Now things aren't horribly indented but coding with this is a tad cumbersome. An
alternative style I like to recommend to people is known by some as "newspaper
article" style. Rust is an expression-oriented language which means we can
`let`-bind to anything! Rust's move semantics and the `try` (`?`) operator means
we can write our first fix as:

```
let x = some_func()?;
let y = some_func2()?;
let z = some_func3()?;
```

without any fuss about needless allocations or random panics. The mental shift
in both approaches is that **a `None` implies the absence of something, possibly with
some added information for _why_ the data we truly want isn't there, which is
exactly what the `Err` variant on `Result` is for**. With the `else` cases this
becomes:

```
let x = some_func()
  .or_else(|| { launch_the_missiles(); Some(1) })?;
let y = some_func2()
  .or_else(|| { engage_thrusters(); Some(2) })?;
let z = some_func3()
  .or_else(|| { reticulating_splines(); Some(3) })?;
```

What I absolutely love about this approach is that it provides a lot of
flexibility for modification and different types. If something in the middle of
our monadic approach changes its type or is removed, we need to change the
structure of our expression that builds up our final value, but with `let`
bindings we can remove or modify just the offending assignments. I personally
find this approachonestly more like `do`-notation that comes with using monads
in Haskell, as if it were:

```
do
  x <- maybe some_func \() -> do
    launch_the_missiles
    Just 1
  y <- maybe some_func1  \() ->
    engage_thrusters
    Just 2
  z <- maybe some_func1  \() ->
    reticulating_splines
    Just 3
```

Rust also lets shadow variables, hence we can do things like expressing
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
  fs::write("../some.json")?;
}
```

Use `let` bindings liberally and you'll make your code easier to modify and
read. If you have custom types you've written yourself you might,

* one day be able to write an implementation for the `Try` trait yourself when it stabilizes (its currently experimental)
* take a cue from `Option` and `Result` and write similar combinators that let you get at internal data for your type
* merely wrap things in `Option` and `Result` and use the bevy of methods they expose
