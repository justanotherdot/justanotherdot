---
title: Building An Intuition for Pattern Matching
author: Ryan James Spencer
date: 2020-05-19T10:08:50.654279125+00:00
tags:
  - rust
summary: >-
---

_What's the point of pattern matching if we already have conditionals and
variable assignment in a language?_

Pattern matching helps tease apart values and construct control flow using the
shape of data rather than bespoke logic, methods on types, or special fields on
a struct. For example, in languages that don't have first-class support for sum
types, enums in Rust, you'd have to encode the variant as a unique tag on
something like a struct, e.g.,

```
struct Option {
    tag: String, // maybe one of 'Some' or 'None'.
    // and so on.
}
```

Then the `tag` field can be checked by traditional control flow. This is precisely
how it is done in languages like TypeScript, but in Rust, where sum types are
supported, we have no unique `tag` field to check and, since the compiler hides
this information away from us, we can't write a method to describe which variant
we have in our hands. It would be a bit clumsy if the compiler generated methods
for us as we might want to have methods with the same name!

Any kind of syntactic sugar used to construct a value is known as a
**constructor**, such as building values for structs, enums, tuples, and so on.
Pattern matching gives us a way to describe the shape of data using constructors
to match on and what to do if the value matches. This analogy isn't perfect, but
I like to think of patterns as mirrors with outlines; if the reflection matches
the outline of a constructor, we go down that path of logic, possibly with some
new values drawn out of the data. Here are some common patterns for
constructors:

```
pub struct S {
    field: i64,
}

pub enum E {
    FirstVariant,
    SecondVariant,
}

pub fn main() {
    // Tuples.
    let a = ("Fizz", "Buzz");
    match a {
        (p, q) => println!("{}", format!("{}{}", p, q)),
    }

    // Numeric literals.
    let b = 123;
    match b {
        std::i32::MIN..=99 => println!("under one-hundred"),
        100 => println!("exactly one-hundred"),
        101..=std::i32::MAX => println!("above one-hundred"),
    }

    // Strings.
    let c = "A string.";
    match c {
        "A string." => println!("it's _the_ string."),
        _ => println!("some other string."),
    }

    // Enums.
    let x = E::SecondVariant;
    match x {
        E::FirstVariant => println!("first variant of E"),
        E::SecondVariant => println!("second variant of E"),
    }

    // Structs.
    let y = S { field: 100 };
    match y {
        S { field } => println!("field is: {}", field),
    }

    // Slices.
    let z = vec![1, 2, 3];
    match *z { // we need * to dereference Vec to a slice.
        [a, b] => println!("{} + {} = {}", a, b, a + b),
        [a, b, c] => println!("{} + {} * {} = {}", a, b, c, a + b * c),
        _ => println!("any other unmatched vector"),
    }
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=483dd9713719d848f7e221047961e8c8)

_Where can we put patterns?_

`match` is the traditional way of doing pattern matching but not the only way.
Matches work top-to-bottom, and they ensure that every case is handled, known as
**exhaustivity checking**.

```
enum Val {
    Integer(i64),
    Float(f64),
}

match {
    Val::Integer(x) => println!("It's an integer: {}", x), // one "arm" or "case"
    // without anything else, this is non-exhaustive; it doesn't include Val::Float!
}
```

which fails to compile with the following error:

```
error[E0004]: non-exhaustive patterns: `Float(_)` not covered
 --> src/main.rs:8:11
  |
1 | / enum Val {
2 | |     Integer(i64),
3 | |     Float(f64),
  | |     ----- not covered
4 | | }
  | |_- `Val` defined here
...
8 |       match v {
  |             ^ pattern `Float(_)` not covered
  |
  = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms

error: aborting due to previous error
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=65e0688f852c45e4c1712f2481ef2231)

Pattern matches in `let`s and function arguments will also work but must be
**irrefutable**, which is a fancy way of saying that the pattern can never fail.
Any pattern that covers all possible values of a type is irrefutable. It could
be literal like a range or with a variable, which will always capture a value
and, therefore, match.

```
// works.
pub fn f((x, y): (i32, i32)) -> i32 {
    x + y
}

// does not work.
//pub fn g((1, 2): (i32, i32)) {
// fails on anything other than g(1, 2).
// the compiler rejects this as a refutable pattern
// which is in place where only an irrefutable pattern can be.
//}

pub fn main() {
    //let 12 = 12; // fails.
    let x = 12; // succeeds.
    f((x, x));
    let std::i32::MIN..=std::i32::MAX = 12; // succeeds, covers all values.
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d21343054645f8500c7701fdcc174171)

With functions, this is a bit different from other functional languages like
Erlang or Haskell. In those languages, you can write multiple function
declarations, each with their pattern match, and the function that matches the
pattern will be the one that executes. You can think of this like match
expressions but for functions! Rust, unfortunately doesn't have this, but it's
still fine to take the full value from the argument and make the entire function
body a `match`. So this in Elixir:

```
def f(1) do
  // first case.
end

def f(2) do
  // first case.
end

def f(x) do
  // final, irrefutable case.
