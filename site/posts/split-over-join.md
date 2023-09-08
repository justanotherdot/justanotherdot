---
title: Split over join
author: Ryan James Spencer
date: 2023-09-08T10:19:06.347789785+00:00
tags:
  - software architecture
  - software development 
summary: >-
---

If you're a software developer, you've likely been there; the manic, not to
mention frantic, working through complex input cases and systems interactions,
all to fix or extend some potentially tiny bit of code. This sort of "global
reasoning" is often considered business as usual for software systems at a
certain size, and I'm genuinely surprised there isn't more active discussion
around the tenants of local reasoning. In fact, some time back I was part of a
meeting discussing what we, software developers, each felt was important for
software quality. When it came my turn, I said "local reasoning" only to
confused faces. 

All that local reasoning means is that you can infer your code is correct using
local information, without having to think about every input permutation or
complex ordering of system processes or interfaces all fitting together. I find
this to be a huge part of my toolkit of writing software of reasonable quality,
and I believe in it enough that I started writing some articles and a talk on
it.

Part of local reasoning is being able to do what's called _case analysis_ where
we can implement and reason about individual situations; for example, if we had
a vending machine, we might want to process the steps individually, such that if
we're in a "vendoring" stage, we wouldn't want to think about the "payment
processing" stage sets, or vice versa. 

And while I love local reasoning, this article isn't where I go over those
foundations. Instead, I wanted to talk about the software design pattern of
favoring splitting out cases and over trying to prematurely generalize. I like
to call this "joining" code as generalized cases are better thought as trying to
join together several special cases by finding the common points, but when we
rush into a general implementation, we may not understand enough of that common
surface area, and mistake the one or two cases we have as being representative
of the larger population. 

Why do we generalize in the first place? Many developers are told "don't repeat
yourself" but forget this advice is for refactoring _after_ the code is in
place; identifying the places where we've split things out, where the same thing
is largely being said, but in insiduously different ways that can lead to bugs.
We are also ushered to consider joint abstractions early on, where we can model
multiple things under a canonical notion or concept, but, as with software,
splitting and joining is cheaper than prematurely joining and having to tease
apart ex post facto. Teasing apart is a sharp corner that leads us down
complicated, highly orchestrated migrations, the kind that take months, if not
years, to finalise, if and only if they manage to not get stalled! This is
partly why, although joining is cheap, joining is not always a reversible
decision.

Summarily:

* Splitting is cheaper than teasing apart premature general abstractions
* Joining is cheap and easy once we have enough specific cases to identify the
  common points between many cases. [Some suggest three as a good number for
  when to trigger this
  conversion.](https://www.oreilly.com/library/view/the-rules-of/9781098133108/)
* Local reasoning is awesome, and case analysis is but one part of the
  foundations that which allow us to reason about code _sanely_
  without having to tear our hair out about every imaginable configuration of
  state, inputs, or system interactions. Each specific case we reason about is 
  isolated from other cases. Splitting out is similarly about identifying the
  concepts that can be isolated and, well, isolating them.
