---
title: Magnifying Glasses for Rust Assembly
author: Ryan James Spencer
date: 2020-05-03T04:40:08.060313636+00:00
tags:
  - rust
  - performance
summary: >-
  Compilers are complex beasts. Our high level source code goes through many
  transformations until it winds up becoming machine code that runs on real or
  virtual hardware. Assembly is the final destination before machine code and it
  doesn't have to be menacing! **Whether you intend to write assembly directly or
  not, knowing how your code translates to assembly can drastically improve your
  ability to analyze programs from the standpoint of performance.**
---

Compilers are complicated beasts. Our high-level source code goes through many
transformations until it winds up becoming machine code that runs on real or
virtual hardware. Assembly is the final destination before machine code, and it
doesn't have to be menacing! **Whether you intend to write assembly directly or
not, knowing how your code translates to assembly can drastically improve your
ability to analyze programs from the standpoint of performance.**

Yes, we need numbers to guide us towards improvements, and yes, that means
having benchmarks. **Arguments over performance that don't include data are
conjecture but understanding assembly gives you a magnifying glass to help guide
you in your optimization adventures.** With some experience, we can learn how to
look at assembly and determine such things as whether or not the assembly
contains efficient instructions, chunks of code are replaced with constant
values, and so on. Benchmarks and analyzing assembly can go hand in hand, but
how do you even get at the assembly in the first place?

If you want to look at Rust's assembly in your project using just `cargo`, there
are two ways. You can call

```
cargo rustc --release -- --emit asm <ARGS>
```

`--release` is optional here. The primary argument that's needed is `--emit
asm`. `ARGS` is the list of arguments you want to pass to `rustc` that might
influence compilation. By default, `rustc` generates AT&T syntax. Still, you can
change to Intel syntax if that's what you prefer by passing `-C
llvm-args=--x86-asm-syntax=intel`, which may not matter to you if this is your
first foray into analyzing assembly, but it can be fun to see as an experiment!

If you want a good starting point for flags, try using:

```
-C target-cpu=native -C opt-level=3
```

These two codegen options instruct the compiler to emit code specifically for
the processor it guesses you are running the compiler on as well as using all
optimizations. You can also pass `opt-level=z` or `opt-level=s` if you want to
optimize for total disk space, instead. **As a note, fewer instructions doesn't
necessarily mean efficient code.** A short set of instructions may end up taking
more cycles than the more verbose alternative.

If, instead, you want to call the standard `cargo build`, you can pass all these
arguments with the `RUSTFLAGS` environment variable. For example:

```
RUSTFLAGS="--emit asm -C opt-level=3 -C target-cpu=native" cargo build --release
```

When the build finishes, the assembly will live in a file with the suffix of`.s`
under `target/debug/deps/CRATE_NAME-HASH.s`
or`target/release/deps/CRATE_NAME-HASH.s`, depending on whether or not you
builtwith the `--release` flag. If I run the above command on a crate with the
name`project` I'll get something like the following:

```
$ find . -name "*.s" -type f./target/release/deps/project-1693e028130a9fa3.s
```

Keep in mind that there may be several of these outputs. If you are confused,
which is the latest, you can try `cargo clean` and building fresh. By default,
the names are going to look pretty weird in the output due to mangling!
**Mangling** ensures that names for identifiers are unique across the process of
compilation. You can try feeding the resulting assembly into
[rustfilt](https://github.com/luser/rustfilt) to get cleaner names:

```
find . -name "*.s" -type f | xargs cat | rustfilt
```

Ok, this is great if you have a project going, but maybe you have some transient
code in the [Rust playground](https://play.rust-lang.org/) and want to know what
the assembly is there. You can emit assembly there, too! If you click on the
ellipses next to the `Run` button, you'll get a menu that has several options.
Select `ASM` for assembly output in another tab. There isn't much control over
compilation options with the Rust playground approach besides picking stable,
beta, or nightly. A more fully-featured web version for picking apart assembly
is [godbolt](https://godbolt.org/), describes itself as a "compiler explorer"
and provides a _lot_ of features to aid you in exploration over the above
bare-bones approaches. Advantage of using godbolt include:

* Viewing highlighted segments of our source code and where they line up to the assembly
* Access to a bevy of compilers from a wide variety of languages, even  selecting which version of Rust you want to use
* Passing arbitrary flags to influence how the generated output is produced
* Diffing changes in assembly between source code assembly
* Looking up the documentation for instructions on the fly

You now know three ways to emit assembly, whether it's on your machine, the Rust
playground, or godbolt! To the uninitiated, this can be overwhelming, but
opening the hood can be liberating and allow us to start exploring the various
instructions and how they all tie together.

To reiterate, you don't always have to look at assembly to guide performance
optimization. Benchmarks are crucial at guiding us towards real-world results.
Try to make it a habit to look at assembly when you're curious about what's
going on under the hood. If you start optimizing, it can be interesting to
compare how assembly changes as you make high-level changes. If things seem to
speed up, try to explore how the assembly itself has changed!