end
```

could be expressed as:

```
pub fn f(x) {
    match x {
        1 => , // first case.
        2 => , // second case.
        x => , // final, irrefutable case.
    }
}
```

_Isn't this a bit tedious? What if you don't care about particular portions of a
shape?_

Ignoring particular values is easy with the `_` variable, or we can prefix a
variable name with `_` if we want to keep the name but ensure it can't be used.
This is formally known as a **wildcard**, but informally known as the "don't
care" variable. The equivalent for structs is `..` where we can specify only the
fields we care about and ignore the rest. These two dots, a bit like an
ellipse, must be mentioned in the last place of the struct.

```
struct S {
    field: i32,
    property: (i32, i64),
}

pub fn main() {
    let s = S {
        field: 42,
        property: (12, 13),
    };
    match s {
        S { property: (12, _), .. } => println!("{}", 12),
        S { field, .. } => println!("{}", field),
        // or `S { field, property: _ } => println!("{}", field),`
    };
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5045a6587c31f5d954ab52467b735d67)

_What if I want to describe some nested shape, but match on the whole thing?_

To do this you can use `@` in front of the pattern, known informally as the
"as-pattern". As of this writing, binding both the whole pattern plus parts of
the pattern isn't allowed.

```
#[derive(Debug)]
struct S {
    field: i32,
}

pub fn main() {
    let s = S { field: 42 };
    match s {
        S { field: x @ 10..=100, } => println!("{:?}", x),
        S { field } => println!("{}", field),
    };
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c0afcb3477a7e0b66e2aaa1a1912abf8)

_What if you don't want to specify literals or bind to variables?_

If you want to do more complicated checking on bound variables, you can use a
match guard. A guard is introduced with an `if` after the pattern, but before
the fat arrow `=>`, and the resulting value must be a boolean value, as would be
the case for other conditionals. You can't use guards on `let` and function
argument patterns.

```
#[derive(Debug)]
struct S {
    field: i32,
}

pub fn main() {
    let s = S { field: 42 };
    match s {
        S { field } if field % 2 == 0 => println!("only executes when field is even"),
        _ => println!("all remaining values go here"),
    };
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=41ed619c36361f7c9481df2c2c6b9ccc)

_What about cases where you might want to combine several possible patterns into
one match arm?_

You can combine patterns using what is known as an `or-pattern` by using a `|`
to try several patterns in a row. This way you can compress several patterns
into one match arm.

```
enum Enum {
    A,
    B,
}

pub fn main() {
    let x = Enum::A;
    match x {
        Enum::A | Enum::B => println!("matches"),
    };

    // or possibly in an if/while-let pattern match.
    let x = Some(12);
    if let Some(13) | Some(12) = x {
        println!("works");
    }
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3727a37c698f041316e3b382b11fb3ab)

_What if I want to check a pattern but I don't want all of the machinery of a
`match` statement?_

The `matches!` macro lets us write a test to see if a supplied pattern will
match a given value. The macro doesn't allow you to bind values, but it can
allow you to extend a pattern using guards which is another handy use I've found
for it (see the quirks later for more details on a precise application).

```
pub fn main() {
    assert_eq!(matches!(12, std::i32::MIN..=100), true);
    assert_eq!(matches!(None, Some(42)), false);
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f523c33e7bf8b893a4253b41c78eb7b3)

### Conclusion

And now you know the bulk there is to pattern matching in Rust! As a recap,
patterns are tested on a value, and if they line up, they will execute some branch of
logic or bind some values to identifiers, or both! You can check
complicated logic with guards, ignore portions of patterns with wildcards, bind
whole matches with as-patterns, combine patterns with or-patterns, and
test for pattern matches with the `matches!` macro. You also can use patterns in
a number of places outside of `match` and the relevant control flow expressions
such as in `let` bindings and function arguments.

### Quirks

These quirks are more around ergonomic uses of patterns rather than any
dealbreakers for writing production-grade code. You can happily skip this
section if you are still processing the information from above.

First up, nested or-patterns or in other locations, such as function arguments,
are unstable and require the `#![feature(or_patterns)]` attribute. Another way
around the nested or-patterns is to use the `matches!` macro in a guard:

```
#[derive(Debug)]
struct Container(Possibly);

#[derive(Debug)]
enum Possibly {
    A,
    B,
}

fn main() {
    let container = Container(Possibly::A);
    match container {
        // Container(Possibly::A | Possibly::B) => // won't work
        Container(inner) if matches!(inner, Possibly::A | Possibly::B) => {
            dbg!(inner);
        }
        _ => {
            dbg!("won't happen unless Possibly changes");
        }
    };
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3c204e9bed56ff5c292d1d84e80226bb)

Exclusive ranges for matching against numbers that aren't literals can be
enabled with `#![feature(exclusive_range_pattern)]`. As it stands, you can
only express inclusive ranges:

```
fn main() {
    let std::i32::MIN..=std::i32::MAX = 12; // works.
    //let std::i32::MIN..std::i32::MAX = 12; // refuses to compile.
}
```

And lastly, bindings after `@` aren't supported unless you turn them on with
`#![feature(bindings_after_at)]`. This is a bit tricky anyway given ownership
and borrowing semantics and how that plays into binding both the top-level value
and the values inside of them.

```
#![feature(bindings_after_at)]

#[derive(Debug)]
struct S {
    field: (i32, i32),
}

fn main() {
    let x = S { field: (1, 2) };
    match x {
        S {
            field: tuple @ (ref a, ref b),
        } => println!("{:?}, {} + {} = {}", tuple, a, b, a + b),
    }
}
```

[Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=2b605ec39ed4884bb4ab92b5c3cc69bc)
