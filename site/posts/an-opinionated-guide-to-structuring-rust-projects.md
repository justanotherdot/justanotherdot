---
title: An Opinionated Guide To Structuring Rust Projects
author: Ryan James Spencer
date: 2020-06-12T10:49:59.563002270+00:00
tags:
  - rust
summary: >-
---

Cargo's initial project layout is good for bootstrapping a project but as time
goes on there is a growing need to automate chores, wrestle with compile times,
and increase discoverability for maintainers and contributors. I've previously
written about [how I personally orchestrate chores on a Rust
project](https://www.justanotherdot.com/posts/structuring-rust-projects-with-multiple-binaries.html)
but this article will focus on the latter two points.

The highest leverage act you can do with structuring a Rust project is to break
it into independent crates. Chunks of logic that have shown stability over a
window of time are immediate candidates for splitting into a crate, as well as
semantic boundaries between concepts in a given codebase. For example, you may
want to keep sets of types distinct from one crate to the next in the same
project or you might want to enforce that driver logic should be as minimal as
possible with only some glue tying together other core logic libraries. Keeping
things cleanly separated means clustered concepts are easier to locate while we
can aggressively cache crates that don't change much. In terms of naming I
prefer each crate to be in kebab-case and to use the projects name as the prefix
for the sub crate. If our project name was "foo" then each crate would be
prefaced with "foo-*". You can tie all of these crates into a workspace for a
central place to build the entirety of the project. If our project had three
crates in it, we could put a simple `Cargo.toml` with the contents:

```
[workspace]
members = [
  "foo-core",
  "foo-cli",
  "foo-benchmark",
]
```

The general advice for build times is to first use something like `cargo check`
and move to `cargo test` and finally some form of `cargo build` with or without
options. A way to drive down build times is to keep building all the time as you
make changes. I prefer to run multiple loops with various subcommands specified
while I code. You can get around locking issues on the same `.cargo` and
`target` directories by changing these with `CARGO_HOME` and `CARGO_TARGET_DIR`
respectively. This means you can spin up several `watchex`, `cargo-watch`, or
`entr` loops as shell jobs or in separate terminals. To give an example I will
sometimes do `cargo watch`, which does `cargo check` by default, and then will
specify `CARGO_HOME=/tmp CARG_TARGET_DIR=/tmp/target cargo watch -x test` to get
test information as it shows up.

If you're using a CI and can afford it, pushing jobs off to a remote server to
build at the same is yet another extension to this "build all the time"
mentality. When you're happy with your changes you are closer to merging. If you
pair this with something like `sccache` for caching crates across projects, you
can see some nice gains on compile times across several build bots or, if you
run a similar environment to your build bots, you can even share crates from
both local development machine and build bots at the same time. Once `sccache`
is installed, you can export `RUSTC_WRAPPER=$(which sccache)` and check if it's
running across builds with `sccache -s`. I'm unsure what gains you'd see over
`cargo` on a single machine as I've yet to dive into the core of how `sccache`
works under the hood but it's harmless to run for a try.

You have the option to be disciplined and keep all crates on the same version to
make downstream consumption easier, such that if you want to install `foo` and
`foo-bar` you could know that version `2.0.0` is valid for both crates. You can
also setup the installation as a transitive thing from some 'central' crate that
could always install the "right" version of `foo-bar` given some build feature
flag. You may want more flexibility in what version of `foo-bar` you use,
however, and as long as `foo` doesn't also depend on `foo-bar` you shouldn't
have to do any juggling.

I've left some stray tricks for last in the possibility that they may help your
specific case. You can try linking with `lld` or `gold` instead of the standard
linker. You can do this with `RUSTFLAGS="-C link-arg=-fuse-ld=lld"` to use `lld`
at least on linux but I don't always see speedups from this. Setting
`CARGO_BUILD_JOBS` to a number higher than the number of capabilities (cores)
you have on your system is likely to _increase_ compile times, but you could
split your test, check, and build jobs across lower number of cores, such as two
cores for `test` and another two for `check` on a four-core machine. If you are
truly desparate you can try gimmicks like building to a less intense target.
I've written a shell script that will build all possible target `rustc` can
attempt to cross compile and report build times in the event that they succeed.
You can find the gist
[here](https://gist.github.com/justanotherdot/ca1f163754e9a90f6c6b9dfb25a0598f)
and can invoke it with `x-compile-test`. You can also narrow down which targets
you want to use with a regex by specifying `FILTER=x86_64 x-compile-test`.
