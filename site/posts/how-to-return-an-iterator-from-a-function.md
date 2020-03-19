---
title: How To Return An Iterator From a Function
author: Ryan James Spencer
date: 2020-03-19T09:40:42.111963174+00:00
tags:
  - rust
summary: >-
  Iterators are handy ways to describe the potential for looping over data but
  without eagerly evaluating it. In other words, we can describe the shape of
  looping over things but we only do work when we call something like `collect`.
  The compiler can do several optimizations around iterators and each `collect`
  needs to allocate memory to store our results. To avoid a lot of needless
  allocations it pays to returns iterators sometimes from functions.
---

Iterators are handy ways to describe the potential for looping over data but
without eagerly evaluating it. In other words, we can describe the shape of
looping over things but we only do work when we call something like `collect`.
The compiler can do several optimizations around iterators and each `collect`
needs to allocate memory to store our results. To avoid a lot of needless
allocations it pays to returns iterators sometimes from functions.

Alright, that's great but you're hitting a wall returning one from a function.
I'll provide two ways you can do just that:

We could use the `impl Trait` syntax where `Trait` is the trait in question we
want to return. In this case, we'd have:

```
fn unboxed_iterator() -> impl Iterator<Item = usize> {
  (0..3).into_iter()
}
```

This is where the compiler will determine the exact type that is being returned
and substitute that in. This approach, albeit with static dispatch, has its
limits. You can read Bodil Stokke's [wonderful introduction to parser
combinators](https://bodil.lol/parser-combinators/) as an example where the
`impl Trait` approach starts to get too complex and require turning to our next
approach of `Box`ing the iterators. This static approach also doesn't work when
we have anonymous types, such as with async functions. Marking a function as
`async` has support from the compiler to return the right type.

We could also use a `Box`. It is the simplest way to package up an iterator but
at the cost of allocation. Anything behind a `Box` is allocated on the heap.

```
fn boxed_iterator() -> Box<dyn Iterator<Item = usize>> {
  Box::new((0..3).into_iter())
}
```

We have to pay the price of dereferencing a pointer each time we want to deal
with this specific boxed iterator. We can manipulate both iterators in the same
way because `Box` implements `Deref` which lets you access the methods
underneath. So we can call `map` and friends on the resulting `Box`. Both of
these forms can usually be used with one another, as, for example, we can chain
both together since they both can be turned `IntoIterator`s.

```
fn unboxed_iterator() -> impl Iterator<Item = usize> {
  (0..3).into_iter()
}

fn boxed_iterator() -> Box<dyn Iterator<Item = usize>> {
  Box::new((0..3).into_iter())
}

fn main() {
  dbg!(boxed_iterator().chain(unboxed_iterator()).collect::<Vec<_>>());
}
```

The above prints:

```
[src/main.rs:10] boxed_iterator().chain(unboxed_iterator()).collect::<Vec<_>>() = [
    0,
    1,
    2,
    0,
    1,
    2,
]
```

You preferably want to use the `impl Trait` approach as much as you can and
fall back to the `Box` approach when that starts to fail or become inordinately
slow to compile. With time this restriction may go away as work on the compiler
continues.
