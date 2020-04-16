---
title: "The Lowly Assert: Involution"
author: Ryan James Spencer
date: 2019-10-28T19:43:10.534012654+00:00
tags: [the lowly assert]
summary: >-
  As part of The Lowly Assert series I wanted to go over some mathematical
  patterns. Filling your arsenal of known properties helps with recognizing common
  ways functions, systems, etc. are, and should continue, to behave.
---

As part of [The Lowly
Assert](https://www.justanotherdot.com/posts/the-lowly-assert.html) series I
wanted to go over some mathematical patterns. Filling your arsenal of known
properties helps with recognizing common ways functions, systems, etc. are, and
should continue, to behave.

Occasionally you'll write functions that flip-flop: when you call the function
multiple times in a row, chunks seem to cancel out. Mathematics calls these
functions ["involutive"][1]. Negating a number or a boolean twice gets you back
to the original value. Involution is handy to recognize because it's a simple
assertion:

```
x == f(f(x))
```

The classic property based testing example of this is the
`reverse(reverse(some_list))` you'll see in endless tutorials and getting
started guides on the subject. When you reverse a list you expect to simply flip
the contents one end to the other, but this may not be immediately applicable to
day-to-day affairs. Here's one: a function that opens a dialogue box has two
states, open and closed, and is commonly tested for involutivity; if you didn't
have the toggling action you'd see no feedback after clicking!

But involution doesn't have to be about functions applied exactly twice.
Rotating around a point back to an original direction can be done with various
divisions: two rotations of pi, four rotations of pi/2, and so on. Precisely,
involution requires that the application be a single function, but we can
sometimes be a bit less rigorous and claim that two or more actions that
eventually cancel out are involutive: removing and adding an item in a
collection or returning someone's money in an exchange.

With this we can hopefully see the application isn't on any end of the scale:
from small to big, XORing bits to whole product flows that might 'restart' the
user in a funnel. Properties like these are worth their weight in gold because
they are useful in almost any type of testing and are definitely a shining
example of The Lowly Assert.

[1]: https://en.wikipedia.org/wiki/Involution_(mathematics)
