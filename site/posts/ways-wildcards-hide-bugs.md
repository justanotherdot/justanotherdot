---
title: Ways Wildcards Hide Bugs
author: Ryan James Spencer
date: 2020-04-09T10:42:10.396858392+00:00
tags:
  - rust
summary: >-
  We call the wildcard variable, denoted by an underscore (`_`), the "don't care"
  variable to throw away values we don't care to keep. Wildcards don't bind any
  values, so wildcards have specific support in the language, as opposed to other
  languages where an underscore may be yet another variable name.
---

We call the wildcard variable, denoted by an underscore (`_`), the "don't care"
variable to throw away values we don't care to keep. Wildcards don't bind any
values, so wildcards have specific support in the language, as opposed to other
languages where an underscore may be yet another variable name.

I'll discuss three ways bugs can lurk innocently behind wildcards. Wildcards are useful, but reckless use of them can lead to bugs! I'll discuss three ways this can happen and how to be a bit more vigilant with their use. The general principle across these fixes is to think twice when you find yourself writing a wildcard. Ask if it is a value you want to ignore?

### Throwing away values

The `;` without a `let` statement in Rust means "turn this thing into a `()`because I want to cast the value into something that isn't an error but isn't a useful value." If you write a `Result` in Rust and don't propagate the value with `?` and don't assign the value to a variable name, say something like this:

```
pub fn main() -> Result<(), std::io::Error>  {
    std::fs::remove_file("/hello");
    Ok(())
}
```

rustc whinges, stating:

```
   Compiling playground v0.0.1 (/playground)
warning: unused `std::result::Result` that must be used
 --> src/main.rs:2:5
  |
2 |     std::fs::remove_file("/hello");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: this `Result` may be an `Err` variant, which should be handled

    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/playground`
```

However, we can entirely silence this with a wildcard:

```
pub fn main() -> Result<(), std::io::Error>  {
    let _ = std::fs::remove_file("/hello");
    Ok(())
}
```

It may seem like you don't care about the value, but if the value represents a failure in any way, including, say, an `Option`, you want to ensure that value is not lost. Silencing any error generally means failures aren't be handled or reported, and that can mean things _seem_ fine on the surface, but might be broken.

To give a production example of this, I recently fixed a test bug that used a database transaction but discarded its `Result`. It did this with the same sort of wildcard binding we saw above. As such, when it encountered any failure in the transaction and ignored it, the test became a false positive because nothing would panic. Two ways to fix this are to turn the enclosing test as one that returns `Result`, which you can do and is excellent, and the other is to bind the value and `unwrap`, `expect` or even do an `assert` on it given what sort of ergonomics and output you like.

### Using wildcard cases in a match carelessly

Not everyone coming to Rust is used to match expressions. The anatomy of a match expression is pretty straightforward; you `match` on the term of interest, and each 'arm' is a case that we consider. Match statements are exhaustive, which means we either check every possible value we are matching against or we slap in a wildcard because we don't care or can't feasibly match on every possible value. When things have high-cardinality of the values they can represent, a wildcard might be funneling unexpected value into the wrong logic.
rustc is great at calling you out on this sort of stuff. Consider this code that
matches on an integer:

```
pub fn main() {
    let x: i32 = 12;
    match x {
      y if y == 12 => x,
      y if y < 12 => x,
      y if y > 12 => x,
    };
}
```

Integers have many values, and we're trying to narrow down the selection to an exact match, values that are greater than the exact match, and values that are lesser than the exact match. However, rustc wants to make sure we've genuinely considered every angle:

```
   Compiling playground v0.0.1 (/playground)
error[E0004]: non-exhaustive patterns: `_` not covered
 --> src/main.rs:3:11
  |
3 |     match x {
  |           ^ pattern `_` not covered
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms

error: aborting due to previous error

For more information about this error, try `rustc --explain E0004`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
```

If you make the first match case `12 => x` rustc tells us that we aren't considering full ranges:


```
   Compiling playground v0.0.1 (/playground)
error[E0004]: non-exhaustive patterns: `std::i32::MIN..=11i32` and `13i32..=std::i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match x {
  |           ^ patterns `std::i32::MIN..=11i32` and `13i32..=std::i32::MAX` not covered
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms

error: aborting due to previous error

For more information about this error, try `rustc --explain E0004`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
```

rustc is already nudging us towards a solution specifically here where guards aren't doing much for us by spelling out the exact patterns for us. With some changes, this builds cleanly:

```
pub fn main() {
    let x: i32 = 12;
    match x {
      12 => x,
      std::i32::MIN..=11i32 => x,
      13i32..=std::i32::MAX => x,
    };
}
```

If possible, it's best to refine the type into something like a sum type (enum) so that we can match on exact variants. In our example above, if we were to `x.cmp(12i32)`, we'd have three cases to check for; `LessThan`, `MoreThan`, and `Eq`.

```
use std::cmp::Ordering;

pub fn main() {
    let x: i32 = 12;
    match x.cmp(&12i32) {
        Ordering::Equal => x,
        Ordering::Less => x,
        Ordering::Greater => x,
    };
}
```

If you still need to use a wildcard, a useful tool is to turn to property-based testing or fuzzing to ensure that the specific branches are covered. Property-based testing and fuzzing are ways to generate test input randomly. Many libraries have support for generating specific ranges of values, such that you could focus more on the "black hole" that the wildcard case is creating, which is "everything that isn't the cases I _have_ covered." For example, you might have three branches, where one of the branches is a wildcard. 10-20% could go into the first two known branches, and the remaining 60-80% could be left to generate data that isn't of those two known branches to shine a flashlight into the dark crevices. When values become known that have special treatment, you can add them to the branches of the `match` and adjust the percentages accordingly.

### Leaving arguments dormant in a function

When we design public-facing interfaces, it can be tempting to try to keep them stable by softening some of their fields. One way to do this with functions is to treat arguments as "unused" by prefixing them with an underscore. The surface seems the same, but now the value isn't used.

Since the function ignores these values, callers may make incorrect assumptions about how arguments might change the output. Leaving arguments dormant is especially nefarious when the function is non-deterministic, and it can *feel* like the different arguments are making a change. We can't always trust that users of our code are going to, or even be able to, read the source code.

Ignoring arguments has a straightforward fix; don't ignore arguments. Favor deprecating the function for a new version as the alternative and signal new versions with some precise apparatus. This principle is the same across a lot of interface design: make a new thing and migrate over to it rather than trying to change the other thing in place. When you rig up the new version, you can delete the old one, but you need to keep the old one around until you finalize the transition. Changing things in place is fine if the consumption is still light, but the breathing room gained from doing it the immutable way far outweighs the 'ease' of trying to modify the pre-existing.

### Wildcards aren't bad!

Don't get me wrong, wildcards _are_ useful, but they are easy to abuse if you are new to them. Hopefully, these points come to mind next time you write a `match` expression or modify a function!
