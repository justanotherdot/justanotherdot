---
title: Let The Caller Decide The Return Value!
author: Ryan James Spencer
date: 2020-08-16T22:46:39.188528014+00:00
tags:
  - rust
summary: >-
  Changing function interfaces to get different return values is a chore and can weigh down refactoring sessions.
  There is a simpler way to "slot in" different functionality based on types and traits!
---

Changing function interfaces to get different return values is a chore and can weigh down refactoring sessions. There is a simpler way to "slot in" different functionality based on types and traits!

If you've used Rust long enough you are likely aware of the pattern with iterators where you can `collect` into different collections based on types. Iterators themselves don't have to keep around all of these definitions: they are implemented [via a trait named `FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html). This trait uses a trick often returned to as "return type polymorphism" to accomplish its task; the trait is generic and only requires one method, `from_iter`, to be implemented. In fact, `collect` is just a [thin wrapper around this function](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#1664-1672)!

```rust
fn collect<B: FromIterator<Self::Item>>(self) -> B
    where Self: Sized,
{
    FromIterator::from_iter(self)
}
```

What's important about this pattern of using `collect` is that it is always the same semantics; you are collecting values of an iterator into a collection. You could theoretically abuse this approach to come up with a way to do "dynamic" dispatch to wildly different behavior by specifying different types, and that *might* work depending on your use case, but the approach would feel a tad unidiomatic in the light of the larger Rust code ecosystem.

Here's a rough example using a trait to allow different types of files to be opened depending on the needs of the caller:

```rust
use std::{
    fs::File,
    io::{BufReader, Cursor, Result},
    path::Path,
};

trait Open<T> {
    fn open(&self) -> Result<T>;
}

impl Open<File> for Path {
    fn open(&self) -> Result<File> {
        File::open(self)
    }
}

impl Open<BufReader<File>> for Path {
    fn open(&self) -> Result<BufReader<File>> {
        Ok(BufReader::new(File::open(self)?))
    }
}

impl Open<Cursor<File>> for Path {
    fn open(&self) -> Result<Cursor<File>> {
        Ok(Cursor::new(File::open(self)?))
    }
}

fn main() {
    let p = Path::new("foo");
    File::create(p).unwrap();
    dbg!(
        Open::<File>::open(p).unwrap(),
        Open::<BufReader<File>>::open(p).unwrap(),
        Open::<Cursor<File>>::open(p).unwrap(),
    );
    let _: File = dbg!(p.open().unwrap());
    let _: BufReader<File> = dbg!(p.open().unwrap());
    let _: Cursor<File> = dbg!(p.open().unwrap());
}
```

[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=50bf8b2c01c6ac5fa709f7613434c9e3).

The beauty of this approach is that context can dictate which function will run. Note that I've included two approaches to showing the same thing; one using the trait's associated function syntax and the other as part of the type annotation on a `let`. I could have just as easily wrapped this function and specified concrete types on the function signature to get the same result.

In this example users of the trait can decide if the file ought to be returned "raw", wrapped for buffered access, or put in a cursor for seeking around the file's contents. This works because we have defined the trait generically, and therefore are really defining implementations for several different traits that all have the same minimal requirements. In this specific case we cannot write a generic implementation for `Open` for all `T` because there's no way for us to write a function that could return all possible `T`. That said, this trick still works even if you are not specifying the return type as part of the function calls, so long as you specify which trait implementation you want to select.

In order to use return type polymopshim you need:

- A generic trait, usually with a function or functions that use the generic type in the return value
- Implementations of the concrete versions of the return type
- Usually some final type annotation or type inference that will trigger the right implementation to be picked depending on context
