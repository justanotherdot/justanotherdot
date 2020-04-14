---
title: Skip Unnecessary Allocations In Your Collections
author: Ryan James Spencer
date: 2020-04-14T09:47:45.519198005+00:00
tags:
  - rust
  - performance
summary: >-
  Rust's standard library offers a lot of neat dynamically-sized data structures
  for use in Rust programs. They are quite performant, but the allocations they
  perform may add up and cause performance issues in your programs.
---

Rust's standard library offers a lot of neat dynamically-sized data structures
for use in Rust programs. They are quite performant, but the allocations they
conduct behind the scenes to grow may add up and cause performance issues in
your programs.

Rust intentionally avoids costly uses of `new` in a program by having the
allocation be empty by default, including types outside of`std::collections`,
too, such as `String::new`.

The backing store usually grows with a doubling strategy, and the growth tends
to happen right as it is needed, as is the case for `Vec`, see
[here](https://github.com/rust-lang/rust/blob/42abbd8878d3b67238f3611b0587c704ba94f39c/src/liballoc/raw_vec.rs#L462-L464)
and
[here](https://github.com/rust-lang/rust/blob/42abbd8878d3b67238f3611b0587c704ba94f39c/src/liballoc/raw_vec.rs#L476-L540)
for references to code as of this writing, but it may not always be the same
story for other collections. I strongly encourage looking at the actual source
code for the standard library when you are curious. Rust uses the language of
**capacity** to designate the total possible amount of memory the backing store
has room for and **length** to designate the total number of actual values in
the data structure.

One of the core tenants of optimization is to avoid doing needless work. Putting
data on the heap isn't necessarily expensive if you've already paid the price
upfront for allocating it. Doing work in this way is called **amortization**.
Imagine I have to store 4096 things in a vector. By default, the vector grows in
powers of two with capacities of 0, 2, 4, 8, 16, 32, 64, and so on, in that
order. That's already six allocations I've mentioned and not done reaching the
final size. Avoiding unnecessary work is at the heart of performance
optimization and these are intermediate steps are unnecessary!

A fantastic part of the Rust standard library collections is they tend to have
common interfaces precisely for this sort of thing! You can avoid these
allocations by using`with_capacity` if you know the value or upper bound you
need initially. If you already have the data structure hanging around, you can
also call `reserve` to request additional capacity to avoid needless allocation.

The way allocation happens with the doubling strategy _is_ a form of
amortization. As the collection grows in powers of two, the number of calls
reduces, but the cost of growing the vector increases. Each time the vector
grows, it will copy all the values over to a new backing store. In general, any
time you think you can use a big chunk of data up front, you should allocate the
full capacity, but if the exact amount you are requesting is unknown, isn't that
a bit wasteful? An alternative strategy where the amount may only be partly
known is to request a large chunk of memory and size it down either with
`shrink_to_fit` or `resize`, but be careful with `resize` as it may truncate the
collection if you aren't careful!

It is always best to get empirical data on how to reasonably size the collection
upfront or while the program is running. If we instead take a chaotic approach
to allocations we may do more harm than good. At the end of the day, the reason
why these data structures grow on their own is to avoid thinking about them
_until_ performance is an issue and we reveal that spending our time on this is
important through profiling.
