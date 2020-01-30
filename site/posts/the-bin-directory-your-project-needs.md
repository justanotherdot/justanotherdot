---
title: The Bin Directory Your Project Needs
author: Ryan James Spencer
date:
tags:
  - scripting
  - automation
  - shell
  - rust
  - javascript
  - python
  - haskell
  - linux
  - bsd
  - darwin
---

A lot of people pick task runners to automate chores around a codebase. People
tend to pick languages that are ergonomic for their needs as a scripting
language, or they pick tools such as `make`. The problem is that a lot of these
tools aren't _necessarily_ designed to be task runners. They were instead
designed to output binaries.

I will talk about `Rust` here but the technique is language agnostic. I think
it's worth pointing out that task runners and build tools are distinct
creatures. If you need to do something that generates an artifact, that's the
responsiblity of a build tool. If the task involved is glue for getting an
artifact or for doing some activity such as running a local development
workflow, stubbing local databases, or some other routine task, this is the
responsibility of a task runner. Don't conflate the two!

By using shell scripts you support two things; portability and sharing. When I
say "shell scripts" a lot of people think `bash` but what I actually am
suggesting is a lot more pedantic. This should be the prelude of all your shell
scripts:


```
#!/bin/sh -eux
```

You may want to drop the `x` if you don't need logging or it might leak details.
By writing every script as a raw `sh` script you opt in to writing `POSIX`
compliant scripts. This might seem hard but with a lint tool (probably the only
one I will recommend!) you can get a lot of guidance around the nuances of
shell. `shellcheck` is a fantastic way to not only learn how to write better
shell scripts by having it yell at you all the time but also how to write
_portable_ shell scripts.

This might sound like a tick against using shell scripts. Afterall, hope is not
a great strategy! But _every_ language has foot guns. If something doesn't bite
you in a shell script, something else will bite you in python. If something
doesn't bite you in python, another thing might bite you in Haskell. There's
just no winning is there! What I'm advocating is that `sh` is ubiquitous; it
takes no dependencies besides opting into a `POSIX` compliant operating system
and means you can focus more on the guts of what tasks are being orchestrated
rather than on the logic shelling out to them.

Lastly, put shell scripts in the same place for every project. My personal
preference is `bin` at the top level directory (root) but others may choose
something like `scripts`. Whatever is chosen it helps to have it the same across
every project to allow the convention that people go looking in one place rather
than guessing across a number of options.

And that's it, no need to learn a new task runner or shoe-horn your chores into
a build tool. It doesn't matter if you use `npm`, `yarn`, `cargo`, `shake`,
`make`, `cargo`, `python`, or any other equivalent.

1. Use shell scripts and push to have them be `POSIX` compliant so the whole
   team can use them granted they are on some `*nix`
2. Keep shell scripts in a common place to enhance discoverability
3. Always use scripts or programs in the context of other tools if you must
