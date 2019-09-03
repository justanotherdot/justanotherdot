---
title: Stdout is Forever
author: Ryan James Spencer
date: 2019-08-30T11:47:00Z
tags:
  - code
  - debugging
  - pattern
  - go
  - rust
---

Debuggers are worth their weight in gold but stdout is the diamond in the rough.
Things like REPLs, automatic tracing, stacktraces, etc. all can help pinpoint
particular problems, but they all wind up being about two things: poking and
prodding.

## A useful macro or two

Rust has the `dbg!` macro and I love it. It's short enough to type and it shows
you what file you are in, line you are on, and how the code looks plus its
value after evaluation. e.g. `dbg!(dbg!(12) == dbg!(1 + 11))` will print

```
[src/main.rs:2] 12 = 12
[src/main.rs:2] 1 + 11 = 12
[src/main.rs:2] dbg!(12) == dbg!(1 + 11) = true
```

Two important quirks with this are,

1. No arguments passed means you just get the file and line number
2. The code still behaves the way it used to except now you have tracing

This gives us just enough information to be lethal. This is possible because
this expands at compile time and can be replicated in other languages that have
macro support. Another option is to write an editor macro (repeated action) or
function. This is a source transformation and we can't use a function because
our line number will always be the line number of the function, not the calling
site. Consider the editor macro with a go function,

```
func AddOne(x Int) Int {
  return x + 1
}
```

and you want to lay down some tracing so you highlight the `x + 1` and hit a
keyboard shortcut which transforms the code into the following,

```
func AddOne(x Int) Int {
  fmt.Printf("[src/main.go:8] x + 1: %#v", x + 1)
  return x + 1
}
```

We could have also used the
[`runtime.Caller`](https://golang.org/pkg/runtime/#Caller) function to get
filename and line number but we can get that spliced in via our editor to avoid
an import. If you are curious what the `runtime.Caller` code looks like here it
is (and, yes, I'm ignoring error handling here since this is intentionally
throwaway code):

```
func AddOne(x Int) Int {
	_, file, line, _ := runtime.Caller(0)
	fmt.Printf("[%v:%v] x + 1: %#v\n", file, line, x+1)
  return x + 1
}
```

The advantage with the above is now we can take our print lines and move them
around at will and we won't have to tweak the filename/lineno combo.

## Laying traps

Sometimes the fastest way to get at a problem is by writing test cases that flex
assertions about the functionality in question. Other times that's not so fast
because the logic might rely on other systems. In those cases, if you have
stacktrace support you might find it useful to panic/throw if particular
assertions aren't met. When that fails you are probably facing some code you
interface with that is calling your code as a callback, say, and any exceptions
it throws it may be caught by the library code. When this happens you could try
stubbing in your own forked version of the code (scripting languages tend to
make this easy) or you could make your own stacktrace. How it works is we
iteratively apply print statements like this

```
fn foo() {
  dbg!()
  <snip>
  dbg!()
  <snip>
  dbg!()
}
```

With `dbg!` this is really easy because I don't have to think
about what to pass to the printing function since `dbg!()` simply
emits the filename and line number. In languages that may not have this I've
done `printf(X)` whee X = "A", "B", "C", and so on.

With this format in place you can use binary search to figure out where you need
to apply more printing statements on each subsequent runs. If, however, your
tests or program take a long while to run it can pay to do upfront work but
perhaps limit yourself to an arbitrary depth to prevent spending too long on
tracing that won't help you.


## Catching the bug

I could then start playing
with values and comparing things. We can print assertions to see if they hold up
or mess around with alternative solutions that may work if the problem is clear.

I had a large JSON blob that was getting deserialized into a hash. I had no
simple functionality for the test to tell me what the difference was between
hashes so I narrowed down on which particular fields didn't match. I realised
that a chunk of code wasn't running that I previously had no tracing in and
eventually chalked it up to the fact that it would try to access a field that
wasn't there, throw an exception, get rescued, and returned early (and
silently). The error wasn't propagated hence the value came back partially
constructed.

When people practice martial arts, they can forget to breath. When people debug
they can forget to print. It doesn't cost you anything except time to lay down
print statements. If you don't have access to assertions and stack traces,
stdout can cut down the time for you. I probably don't need to say that caution
should be taken that print statements that expose secrets or sensitive data
should definitely be stripped before reaching production but most linters can
stop this as part of CI.

## Summary

1. Editor or language macros, functions, etc. that print a value with filename,
   line number, the code in question, and the final, evaluated value are
   invaluable
2. stdout and your editor will usually give you most of what you need to swiftly
   ascertain bugs

Debuggers can be useful for things like [gdb and core
dumps](https://jvns.ca/blog/2018/04/28/debugging-a-segfault-on-linux/) where you
can explore the call stack after a segfault. Most browsers support breakpoint
functionality but you can always be savage and shove items onto the window for
your development session so you can explore with values during execution.

#### Acknowledgements

_The name of this post is inspired from [Bodil
Stokke](https://twitter.com/bodil/status/878563460233277440?s=20) when
responding to what "What are everyone's fave debugging tools for languages you
write code in?"_
