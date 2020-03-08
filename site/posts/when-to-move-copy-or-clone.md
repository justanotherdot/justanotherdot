---
title: When to move, copy, or clone?
author: Ryan James Spencer
date:
tags:
  - rust
summary: >-
  Do you understand ownership and borrowing in theory but find it hard in
  practice? Do the differences between things like `iter` and `into_iter` still
  confuse you? Is the difference between `Copy` and `Clone` still unclear? I will
  shed some light on practical examples that should help you gain a better grip on
  owning and borrowing values.
---

Do you understand ownership and borrowing in theory but find it hard in
practice? Do the differences between things like `iter` and `into_iter` still
confuse you? Maybe the difference between `Copy` and `Clone` is still unclear? I
will shed some light on practical examples that should help you gain a better
grip on owning and borrowing values.

As you may know, all values in Rust need an owner. *Owners are about
responsibility*; some resource, usually, but not always, memory, is allocated and
the responsibility for releasing it is up to the owner. Ownership, or rather,
responsibility, is only transferred on a move, hence borrowing, not counting
towards releasing resources, is a *view* into the data. Rust's borrowing rules
mimic the solution to the readers-writers problem of concurrency; there may be
any number of readers but no writers and only ever one writer and no readers.
These two states are the same as immutable borrows and mutable borrows,
respectively.

When we rebind values that aren't `Copy`, by default we use move semantics and
transfer ownership to the new identifier. However, if somthing is `Copy` this
action now performs a bit-wise copy of the contents. An `i64` that is
re-assigned to a new variable will be a bit-wise copy. Contrast this to `Clone`
where the copying is explicit with the call to `Clone`. *Thus both `Clone` and
`Copy` signify copying of some kind, whether cheap or expensive, but the choice
is dependent on when the copying is preferred.*

### tl;dr

1. I borrow values to avoid producing new values. In other words, I re-use
   values that are already hanging around so as not to be wasteful with
   allocations.

2. I copy/clone based on how I want to reduce allocations in the face of needing
   to duplicate data, picking to allocate automatically or explicitly respectively.

3. I own values when I want total control of the data in question. I like to
   think of this as *data recycling*.

### When to borrow

Firstly, we can make a Vec of references, since the owner still lives while the
references live we don't need to allocate any new data.


```
struct Wrapper {
  id: i64,
}

let values: Vec<Wrapper> = vec![
    Wrapper { id: 1 },
    Wrapper { id: 2 },
    Wrapper { id: 3 },
];
let xs: Vec<&Wrapper> = values.iter().collect();
```

### When to clone or copy

Next up, we might want to keep two copies of our `values`, if we change the
`Wrapper` to derive `Clone` we can use `cloned` on our iterator which is
functionally the same as `.map(|x| x.clone())`:

```
#[derive(Clone)]
struct Wrapper {
  id: i64,
}

let values: Vec<Wrapper> = vec![
    Wrapper { id: 1 },
    Wrapper { id: 2 },
    Wrapper { id: 3 },
];
let xs: Vec<Wrapper> = values.iter().cloned().collect();
```

By default, most primitives are `Copy` because it's easy enough and usually
performant for the compiler to bitwise copy them. Since our `Wrapper` type in
the previous examples is just wrapping up a primitive `i64` integer, we can make
it also derive `Copy`:

```
#[derive(Copy)]
struct Wrapper {
  id: i64,
}

let values: Vec<Wrapper> = vec![
    Wrapper { id: 1 },
    Wrapper { id: 2 },
    Wrapper { id: 3 },
];
let xs: Vec<Wrapper> = values.iter().copied().collect();
```

You might use `copied` if you don't want to write `.map(|x| *x)` if you happen
to have a collection of borrowed values at your disposal (imagine you are passed
a `Vec<&Wrapper>`), this could be handy. The same logic stands for `cloned`. The
case is a little bit different for `Copy`, though. If we can own the iterator
with `into_iter` then any move of the values will result in a bitwise copy. This
is why you will sometimes see the rust compiler complain that a value is moved
and doesn't implement the `Copy` trait: it can't make a copy for you and it also
can't re-use a moved value.

### When to own

Ownership is the basis of why we don't need garbage collection in Rust. Passing
an owned value across several different method calls could make copies or pass
pointers depending on what optimizations the compiler wishes to perform, hence
they could be `memcpy`s or as copied pointers. Regardless of how they work under
the hood, they prevent a host of bugs by ensuring _only one thing_ has the
responsibility of finalizing the release of memory.

Expecting owned values is a nice way to push the decision to allocate on the
caller. If the caller wants to keep the value it owns, it must clone the value
itself, instead of guessing if a clone is happening elsewhere. More importantly,
writing code that expects values to be owned exposes the intent that I want to
have full control over the memory to do as I please, rather than trying to work
around what may already exist. This is why anytime you want to transform
something from one shape to another and don't care much or at all about the
original shape, taking ownership is the right choice.

```
#[derive(Copy)]
struct Wrapper {
  id: i64,
}

let values: Vec<Wrapper> = vec![
    Wrapper { id: 1 },
    Wrapper { id: 2 },
    Wrapper { id: 3 },
];
let values: Vec<Wrapper> = values.into_iter().collect();
```

Note how I re-assign `values` after the transformation; albeit not necessary, it
does let me think a bit less about re-naming the original binding that I can't
use anymore.
