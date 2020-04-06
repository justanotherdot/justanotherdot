---
title: Make Your Errors Clearer By Splitting Them In Half
author: Ryan James Spencer
date: 2020-04-06T10:26:18.979664965+00:00
tags:
  - rust
summary: >-
  Are your errors type devolving into grab bags with varying degrees of
  categorization? Frequently _who_ is at fault is not clear, and that can be one
  of the most ergonomically essential classifications. If a program makes it clear
  that an error is due to a user mistake, an internal complication, or possibly a
  bug, that dramatically eases the usability of the service/library/tool in
  question.
---

Are your errors type devolving into grab bags with varying degrees of
categorization? Frequently _who_ is at fault is not clear, and that can be one
of the most ergonomically essential classifications. If a program makes it clear
that an error is due to a user mistake, an internal complication, or possibly a
bug, that dramatically eases the usability of the service/library/tool in
question.

Making this clear is trivial with a top-level enum. I'm going to pretend we an
interface with two sides in question; the program authors (the providers) and
the users of the program (the consumers). For simplicity, I've called these
`External` and `Internal`, but it could also be `Provider` and `Consumer` or
whatever makes the designation clear for your use case.


```
mod error {
  pub enum ExternalError {
    // e.g. MalformedInput, MissingArguments, and so on.
  }

  pub enum InternalError {
    // e.g. IoError, SerdeError, and so on.
  }

  pub enum Error {
    External(ExternalError),
    Internal(InternalError),
  }
}
```

A sum type lets you easily pattern match and analyze the error itself
without fickle operations or messy validation logic. Sum types (enums) are
fantastic, and you should be looking for ways to leverage them whenever
possible.

A mental model for this I like is thinking every error has an owner. Then you
can write functions that have clear offenders. Then, by signature, you are
assured that a module only deals with internally related failures or externally
associated concerns.

I love this approach because when you get to debugging, you can quickly
ascertain if an error is from mishandling, some operational concern, or perhaps
a bug. The user sees each diagnostic without any ambiguity to who is at fault.
In the same vein that a bug may be breaking an invariant, we might have an
`Invariant` case, which stipulates that an invariant has been breached, without
necessarily having to reach for assertions, or hoping that `debug_assertion`s
will fire in tests. And by all means, if there are more than two offenders, it
is best to define them clearly!
