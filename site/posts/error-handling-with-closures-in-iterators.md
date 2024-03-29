---
title: Error Handling With Closures In Iterators
author: Ryan James Spencer
date: 2020-02-03T09:38:54.087319182+00:00
tags:
  - rust
  - error handling
summary: >-
  Iterators give us a wonderful array of functional-style combinators. Past
  readability, the rust compiler can occasionally optimize iterators better than
  it can for-loops, too. However, as iterators work by taking closures it can be
  confusing on how to best handle them compared to using classic for-loops. Here's
  a toy example:
---

Iterators give us a wonderful array of functional-style combinators. Past
readability, the rust compiler can occasionally optimize iterators better than
it can for-loops, too. However, as iterators work by taking closures it can be
confusing on how to best handle them compared to using classic for-loops. Here's
a toy example:

```
fn parse_str_of_i32(input: &str) -> Vec<i32> {
    input.split(",")
        .map(|char| char.parse().unwrap()) // `unwrap`!
        .collect()
}

let input = "1,2,3,4,5,6,7,8,9,0";
let numbers = parse_str_of_i32(input);
assert_eq!(numbers, vec![1,2,3,4,5,6,7,8,9,0]);
```

This works but it has an `unwrap` which means that if callers pass invalid
strings, such as `"oh boy, here we go again"`, it will panic, which gives
callers of this code little control when things go wrong. How can we convert
this to use `Result` and be more ergonomic? Consider the for-loops variant,
first:

```
use std::num::ParseIntError;

fn parse_str_of_i32(input: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut numbers = vec![];
    for char in input.split(",") {
        numbers.push(char.parse()?)
    }
    Ok(numbers)
}

let input = "1,2,3,4,5,6,7,8,9,0";
let numbers = parse_str_of_i32(input).unwrap();
assert_eq!(numbers, vec![1,2,3,4,5,6,7,8,9,0]);
```

You might think this means if you want to use error handling while iterating you
need to have a for-loop instead of using Iterator but you can still have an
Iterator and have get the same type signature above for our parser!


```
use std::num::ParseIntError;

fn parse_str_of_i32(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(",")
        .map(|char| char.parse())
        .collect()
}

let input = "1,2,3,4,5,6,7,8,9,0";
let numbers = parse_str_of_i32(input).unwrap();
assert_eq!(numbers, vec![1,2,3,4,5,6,7,8,9,0]);
```

How does this work? `collect` knows how to take an Iterator of `Result`s and
turn it into an `Result<Vec<A>, B>`. At the first sight of an `Err` the whole
expression will become the `Err` case but if everything works out with `Ok` then
the Iterator will take all the values into their own `Vec` and return `Ok` of
the enclosing `Vec`. This is sometimes referred to as a "transpose" and you can
see similar 'inside-out' behaviour elsewhere, including `Result`
[itself](https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose).

You can also specify collections other than `Vec`. If `A` is something that can
be `collect`ed into some container `V`, then an `Iterator<Item=Result<V, B>>` is
possible. Have a poke around the `FromIterator` [trait
docs](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) to get a
better sense of what `collect` can roll up!
