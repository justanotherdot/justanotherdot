---
title: Stdout is Forever
author: Ryan James Spencer
date: 2019-09-04T05:49:00Z
tags:
  - coding
  - debugging
  - pattern
  - go
  - rust
summary: >-
    Debuggers are worth their weight in gold but stdout is the diamond in the rough.
    All the tools we have to pinpoint problems such as REPLs, automatic tracing,
    stacktraces, and even printing to stdout wind up being about two things:
    poking and prodding.
---

Debuggers are worth their weight in gold but stdout is the diamond in the rough.
All the tools we have to pinpoint problems such as REPLs, automatic tracing,
stacktraces, and even printing to stdout wind up being about two things:
**poking** and **prodding**.

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
macro support. This is a source transformation and we can't easily use a
function because our line number will always be the line number of the function,
not the calling site. As such, one option is to write it as some repeated action
in your editor of choice. Imagine you have the following go code in front of
you,

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

## Poking

Sometimes the fastest way to get at a problem is by writing test cases that flex
assertions about the functionality in question. Other times that's not as fast
because the logic might rely on other systems, e.g. integration tests. In those
cases, if you have stacktrace support you might find it useful to panic/throw if
particular assertions aren't met. When that fails you are probably interfacing
with code that is covering up exceptions or panics, say a piece of library code
that takes your code as a callback. You could try stubbing in your own forked
version of the code (scripting languages tend to make this easy) or you could
turn to building your own stacktrace. You iteratively apply print statements in
the following fashion,

```
fn foo() {
  dbg!() # beginning
  <snip>
  dbg!() # middle
  <snip>
  dbg!() # end
}
```

With `dbg!` this is really easy because I don't have to think
about what to pass to the printing function since `dbg!()` simply
emits the filename and line number. In languages that may not have this I've
done `printf(X)` where X = "A", "B", "C", and so on.

With this format in place you can use binary search to figure out where you need
to apply more printing statements on each subsequent run. If, however, your
tests or program take a long while to run it can pay to do upfront work but
perhaps limiting yourself to an arbitrary depth to avoid spending too much time
on tracing that won't pay off.

## Prodding

You can load your [core
dumps](https://jvns.ca/blog/2018/04/28/debugging-a-segfault-on-linux/) into
`gdb` and explore the call stack after a segfault among all sorts of other cool
things that debuggers allow you to do, or you can rig up systems to
automatically provide tracing, such as in [erlang or
elixir](http://erlang.org/doc/man/dbg.html) but hopefully this article has shown
that stdout gives you powerful debugging functionality since we already have
access to executing the program and manipulating its source. We can print
assertions to see if they hold up or mess around with alternative solutions that
may work if the problem is clear. Stdout isn't always the fastest but it's
_lightweight_ which makes it invaluable as it can circumvent a lot of
preparatory work. You can pair this approach into a feedback loop, too, to
reduce duplicated work such as running the tests or program over and over again.
In a future article I'll discuss ways to do this in a range of languages and
environments but at least we've set the tone for some thinking about how to
improve what we spit out while you hack to give you a better understanding of
what's going on under the hood.

#### Acknowledgements

_The name of this post is inspired from [Bodil
Stokke](https://twitter.com/bodil/status/878563460233277440?s=20) when
responding to what "What are everyone's fave debugging tools for languages you
write code in?"_
