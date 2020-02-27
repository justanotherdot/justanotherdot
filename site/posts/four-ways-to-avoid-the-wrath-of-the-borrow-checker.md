---
title: Four Ways To Avoid The Wrath Of The Borrow Checker
author: Ryan James Spencer
date: 2020-03-01T06:07:25.271933517+00:00
tags:
  - rust
summary: >-
  You've likely tried to write a simple program using references that would
  normally have taken you an hour in C only to find yourself _hours_ later still
  fussing about with the Rust compiler. If the borrow checker seems too
  restricting, here are four ways to loosen its grip!
---

You've likely tried to write a simple program using references that would
normally have taken less than an hour in C only to find yourself _hours_ later
still fussing about with the Rust compiler. If the borrow checker seems too
restricting, here are four ways to loosen its grip.

1. Shared Ownership with `Arc` or `Rc`

Shared ownership is what most garbage collected languages support and is usually
so because of how memory is laid out in the garbage collector. This is done
using a reference counted pointer, `Rc`. If you plan on having reference counts
to update correctly when using multi-threaded code, you'll want `Arc` where the
`A` stands for `Atomic`.

Passing around an `Rc` means that if someone wants to jointly own the data, they
can simply call `clone` on the `Rc`. This can be used as a drop-in replacement
for places where you would borrow. Since these reference bumps count as new
owners there is no borrowing at all. However, now that we can express shared
ownership we also express a _graph_ and graphs can have cycles (place where
pointers loop back on themselves). A cycle in a graph means a value may never be
deallocated, hence any self-referential structure poses a memory leak in our
program.

You can avoid this by having the pointer that "ties the knot" be a `Weak`
pointer, which just means it's a non-owning pointer. A classic example is with a
cache: you want to have entries in the cache to objects owned outside of the
cache but you don't want the entries to count towards owning anything otherwise
keeping the cache around means keeping all of the memory, even when things
outside of the cache are done using it!

"When in doubt, reference count" is appropriate for places where laying out
borrows and static lifetimes can be a pain and you want to get things passing
quick. Places where you temporarily use an `Rc` can easily be targeted for
borrows, so going back to fix things is clear. It may take some jiggling to get
things into place but at least it can happen later down the line when you've got
the breathing room, perhaps.

`Rc` can be particularly handy when you want to pass around function references
in all sorts of ways. I used `Rc` extensively when porting a functional library
from F# and Haskell directly into Rust and needed to easily get mutual recursion
working quickly where using direct references or owned trait generics (e.g. F:
Fn(A) -> B). I was later able to swap out the calls to references, which meant
the ergonomics of the first call simplified to borrowing rather than wrapping
the closures in question in `Rc`s.

3. Interior Mutability with Cell or RefCell

Exterior mutability (otherwise known as "inherited mutability") is great because
it lets us know what things are actually changing beneath us. But sometimes
clients don't care that some housekeeping state is changing underneath some
operation. Perhaps we memoize the result of a function or manipulate a counter;
in both of these cases with exterior mutability, the function wrapping this
action would have to be marked as mutable in some way, but if we want to keep
things look immutable on the surface, we can use `Cell` or `RefCell` instead.

If you have gone the `Rc` route for anything but need to change the contents to
the referenced content, you'll find you can't easily do this. Wrapping a
`RefCell` with an `Rc` gives you this extra power; now you have easy sharing of
references and mutability that is suitable for most single-threaded
applications. If you do happen to start charging into multi-threaded territory,
you'll want to look into a lock such as an `RwLock` or other sync primitives
that core or external libraries might provide.

To give a concrete example of the memoization example, you might have an
expensive computation that you only want to do once. You have a function that
only need to be mutable for this one time and can be immutable the rest of the
time. You can wrap whatever you are memoizing in a `Cell` or `RefCell` and keep
the function call with immutable references.

You could also keep an immutable reference to something that has `Cell` or
`RefCell` and still manage to change the contents although technically with
`RefCell` this might result in a panic if you are running any multi-threaded
code that would take more than one mutable borrow to the `RefCell` at a time.

2. Duplicate data

Often people think that coming to Rust means programs should be completely
devoid of `clone` but if you think about the language you may be coming from,
whatever `clone`ing you are doing is still probably pales in comparison to what
that language is doing under the covers.

You don't need to feel bound to a `clone`less program by default. By abandoning
this idea of a slim program from the outset and move towards something far more
flexible. This generally means having duplicate formats of a data structure for
varying purposes such as one be a game map where walls are located whereas
another could be where someone has explored and yet another could contain items
on each cell, etc. It can even mean having a duplicate you want to make changes
to, leaving the original in-tact; this is more of the pure approach functional
programming languages tend to take, but these languages can also make particular
optimizations around immutability such as persistent data structures or
"sharing" of data.

If you are having issues with the borrow checker because you are mixing types of
borrows on a single data structure, you can always leave one copy as a reference
for reading and the other as the new version that may possibly replace the
reference. Here's a classic example:

```
let mut xs = vec![1,2,3];
for x in xs.iter_mut() {
  if x % 2 == 0 {
    xs.push(x+1);
  }
}
```

Here we try to manipulate a structure while iterating over it. However, we could
easily do this:

```
let mux xs = vec![1,2,3];
let ys = xs.clone();
for x in xs.iter() {
  if x % 2 == 0 {
    ys.push(x+1);
  }
}
```

3. Single ownership and data pipelines

Single ownership can be seen as ideal for cases where you want to make a
pipeline or a stream where data transforms at each step as it flows through.
Moving values is about protection against aliasing. If only one thing owns the
value, and we have strict rules about references, then we can be sure that a
reference won't point to something that is deallocated. There is no magic going
on here; for example, moving an argument into a function might be a `memcpy` if
the object is small enough or it could be passed by pointer, it's up the
optimizations in place to decide. What a move _really_ does is simply ensure the
old binding doesn't point to any value whatsoever anymore (rust marks the value
as uninitialized memory and forbids access statically to uninitialized memory).

A great mental model for thinking about this sort of pipeline is that steps
either generate changesets or apply changesets. You can own a value, calculate
what you need to do with it, and pass it onto another step that may apply that
change and so on.

Iterators may not necessarily own the value passed them, although technically
they do with `into_iter` and this allows things like `collect` to re-use the
allocated memory if they wanted to. This is similar to making a string with a
`Vec` full of `char`; sure, there will be a check to verify the contents are
valid UTF-8, but once validated, the `Vec<char>` can be owned by the new
`String` and be re-used rather than copied.
