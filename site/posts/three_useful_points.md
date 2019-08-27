---
title:
author: Ryan James Spencer
date: 2019-08-27T09:00:00Z
tags: [software, tips]
---

People I respect who hack on code are curious, patient, cordial, and efficient.
In fact, I feel as though these role models are truly efficient because of the
first three points. There is one role model I know, and have the pleasure to
work with, who does a set of things I have found truly invaluable and have
wanted to share.

### Principles

You are structuring a system or a program and you want a framework for how to
make decisions. You want to retain consistency across decisions rather than
things being based on ad hoc requirements or spur of the moment reactions.

[Ray Dalio wrote a whole book about
principles](https://www.goodreads.com/book/show/12935037-principles).

### Patterns

Lightweight patterns provide re-use. Keeping them lightweight and setting up a
system of trust around them provides a framework for only keeping the ones that
pay off rather than letting them bog you down or keep you in poor processes.

Say you are writing some code and you discover a way to iterate across
directories. You can re-use that pattern over and over again until it betrays
you. In that case you have two options: change the pattern or ditch it. Ditching
is a perfectly acceptable form of improving your set of usable patterns and
might even be faster than trying to tweak a non-working pattern.

### Refinement of Feedback Loops

I could easily write a whole ream of articles on feedback loops and I probably
will. It's not enough to simply set up a feedback loop. Refinement is absolutely
crucial for getting the most of their usage, and feedback loops are everywhere
once you start seeing them! From automatically running type checking or tests
using something like [`entr`](http://entrproject.org/) or even learning from
prior failures while also minimizing risk to tinker on proof-of-concepts, they
form the basis of improvement.
