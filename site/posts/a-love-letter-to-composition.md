---
title: A Love Letter to Composition
author: Ryan James Spencer
date: 2019-11-06T20:07:08.632954959+00:00
tags: [composition, principle, pattern]
---

Using composition gives you superpowers. It is by far the most practical
experimentation tool I know.

The [dot (.)
operator](http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Function.html#v:.)
is my favorite infix operator in Haskell. Statically typed languages help ensure
that [function composition](https://en.wikipedia.org/wiki/Function_composition)
is structurally sound before anything is run. Composition of two functions means
the type of the output of the first function must equal the type of the input of
the next function. Many languages now have a pipe operator which is the
composition operator in reverse. Some even use pipe or dot to write flow of
execution top-to-bottom or bottom-to-top, given how you can stack the calls.

This isn't just an article about the usefulness and specifics around function
composition itself. Composition as a concept forms a basis of for problem
solving and systems of proof. By decomposing a system or problem into parts we
can scrutinize and, thus, verify them for use in constructing the same or
potentially different solutions, proofs, and so on. Having solid building blocks
means we can play around with different arrangements. Playing around with these
building blocks and assumptions is how
[mathematics](https://www.goodreads.com/book/show/192221.How_to_Solve_It) and
[experimentation](https://www.justanotherdot.com/posts/may-you-be-the-author-of-two-to-the-n-programs.html)
works at its core.

Composition also forms part of the basis of a fascinating branch of mathematics
known as [category theory](https://github.com/hmemcpy/milewski-ctfp-pdf).
Envision a type of mathematics that encodes any arbitrary concept as a
graph-like diagram to explore general structures and relationships. Having a
[mechanism for encoding general
topics](https://rs.io/why-category-theory-matters/) empowers you with the
ability to play with structure and assumptions and study the structure and
implications of those arrangements. Caveat emptor; I am not saying composition
_requires_ category theory to be useful! In fact, having too complicated a
system defeats the purpose of having a
[lightweight](https://www.justanotherdot.com/posts/lightweight-is-beautiful.html)
guide.

Architecturally, the common phrase that "systems are the sum of their parts" is
a farce. If systems were some linear combination then removing individual
elements would merely reduce the size of the system, but removal can mean total
system failure, no change whatsoever, and possibly improvement in the system as
a whole!

It is rare to find a mental tool so broadly applicable and yet so uncomplicated
in nature. I'll reiterate strongly here; you don't necessarily need to be
pedantic about the shape of things to reap these benefits. Nor do you need to
understand category theory to its [highest levels of
complexity](http://eugeniacheng.com/wp-content/uploads/2017/02/cheng-lauda-guidebook.pdf)
to piece together solutions. In my mind the _broad_ steps are always the same:

1. **Take, or produce, components**
2. **Scrutinize the components** as you may be able to
    i. break things down further (1)
    ii. see how things connect
    iii. or verify the parts are sound
3. **Experiment with arrangements of components**

I see composition as a framework for experimentation with no added consequence
of increased complexity from the use of the framework itself. Experimentation
allows us to explore new connections. Exploring new connections means finding
solutions to problems in any domain. Discoveries are the bedrock of learning.
Rapid experimentation increases rate of knowledge acquisitions as well as
improved retention of knowledge. This is why composition is a superpower.
