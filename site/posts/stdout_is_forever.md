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
  - ruby
---

Debuggers are worth their weight in gold but stdout is the diamond in the rough.
Things like REPLs, automatic tracing, stacktraces, etc. all can help pinpoint
particular problems, but they all wind up being about two things: poking and
prodding. A lot of what I do with stdout is the same I'd be doing in a debugger;
print, play, and evaluate code and values.

## A useful macro or two

Rust has the `dbg!` macro and I love it. It's short enough to type and it shows
you what file you are in and line you are on and how the code looks plus its
value after evaluation. e.g. `dbg!(dbg!(12) == dbg!(1 + 11))` will print

```
[src/main.rs:2] 12 = 12
[src/main.rs:2] 1 + 11 = 12
[src/main.rs:2] dbg!(12) == dbg!(1 + 11) = true
```

Two important quirks with this are,

1. No arguments passed means you just get the file and line number
2. The code still behaves the way it used to except now you have tracing

This gives us just enough information to be lethal. The code display is possible
because this happens at compile time but should be possible to replicate in
languages that have macro support. Another option is to write an editor macro or
function. I have been using Vim for a long while but I'm sure you could do the
same in VSCode, Emacs, etc.

Let's say you have some code,

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
is (and, yes, I'm ignore error handling here since this is throwaway code):

```
func AddOne(x Int) Int {
	_, file, line, _ := runtime.Caller(0)
	fmt.Printf("[%v:%v] x + 1: %#v\n", file, line, x+1)
  return x + 1
}
```

## Laying traps

I recently had to extend some ruby for a legacy codebase and had some failing
tests. There was a lot of code `rescue`ing exceptions making it difficult to
pinpoint why. The tests wouldn't give me a backtrace. Was it an error in the
data or something I was doing in code? In hindsight I could have gone and
stripped out the exception handlers but I may not have had that choice. Imagine
it was a library function with our code as the callback. Here's a rough way I
tend to do this as a pattern with stdout.

First, I 'lay traps' in a binary-search fashion,

```
def foo
  puts "A"
  <snip>
  puts "B" # Middle of function
  <snip>
  puts "C"
end
```

It sometimes pays to do a lot of tracing before running tests/code if it takes
far too long to run. With `dbg!` this is really easy because I don't have to
think about what to pass to the printing function since an empty `dbg!()` call
simply emits the filename and line number. This is basically building a manual
stacktrace. By putting a statement at the beginning, middle, and end we can
choose quickly where to put more statements.

## Catching the bug

Eventually with this approach I would hone in on where things were acting awry.
I could then start playing with values and comparing things. We can print
assertions to see if they hold up or mess around with alternative solutions that
may work if the problem is clear.

I had a large JSON blob that was getting deserialized into a hash. I had no
simple functionality for the test to tell me what the difference was between
hashes so I narrowed down on which particular fields didn't match. I realised
that a chunk of code wasn't running that I previously had no tracing in and
eventually chalked it up to the fact that the data was only partially valid
since some extra code wasn't being run as it would try to access a field that
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
