---
title: Picking Variants or Fields Out Of Collections of Enums
author: Ryan James Spencer
date: 2020-06-10T10:06:54.545131387+00:00
tags:
  - rust
summary: >-
  Occasionally you want to pull out a specific field or variant out of one or more
  enums. Doing this with pattern matching is tedious and verbose, but there's a
  simple way to use methods and combinators to do exactly what you want.
---

Occasionally you want to pull out a specific field or variant out of one or more
enums. Doing this with pattern matching is tedious and verbose, but there's a
simple way to use methods and combinators to do exactly what you want.

There's a pattern I use to solve both of these. If you already have the
collection in hand, you can write a simple method that operates on the type
named something like `as_foo` where foo is the name of the variant or field name
you are after. There's a clippy lint that says `as_*` functions should always
take references so I've followed the lint in the following example but it
doesn't really matter what you name the method and it's fine to have the method
take ownership of the value, too, if that makes sense for your use case. When
you define a method you can use it either with the basic method syntax
`value.as_foo()` or you can access it as an associated function e.g.
`Type::as_foo(value)`. Then we can use either method in tandem with the
`filter_map` or `flat_map` methods of an iterator. I personally prefer the more
terse way of passing the associated function instead of the closure, which is
sometimes referred to as "point free" style where the arguments, or points, are
not mentioned:

```
#[derive(Debug, Clone, PartialEq)]
enum E {
    A { x: i32 },
    B { x: i32 },
}

impl E {
    pub fn as_x(&self) -> Option<i32> {
        Some(match self {
            E::A { x } => *x,
            E::B { x } => *x,
        })
    }

    pub fn as_b(&self) -> Option<&E> {
        match self {
            x @ E::B { .. } => Some(x),
            E::A { .. } => None,
        }
    }
}

pub fn main() {
    // Method access off type.
    let a = E::A { x: 1 };
    let b = E::B { x: 2 };
    assert_eq!(a.as_x(), Some(1));
    assert_eq!(b.as_x(), Some(2));

    // Associated function on impl.
    let a = E::A { x: 1 };
    let b = E::B { x: 2 };
    assert_eq!(E::as_x(&a), Some(1));
    assert_eq!(E::as_x(&b), Some(2));

    // In a collection.
    let a = E::A { x: 1 };
    let b = E::B { x: 2 };
    let as_and_bs = vec![a, b];
    let xs = as_and_bs.iter().filter_map(E::as_x).collect::<Vec<i32>>();
    assert_eq!(xs, vec![1, 2]);

    // Selecting a field as a dummy pattern match.
    let b = E::B { x: 2 };
    let xs = Some(&b).and_then(E::as_b);
    assert_eq!(xs, Some(&E::B { x: 2 }));
}
```

[Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5bb110c4a9981caa3bc3317b9e2350c3).

Pattern matching is powerful but sometimes you can reduce the number of explicit
pattern matches you perform by taking advantage of functions and combinators and
keeping the logic small and simple, letting you reason about what the result
ought to be on the other end. In the last case using `and_then` above, we can
reason that whenever we call `as_b` we're sure to get a single pattern match if
we must simply checking for `Some(E::B { .. })` or `None`. The compiler may not
understand that, though, and you'll most likely have to include a wildcard case,
but the brilliance of combinators is that you can chain them together in a
pipeline similarly to the fluid interface that iterators present.
