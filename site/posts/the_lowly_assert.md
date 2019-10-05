---
title: The Lowly Assert
author: Ryan James Spencer
date: 2019-09-19T10:15:00Z
tags: [testing]
---

There is one thing that ties all forms of testing together, **assertions**. The
lowly assert humbly serves whether it's as types, panics, automated tests, or
any other glorious form. Regardless of how it manifests itself, it allows us to
declare things about our systems or program and mechanically check them.

Types of assertions: invariants, preconditions, postconditions. Imagining a
system as a black box. The inputs-outputs model of computation.

A journey through the many forms of the assert

* Replacing tedium/toil with automated testing, rather than manual testing
* assertions library a la assert.h, assert/debug_assert in rust, panics in go
    * recovery vs. unrecoverable bugs.
* assertions with types
* assertions with axiomatic semantics (invariants, pre- and post-conditions)
* the model of a function; inputs and outputs
* the model of a state machine; transitions and nodes

Let's build a testing framework. This isn't going to be your common type of
testing framework. It's going to be a mental framework we will buildup and tour
through the various forms assertions can take.

The first form of assertions is tedious, manually reproductions. You build the
program, run it over and over again observing the output. This approach works
(and some people slavishly adhere to it!) but it's slow, tedious, and incredibly
error prone. The solution here is to make the machine do it!

Systems are guided by principles. Systems come in all sizes. One-line scripts,
programs of all sizes, libraries, ecoystems of disparate parts.
