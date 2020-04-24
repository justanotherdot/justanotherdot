---
title: Cut Down On Clones With Cows
author: Ryan James Spencer
date: 2020-04-29T08:59:34.654261155+00:00
tags:
  -
summary: >-
---

At the start of a program, it is easy to `clone` data all over the place to get
things working and soon enough the program is overrun by them. Switching away from
clones can be hard because it requires fighting with the borrow checker, and
[alternative
solutions](https://www.justanotherdot.com/posts/four-ways-to-avoid-the-wrath-of-the-borrow-checker.html)
aren't quite right for the job. How do you cut down allocations from cloning as
if you were borrowing without winding up in borrow hell? Consider using a Cow.

`Cow` stands for **C**lone **o**n **Write** and they are underrated for what
they can do in this regard. On their own [cows are usually larger than their
owned or borrowed
variants](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ceab3b70e17fc69d254404ae3357d0b4),
but cutting down careless memory allocations may help improve performance.

Use a `Cow` when there is data allocated outside of a function or block and
there is some runtime logic that determines whether a write will take place.
**Cows are a useful mechanism for transferring ownership lazily by cloning data
once and only once**. The clone occurs when you call `to_mut` on the Cow, but
the all reads of the data are behind an immutable reference.

Cow's are like hybrids such that at runtime they might be borrowing data that's
already been around or they may be handing out borrows to an owned piece of data
that _they_ own.

Consider a function that replaces values in a string that we already have
allocated outside of the function. Replacing characters might mean changing the
size of the string or it could even be a case of soft failure where we replace
all invalid characters with the [U+FFFD REPLACEMENT
CHARACTER](https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html)
as is the case for
[`String::from_utf8_lossy`](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy).
We don't need to always return an owned value if we can recycle data that's
already hanging around. We can recycle in other ways, too, such as taking a
reference to a default rather than assuming it must always be allocated on the
fly, or having a collection lazily clone values as it needs to rather than
cloning the base set of values from the start. With a bit of imagination there's
a number of places that Cows can be used to possibly improve performance and cut
down on clones.
