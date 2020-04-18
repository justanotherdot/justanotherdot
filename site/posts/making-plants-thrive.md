---
title: Making Plants Thrive
author: Ryan James Spencer
date: 2019-10-23T09:38:18.919799940+00:00
tags:
  - infrastructure
  - coding
summary: >-
  It's often lamented that software projects become dead plants in an unloved
  garden: we excitedly keep buying new plants but we don't put in the time to see
  them thrive.
---

It's often lamented that software projects become dead plants in an unloved
garden: we excitedly keep buying new plants but we don't put in the time to see
them thrive.

<blockquote class="twitter-tweet" data-lang="en"><p lang="en" dir="ltr">Anyone else&#39;s GitHub account literally just a graveyard of good intentions? 🙎‍♀️🙋‍♀️</p>&mdash; CaroOooOoOolyn 👻 (@carolstran) <a href="https://twitter.com/carolstran/status/1184938790533681152?ref_src=twsrc%5Etfw">October 17, 2019</a></blockquote>
<script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

The appeal of building something new, playing with some fancy dependency or
tool, trying out some new process; if only we could resist the temptation. But
we shouldn't resist the temptation because this is the sign of healthy
experimentation! **It's far better to experiment in your spare time than to use
your career as an excuse to try out the next shiny thing.**

I'm a huge fan of "laboratories" where questions you have regarding code are
answered by creating code and committing them to a central repository. Making
them multi-language helps by reducing friction for testing things out. A
graveyard of good intentions becomes a collection of prior discoveries.

This doesn't change the fact that we feel guilty that we can't keep the plant
alive. It takes a little discipline, and maybe for some, a bit of prior
knowledge, but it's not too hard to get things into place. In the same way we
reduced friction by making a project multi-language, introducing automation to
reduce toil is the best way for us to combat bitrot; if we can come back to
projects knowing full-well they build, we are much more willing to continue to
"water the plants". Making a project thrive comes in a few major parts:

1. Testing and building the code before it reaches trunk/master
2. Artifacts (library, binary, etc.) are created and published
3. Said artifacts may be deployed to a server to run

Many other automations can be done too: linting, dependency updates, scheduled
builds, et. al. Scheduled builds are cool (and underrated) because they
continuously show projects are building and tests are passing. You now have
extra capacity from all the built artifacts to handle services going down or
security updates having been released. **If you automate away the toil, you can
treat a project less as a chore (by focusing less on the accidental complexity)
and more as a labor of love (by focusing on the inherent complexity of the
problem you are trying to solve).**
