---
title: How to pick stable, beta, or nightly Rust
author: Ryan James Spencer
date: 2020-04-02T09:04:29.420894589+00:00
tags:
  - rust
summary: >-
  It would seem natural to always pick stable Rust, but how much awesome new
  stuff do beta and nightly have and how unstable are they? It can be confusing
  that such a plethora of feature flags sits in nightly but we don't want to
  sacrifice stability.
---

It would seem natural to always pick stable Rust, but how much awesome new
stuff do beta and nightly have and how unstable are they? It can be confusing
that such a plethora of feature flags sits in nightly but we don't want to
sacrifice stability.

Stability is a guarantee that something won't change. With that said, unstable
features theoretically have no guarantees, but in practice there is generally a
modicum of acceptable change and stability in nightly releases for most
purposes.

Before we begin, I've condensed my thought process into a simple diagram:

<figure>
  <img
    src="/assets/images/pick-rust-toolchain-flowchart.png"
    alt="a flowchart describing how to choose between rust toolchains"
    title="A flowchart for choosing between stable, beta, and nightly toolchains">
  </img>
</figure>

**If you don't need anything specifically from nightly or beta, stable should be
your default option.** Cargo is quite good at mentioning what features you can
possibly turn on to help guide you into nightly. If you want a full guide on all
current and prior unstable features you can check out the [unstable
book](https://doc.rust-lang.org/unstable-book/index.html).

A way to run nightly with a slightly increased sense of stability is to use a
pinned variant. Cargo supports finding `rust-toolchain` files (the [toolchain
file](https://github.com/rust-lang/rustup#the-toolchain-file), as it's called)
at the root of crates which specify the specific version to use when building
the project. You can pin a nightly with a date, so something like

```
nightly-2020-01-01
```

will ensure the nightly released on January 1st, 2020 will be the toolchain
picked.

In my own experience, when I encounter a bug in a pinned nightly I am using, I
can usually bump the pinned version to the latest nightly and go on with my
life. Although nightly Rust is still a moving target but in my experience it is
a remarkably sturdy moving target! Having run Rust for work and personal uses,
I've used pinned nightlies in both cases to great effect.

What's the difference between beta and nightly? `beta` is the first step before
a stable release. Beta is continually improved as nightlies progress and
regressions and features are discovered. The flow goes from nightly, to beta, to
stable, as you can see here in the [Rust Programming Language Book Appendix
G](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html). As stated in
the same appendix:

> Most Rust users do not use beta releases actively, but test against beta in
> their CI system to help Rust discover possible regressions.

Or, another way of putting it; if you use stable, having beta and nightly builds
can help point out failures to be raised with the Rust core team, i.e. beta
should do everything stable does, and more. In the same vein, nightly should do
everything beta does, and more, but with the caveat that unstable APIs are
subject to change. Technically, one could try stable, then go to a pinned beta,
then go to a pinned nightly if they really want to tracking changes to specific
features.
