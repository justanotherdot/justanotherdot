---
title: Move Fast and Tuck Code Into the Shadows
author: Ryan James Spencer
date: 2019-08-23T11:51:00Z
tags: [code, software]
summary: >-
  Migrations are a part of life as a dev. They help cut down tech
  debt but they can be risky. It's always less
  risky merging in new and different sets of changes rather than changing code
  in-place. This buys you time. You gain the control over the switch granted
  switching doesn't adversely affect some shared, mutable store of data.
---

Migrations are a part of life as a dev. They help [cut down tech
debt](https://lethain.com/migrations/) but they can be risky. It's always less
risky merging in _new_ and _different_ sets of changes rather than changing code
in-place. This buys you time. _You_ gain the control over the switch granted
switching doesn't adversely affect some shared, mutable store of data.

The [parallel implementation
approach](http://sevangelatos.com/john-carmack-on-parallel-implementations/) is
brilliance incarnate; you keep a functional reference implementation and you
copy it as your 'experimental' version whose sole aim is to eventually replace
(and hence become) the new reference. However, Carmack hits a good point,

> It is often tempting to shortcut this by passing in some kind of option flag
> to existing code, rather than enabling a full parallel implementation. It is
> a grey area, but I have been tending to find the extra path complexity with
> the flag approach often leads to messing up both versions as you work, and
> you usually compromise both implementations to some degree.

I am keen to start experimenting more with the Carmack approach, though. Some
things I've already thought about:

* Having a duplicated directories messes up navigation for a lot of editors and
  is unnecessary bloat
* `git flow` styled approaches and any vcs-based approach will never work
  because it lends into the 'change in place' idea by merging the reference with
  the experiment

Otherwise, there are many ways to define clear boundaries between the reference
and experimental implementation. The most popular solution out of many is
feature flag services but I recommend switching between whole modules rather
than having a lot of logic caked into modules to check flags. Keeping flags
macro and mutually exclusive is important because it means changes are kept
cohesive and conflict free. One thing people tend to forget about is the
original feature-flag: versioning. In the end it doesn't matter which technique
you employ so long as you can 1. **toggle between changes** and 2. **keep
differences clear**.

You can start something similar to this approach by focusing on leaving written
code in a disconnected state but being aggressive about it finding its way to
master. This is healthy because it will change the incumbent attitude of
"production means done" to "production means refinement". I like this approach
and do it as often as I can remember to because it means PRs are kept small
(great for code review) and when I finally do want to rig everything up I can
focus squarely on the plumbing, rather than juggling the correctness of the core
implementation _and_ the coupling to the rest of the system.
