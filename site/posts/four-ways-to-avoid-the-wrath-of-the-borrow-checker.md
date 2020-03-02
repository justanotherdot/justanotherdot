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

Maybe you've tried to write a simple program in Rust using references that would
normally have taken less than an hour in C only to find yourself _hours_ later
still fussing about with the Rust compiler. If the borrow checker seems too
restricting, here are four ways to loosen its grip.

### Shared Ownership with `Arc` or `Rc`

Shared ownership is what most garbage collected languages support. This is done
using a reference counting to objects in memory. We can mimic this in Rust with
the wrapper type `Rc`. If you plan on reference counting in multi-threaded code
you can use `Arc` where the `A` stands for `Atomic`.

Passing around an `Rc` means that if someone wants to jointly own the data, they
can simply call `clone` on the `Rc`. This can be used as a drop-in replacement
for places where you would borrow. Since these reference bumps count as new
owners there is no borrowing at all. However, now that we can express shared
ownership we also express a _graph_ and graphs can have cycles (place where
pointers loop back on themselves). A cycle in a graph means a value may never be
deallocated, hence any self-referential structure poses a memory leak in our
program. You can avoid this by having the pointer that "ties the knot" be a
`Weak` pointer, which just means it's a non-owning pointer. A classic example is
with a cache: you want to have entries in the cache to objects owned outside of
the cache but you don't want the entries to count towards owning anything,
otherwise keeping the cache around means keeping all of the memory! Also, `Rc`
means you won't be able to take mutable borrows to the contents. This is easily
remedied with the use of `Rc<RefCell<T>>` or even `RwLock` as we'll see later.

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

### Interior Mutability with Cell or RefCell

Exterior mutability (otherwise known as "inherited mutability") is great because
it lets us know what things are actually changing beneath us. But sometimes
clients don't care that some housekeeping state is changing underneath an
operation. Perhaps we memoize the result of a function or manipulate a counter;
in both of these cases, with exterior mutability, the function wrapping this
action would have to be marked as mutable in some way, but if we want to keep
things looking immutable on the surface, we can use `Cell` or `RefCell` instead.

To give a concrete example with memoization, you might have an expensive
computation that you only want to do once and stash the result. As such, you
have a function that only need to be mutable for this one time and can be
immutable the rest of the time, so it doesn't make sense to have it marked as
`mut`. Whatever the result type is, we can wrap that in a `Cell` or `RefCell`,
depending on type: `Cell` is generally for things that support `Copy` and
`RefCell` for the rest.

As `RefCell` uses dynamic borrow checking, it can panic if multiple mutable
borrows are taken to the contents. `Cell` doesn't suffer from the same issue as
it moves the values in and out of the internals of the `Cell`. As such, you may
want to use something like `RwLock` if you are using an `Rc<RefCell<T>>` for
multi-threaded code. `Rc<RefCell<T>>` is a common way to have a shared object,
such as a `HashMap`, across several owners, but still mutate it. If one used
`Rc::get_mut` one would need to mark the `HashMap` itself as `mut`.

### Duplicate the data

Often people think that coming to Rust means programs should be completely
devoid of `clone` but if you think about the language you may be coming from,
whatever `clone`ing you are doing is in Rust probably pales in comparison.

You don't need to feel bound to a `clone`less program by default. By abandoning
this idea of a slim program from the outset and move towards something far more
flexible. This generally means having duplicate formats of a data structure for
varying purposes such as one be a game map where walls are located whereas
another could be where someone has explored and yet another could contain items
on each tile, etc. It can also mean having a duplicate you want to make changes
to, leaving the original in-tact. This is more of the pure approach functional
programming languages tend to take, but these languages can also make particular
optimizations around immutability such as persistent data structures or
"sharing" of data since _everything_ is immutable by default. Here's an example
that having some duplication of data might help. Perhaps you are trying to
iterate over a collection and mutate it:

```
let mut xs = vec![1,2,3];
for x in xs.iter_mut() {
  if x % 2 == 0 {
    xs.push(x+1);
  }
}
```

In fact, this fails because we are borrowing to `xs` mutably twice! Once when we
construct the iterator and another time when we push to the `Vec`. This is a
classic "modify a data structure while you iterate over it" issue. However, we
could easily do this:

```
let mux xs = vec![1,2,3];
let ys = xs.clone();
for x in xs.iter() {
  if x % 2 == 0 {
    ys.push(x+1);
  }
}
```

and hum along. In fact, we can keep these allocations to be short-lived, which
may or may not be a performance issue but that can always be addressed later
with proper profiling.

### Single ownership and data pipelines

Lastly, you can try to go away from references entirely. Ownership is ideal for
the kinds of problems best described as transforming values into other kinds of
values.

Pipelines have stages or steps. Steps may build up required changes or apply
earlier changesets. Pipelines are useful for a variety of solutions. Parsers,
compilers, streaming analysis, and so on, however that isn't the end of it.
Configuration could be seen as a stream that updates when new values are added.
This isn't to say all pipelines are pull models but it is to say the solutions
are broad.

If the use of borrows is for performance (want to ensure a large structure isn't
`memcpy`d to a function), this type of optimization should already happen behind
the scenes with move semantics; an owned type will typically get passed by
`memcpy` for smaller sized objects and may be passed as a pointer for larger
objects. This means that there is no mechanical difference for borrows besides
marking the earlier variable as uninitialized meaning we always have no more
than one owner of a value at a time.

When data flows through a pipeline, doing it all by mutable reference can
achieve the same effect and the owner doesn't relinquish control. Although this
_might_ mean there are less allocations, it will mean each step of the way we
are passing a pointer where copying an object might have actually been faster.
This is also limiting in that we can only have on mutable borrow at a time! If
callers allocate the objects they own and request they be transformed into a new
shape, any number of threads could pass values into this pipeline for changes
and be content that value changes will be isolated from one-another.

### Recap

1. Use `Rc` or `Arc` when you want to quickly get past tricky borrow issues and
   want to convert back to borrows possibly later in time to reap the benefits
   they offer
2. Use `Cell` or `RefCell` when you have an API that doesn't need to expose
   mutability to a client
3. Get comfortable with cloning data for multiple purposes to avoid conflicting
   borrows
4. Don't borrow at all and create pipelines that pass ownership from step to
   step, producing a final, desired result
