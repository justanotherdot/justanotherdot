---
title: Errors for You and Me
author: Ryan James Spencer
date: 2019-09-08T10:03:00Z
tags: [error handling, code]
---

There is a tendency to clump errors into a single type. ERRORS are treated as a
glob. Something went wrong, and there is some associated complication for it.
Error handling is mixed across languages and changes based on style from
codebase to codebase. I have strong feelings about how error handling should be
done, but instead of belaboring on the particular styles or even my own style of
error handling I'll focus on something that a lot of these styles tend to miss
but could potentially integrate if they were so inclined.

Some service is always provided, whether the interface is on a terminal, over a
network, in a program, there is always an end user and that end user is _not_
the programmer. This is the boundary of provider and consumer. You draw this
boundary every time you utter the word "encapsulation". It's the lines you draw
around nodes in your diagrams and the

Where I work we like to jokingly say that either "we [the provider] are holding
it wrong" or "they [the consumer are holding it wrong".

A lot of devs tend to clump errors into a single categorisation. In reality,
errors fall into two broad categories, cases where the end user is holding it
wrong and cases where the program is holding it wrong. When it's the program's
fault, it's the creators fault, and when it's your end user, it's always
appropriate to be polite and human.

___
This is really a case of a larger principle for systems design. We tend to
daftly say that systems should be decoupled or that they should be encapsulated,
but in reality there are points of connection between our systems and the
components of the systems themselves.
