---
title: Untangle Your Errors With Enums
author: Ryan James Spencer
date: 2020-04-22T11:04:16.302310060+00:00
tags:
  - rust
  - error handling
summary: >-
  Do you find it far too easy to reach for panics or shoehorn pre-existing errors
  to fit your needs? Is it unsatisfying that there are no exceptions in Rust and
  challenging to adjust to handling errors with `Result`? Here is a fundamental
  method for modeling data that will help untangle error handling in your
  programs.
---

Do you find it far too easy to reach for panics or shoehorn pre-existing errors
to fit your needs? Is it unsatisfying that there are no exceptions in Rust and
challenging to adjust to handling errors with `Result`? Here is a fundamental
method for modeling data that will help untangle error handling in your
programs.

Programs laden with `unwrap`, `expect`, `assert`, and `panic` are quick to gain
momentum, but this approach is clunky. For those coming from languages where
exceptions are the norm for error handling, it can feel natural to reach for,
but also awkward to use, something as blunt as a panic. Exceptions have handlers
registered, whereas panics do not, which is the primary difference between the
two.

Panics are for critical situations where a program has no other option but to
commit suicide. These vital situations are why capturing a panic in Rust carries
a stigma. Recovering from a panic depends on how the panic unwinds or aborts,
which is not always under our control.

Handle errors in Rust with `Result`. However, for newcomers, it's not apparent
how to best design error types. `Result<A, B>` implies that `B` could be
anything and we don't want to put _anything_ in there; ad hoc matching of
strings or cross-referencing integers for errors is the pits.

The problem with strings, integers, and other unrefined types is that the range
of values you can express with them is _vast_, and when it comes to errors, we
want to categorize them neatly, so the range of things we can express is
_concise_. Unstructured data is hard to check against, parse by a machine, and
find in a codebase. If you do not want to be caught in molasses later in your
project, error handling brevity and classification matter a lot.

Enter the endlessly useful enum. **The beauty of enums is that we can refine
and, therefore, narrow down the range of things we can express**. Enums expose a
handle other coders can rely on, whether they be consumers of your crate or
internal maintainers. Enums optimize for categorization and aggregation, which
makes errors easy to find in code.

To start, create a bare-bones `error` module in your project with a top-level
`Error` enum. I'll put some things in here for demonstration purposes, but I'm
sure you can extrapolate for your own needs:

    use std::fmt::Display;

    mod error {
      pub enum Error {
        IoError(std::io::Error),
        SerdeError(serde_json::Error),
        // ... and so on.
      }

      impl Display {
        // display implementations for each variant.
      }
    }

Once we have this top-level `Error`, keep pushing; Maybe `Error` is too much of a
grab bag. [Keep clarifying your error
types](https://www.justanotherdot.com/posts/make-your-errors-clearer-by-splitting-them-in-half.html).
In the above example, we only expressed external errors but we will inevitably
need to express errors relating to our concerns. I'll extend our top-level
error and even grow some new ones:

    use std::fmt::Display;
    use crate::token::Token;

    mod error {
      pub enum Error {
        pub Vendor(VendorError),
        pub StdError(StdError),
        pub Internal(InternalError),
      }

      pub enum InternalError {
        pub Lex(LexError),
      }

      pub struct LexError {
        pub path: Path,
        pub line: i64,
        pub column: i64,
        pub token: Token,
      }

      pub enum VendorError {
        pub SerdeError(serde_json::Error),
        // ... and so on.
      }

      pub enum StdError {
        pub IoError(std::io::Error),
        // ... and so on.
      }

      impl Display {
        // display implementations for each variant.
      }
    }

From such small beginnings we have grown a relatively comprehensive error type
to use in a variety of situations for our program or library. With all of this
in place there is little reason to turn to a panic. The astute observer noted
that we had `Display` impls laying around. I've structured the output in the
"NASA" style of error reporting, showing a 'stack' of errors. Each layer of the
classification above might have nested descriptions with colons or some other
nested format, for example:

    <snip>
      //  top level, we start with [foo] to help describe things.
      impl Display for Error {
        fn fmt(error: Error, f: Formatter) -> {
          match error {
            Error::Vendor(ve) => fmt.write("vendor: {}", ve),
            Error::StdError(se) => fmt.write("stdlib:  {}", se),
            Error::Internal(ie) => fmt.write("internal: {}", ie),
          }
        }
      }

      impl Display for InternalError {
        fn fmt(error: InternalError, f: Formatter) -> {
          match error {
            Internal::Lex(e) => fmt.write("could not lex source: {}", e),
          }
        }
      }

      impl Display for LexError {
        fn fmt(le: LexError, f: Formatter) -> {
          fmt.write("unrecognized token `{}' in {}:{}:{}", le.token, le.path, le.line, le.column),
        }
      }
    <snip>

If we had a lexing error we could get a nice output like this:

    internal: could not lex source: unrecognized token `asdf' in src/main.rs:3:4

In a language with sum types, or as Rust calls them, enums, there's no excuse
not to use them liberally for data modeling of all kinds. Being meticulous about
how you design errors and using categorization as a guiding heuristic makes
error handling a snap rather than a grind.
