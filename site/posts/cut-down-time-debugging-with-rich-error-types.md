---
title: Favor richer error types when using Result
author: Ryan James Spencer
date: 2021-01-28T19:47:15.613719661+00:00
tags:
  - rust
  - error handling
summary: >-
  Have you ever spent a considerable amount of time tracking down the meaning of
  an error flag or code after a program has crashed? In languages that don't let
  you break down values with pattern matching, booleans and error codes run
  rampant and require extra investigative effort on the part of the programmer.
  Diagnosing problems in programs doesn't have to be hard in Rust given we have
  `Result` to carry along lots of useful information for us.
---

Have you ever spent a considerable amount of time tracking down the meaning of
an error flag or code after a program has crashed? In languages that don't let
you break down values with pattern matching, booleans and error codes run
rampant and require extra investigative effort on the part of the programmer.
Diagnosing problems in programs doesn't have to be hard in Rust given we have
`Result` to carry along lots of useful information for us.

There was a recent video on why [std::process::exit is
'evil'](https://www.youtube.com/watch?v=zQC8T71Y8e4) demonstrating that by
requesting normal or abnormal termination by the operating system through
`std::process::exit` you could fail to do cleanup that the operating system may
fail to do. I would say std::process::exit is quirky rather than evil here
because it is doing exactly what you ask of it. The example is roughly like
this:


```
#[derive(Debug)]
struct Resource(i32);

impl Drop for Resource {
    fn drop(&mut self) {
        println!("goodbye from {}", self.0);
    }
}

enum Error {
    Foo
}

impl Error {
    pub fn exit_code(self) -> i32 {
        match self {
            Error::Foo => 114,
        }
    }
}

fn main() {
    let _x = Resource(0);
    println!("about to terminate the process");
    std::process::exit(1); // "goodbye from 0" never prints.
}
```

In the above code, the destructor for `Resource` never runs because the program
is effectively terminated at the point that `std::process::exit` is called. It's
a blunt tool, and can be used for both zero and non-zero exit codes, which in an
operating system execution context can roughly relate to success or failure
respectively. Exit codes allow both minimal diagnostic information and sometimes
even a way to handle control flow, [as is the case with driving `git bisect`
automatically](https://www.justanotherdot.com/posts/discovering-problematic-commits-with-git-bisect.html).

Some resources definitely need cleanup on program failure and the solution is to
wrap the main logic in another function, preferably one that returns `Result`,
to ensure the resources go out of scope before calling `std::process::exit`. The
solution given in the video has this function returning an exit code (an i32)
for it's error:

```
#[derive(Debug)]
struct Resource(i32);

impl Drop for Resource {
    fn drop(&mut self) {
        println!("goodbye from {}", self.0);
    }
}

fn run() -> Result<(), i32> {
    let _x = Resource(0);
    Err(114)
}

fn main() {
    let _x = Resource(0);
    run().unwrap_or_else(|exit_code| {
        println!("about to terminate the process");
        std::process::exit(exit_code);
    });
}
```

Now we have destructors running on exit, but we have totally lost relevant
*human readable* diagnostic information in the process. **By using richer types
for our errors we gain that information back**:

```
use std::fmt::{Formatter, Display};

#[derive(Debug)]
struct Resource(i32);

impl Drop for Resource {
    fn drop(&mut self) {
        println!("goodbye from {}", self.0);
    }
}

enum Error {
    MissingData
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::MissingData => write!(f, "could not find any data"),
        }
    }
}

impl Error {
    pub fn exit_code(self) -> i32 {
        match self {
            Error::MissingData => 114,
        }
    }
}


fn start() -> Result<(), Error> {
    let _x = Resource(0);
    return Err(Error::MissingData);
}

fn main() {
    let _x = Resource(0);
    start().unwrap_or_else(|e| {
        println!("about to terminate the process");
        eprintln!("[program] {}", e);
        std::process::exit(e.exit_code());
    });
}
```

In the above code we can propagate failures from various parts of our program
and we don't have to lose that information that is useful for diagnostics. By
using a method to map the error to an exit code, we decide to shed that
information at the point when we call `std::process::exit`, allowing us
to print out our error onto stderr or pattern match on it to do emit metrics,
and so on. **If you want to avoid the headache of tracking down bugs in production
systems, keep information as semantically rich as possible for as long as
possible**. If you think of a program like a parser that builds up values from
external input or stimuli, then you want to take advantage of that work for as
long as possible and only discard it at the fringes.
