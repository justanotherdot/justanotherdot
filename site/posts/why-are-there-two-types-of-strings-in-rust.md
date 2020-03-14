---
title: Why Are There Two Types of Strings In Rust?
author: Ryan James Spencer
date: 2020-03-14T00:45:46.961720071+00:00
tags:
  - rust
summary: >-
  Understanding the distinction between str and String can be painful if you
  need to get something done in Rust now. Rust doesn't sugar coat a lot of the
  ugliness and complexity of string handling from developers like other languages
  do and therefore helps in avoiding critical mistakes in the future.
---

Understanding the distinction between `str` and `String` can be painful if you
need to get something done in Rust _now_. Rust doesn't sugar coat a lot of the
ugliness and complexity of string handling from developers like other languages
do and therefore helps in avoiding critical mistakes in the future.

By construction, both string types are valid UTF-8. This ensures there are no
misbehaving strings in a program. A `char` is always four-bytes in Rust, but a
string doesn't have to be composed of just four-byte chunks (that would be a
UTF-32 encoding!). Being UTF-8 means that Strings can be encoded with
variable-width code points, but you can iterate across the `char`s if you want
without them being stored as such.

I'll cover the remaining difference between a `String` and a `str` through
arrays, vecs, and slices.

An array is a contiguous chunk of memory where every element is the same type
and adjacent. Arrays are, however, of a fixed size. If we want to actually grow
or shrink an array we can turn to a `Vec` which is sometimes known as a
"resizable array". This type abstracts away the housekeeping around allocating
bigger or smaller arrays.

A vec grow as elements fill the backing array near or at capacity. Vecs also
shrink to size if requested. The perks of ownership in Rust mean we, the vec,
can do whatever we please to the data we own. We can always borrow owned things
to temporarily read or change data. Why do you need more?

A slice is a view into a portion, or _slice_, of owned, contiguous memory.
Whenever we have a slice we know we can access its elements safely without
exposing any elements outside of the portion described by the slice and without
copying any data over to a new owner. Slices give us the capacity to provide
entire views of the original data rather than just a segment.


This relationship between an owned piece of data and a view into an owned piece
of data is pervasive in Rust. Not every view may exclude access outside of its
elements but it may provide a copy-free access such as an `Entry` for a
`BTreeMap` or a `Cursor` to a `File`.

This is the same relationship between `String` and `str`. A `String` is the
`Vec` and `str` is the slice. Since a slice is its own type, we can borrow it to
change or read as we please. This is the difference between `str` and `&str` in
that you will only ever manipulate a `&str` but it's technically a borrowed
"string slice" `str`.

There is one bit of "magic" that Rust allows which is that taking a borrow to an
owned string to a function will cast it to a string slice for you.

```
let s = String::new();
fn takes_a_string_slice(the_string: &str) {
  // reads the_string.
}
takes_a_string_slice(&s);
```

This is a convenience so that you don't have to describe the bounds as you would
for an array or vector slice, a la `&xs[0..n]`, although you _can_ use the same
syntax to create a slice into a portion of a string if you want.

As a final point, the backing store of a `String` is actually `Vec`; `String`
just brings along the requirement that the contents are valid UTF-8 and heaps of
convenience functions, as does `&str`. A slice is what we commonly call a "fat
pointer" which consists of two words: one pointing to the start of data and
another dictating the length of the content. In this sense casting between a
slice and back is cheap in the sense that we do not copy any data besides
creating a fat point and perhaps re-using it when we borrow.
