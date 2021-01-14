---
title: Increase The Rate Of Oxidation: Getting More Rust Into Your Life
author: Ryan James Spencer
date: 2021-01-14T21:45:31.267576425+00:00
tags:
  - rust
summary: >-
    With the growing number of resources to learn Rust nowadays, where does one even
    begin? How does one get into writing idiomatic Rust code as quickly as possible?
    If passively consuming content isn't for you, here's some tips and ideas to
    kickstart your carcinisation.
---

With the growing number of resources to learn Rust nowadays, where does one even
begin? How does one get into writing idiomatic Rust code as quickly as possible?
If passively consuming content isn't for you, here's some tips and ideas to
kickstart your [carcinisation](https://en.wikipedia.org/wiki/Carcinisation).

### Tips For Breaking Down Your First Bit Of Metal

You're excited to learn Rust and you want to build something; awesome!

Here's the rub, though: projects are great for learning, but have you ever
stopped to consider the scale and dimensions of what you're about to undertake?
Pick a project that's too gargantuan and you wind up stopping dead in your
tracks early on. Pick a project that's too trivial and you feel unsatisfied from
what you've learned. Both of these finishes can put a knot in your quest to learn
the language.

This isn't to say that you can't have failed projects that have taught you
lessons. The bigger problem stems from failed projects that affect your
motivation to continue learning.

Here's a number of tips and approaches I've picked up over the years that have
worked for me coming to new languages or technologies:

* **Don't fuss over quirks** - Quirks of a language and its tooling are
  important, and you'll get to them, but it's wise to think about knowledge like
  [a jar that's to be filled with
  rocks](https://www.developgoodhabits.com/rock-pebbles-sand/); focus on locking
  in the big concepts first, then fill out your knowledge with the next smaller
  granularity of detail. The jar-filled-with-rocks analogy is a bit like Zeno's
  paradox, however, as [there is always another finer grained detail you can put
  into the jar](https://www.justanotherdot.com/posts/an-infinite-barrage-of-mountains-to-climb.html).

* **Be specific before you generalize** - try to write concrete implementations
  before you tackle generics. This is true even if you're _not_ starting anew
  with Rust! It is drastically simpler to generalize a tableau of concrete
  things than it is to first start with a generalization. Keep in mind this
  isn't a rule, and sometimes starting with the generalization will save you a
  lot of headaches.

* **You can borrow/reference things later** - writing correct programs that clone all
  over the place should be your first order of business. In the same spirit of
  "make it work", the beauty of having `clone` be explicit is you will be able
  to tell where copies are happening, and hence be able to switch over to
  borrowing **in time**. Until then, focus first on getting the foundations
  laid down, namely with ownership.

* **Foundations are built from smaller insights** - if there is anything you
  take away from this article, it should be this: large-scale projects are
  educational, but the learnings from them can be sparse and unclear. It's far
  better to start with smaller understandings and build up from there to bigger
  projects. If you do want to do a bigger project, focus on each module as it's
  own encapsulated thing that can be tested and run in isolation and later
  rigged up to the main system.

* **Hold an experimenters mindset** - as you cover the surface area of the
  language, you'll start to ask questions. Those questions are best answered
  _and captured_ in minimal code examples or in notes. I find code examples
  better here because I can continually run the example against different
  versions of a language as time goes on. Using gists or gist-like services is a
  great way to store ad hoc solutions to questions you have. In fact, the [Rust
  playground](https://play.rust-lang.org/) will save its permalinks to gist
  files, but it can help to have everything all in a single place for yourself.
  GitHub gists also support multifile gists meaning you can tuck in a
  `Cargo.toml` manifest or other parts that may be relevant to the code. This is
  why I also suggest sometimes having a `lab` or `playground` repository for
  various snippets, too, but `cargo new` already bootstraps a git repo that it
  is easy to push up various experiments. If you are using GitHub, you can
  automate the `gh` cli to also create a new repository, too, allowing you to
  automate the whole process without having to go to a browser.

* **Read rust code from major projects** - If you want to start writing
  idiomatic code, try observing the common patterns you see in major projects.
  Idioms are formed as a communal reaction. Understanding global and local
  idioms alike can help you discover which work for you or your team. Resources
  like [lib.rs](https://lib.rs/), the Rust standard library, and other major
  projects on various source code hosting platforms can let you dip into
  interesting portions of code without being overwhelmed by the whole of the
  project.

* **Translate code from another language to explore the shape of things** - For
  those coming to Rust from another language, it can help to translate from your
  language of choice _with the caveat that Rust is its own language and the
  mapping is not going to be one-to-one_. Translations are helpful as lessons
  around what is similarly possible, different, and non-existent between your
  source and target languages. As an example, I learned a lot about closures and
  function types in Rust when I [tried to port a property-based testing library
  from Haskell to Rust](https://github.com/justanotherdot/rust-hedgehog), but I
  now know I would not build it the same way that the Haskell code was written.
  There is a similar desire to do the same for C, C++, or other C-derived
  languages since they are so similar to Rust, but again, Rust is enough of it's
  own beast that you are likely better off deeply understanding the concepts and
  having a mental model of what you are about to build and going off that rather
  than directly porting code. Also, side note: if you have a desire to cleanup
  the code you are about to translate, keep in mind that the sooner you start
  writing _Rust_ code. This is also true if the intent of the conversion isn't
  about education and more about bringing over code into Rust, as you will be
  able to refine the code once you have the rough semblance of logic laid down.

If there is a single most important point above, it is the point about the
"experimentalist" or "explorer's" mindset. Treat what you are about to go into
like an alien landscape; if you make small, insular understandings over time,
they will chalk up to forming a really intricate patchwork of knowledge. **Rust
is a hard language, but the notion is that by moving slower in localised spots
we move faster overall**. Rust seeks to help you discover more problems at the
time you are writing code than when you have released it into the wild. With
time, any programming language gets faster to write code in overall: you know
the idioms, the options for solving various problems, common libraries, and what
is caked into the language from the start.
