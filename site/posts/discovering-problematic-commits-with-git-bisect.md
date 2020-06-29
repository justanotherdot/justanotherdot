---
title: Discovering Problematic Commits With Git Bisect
author: Ryan James Spencer
date: 2020-06-29T22:50:46.918821022+00:00
tags:
  - rust
  - shell
  - git
summary: >-
  Find out how to search through your git history to find first instances of any
  sort of behavior or characteristics. With git and a hacked-together program
  you can do this quickly and automatically.
---

A problem has happened due to some offending code landing on your main,
production branch. You use `git` and your best bet is to keep rolling back
commits until the system finds itself in a steady state. You come late into this
picture and you're unsure how far back you need to go.

Firstly, you ought to be using something that alleviates the need for cranking
out an entire CI pipeline in order to produce a deploy. I've talked a bit about
this in the past on my [screencasts about setting up a
CI](https://www.youtube.com/playlist?list=PLG8S6YrJRoYI3CIUqvGX4NBSaMWZxe9in).
If you have something like this, rolling back a fair few number of releases is
probably trivial and easy enough to attempt.

However, if you don't have this in place or you really do need to roll through
an entire CI pipeline, then you can still using something like `git bisect` to
find the first offending commit.

`git bisect` runs a binary search across a span of commits. The general
framework for running a `git bisect` is the following:

1. `git bisect start FROM_COMMIT TO_COMMIT`
2. test the commit, determine if it is good or bad, and tell `git` with `git bisect good` or `git bisect bad`

The trick to finding the first offending commit isn't to run the same steps your
CI pipeline would; you should have all those builds available for review and
they will tell you whether or not a build truly succeeded, unless you can't
trust your CI and, in that case, you have other issues on your hand. Crafting
your own test and running it each time in (2) will help guide you in the
decision to making a choice for whether or not the commit is `good` or `bad` in
light of what you are trying to find.

You can alleviate the tedium of (2) by using `git bisect run` and supplying a
script. If the script fails or you ever want to abandon your search midway, you
can always run `git bisect reset` and start over again. There are some tricks to
how you can craft the exit codes from the script you write for `git bisect run`
that really make this process a lot faster. To give a sense of the range of use
for `git bisect` as a general search tool, let's call our test script
`predicate`.

```bash
#!/bin/sh -eux

# NB.
# exiting with 125 tells `git bisect run` to skip this commit.
# exiting with 0 means the commit is `good'.
# exiting with 1 means the commit is `bad'.

cargo build || exit 125 # skip failed builds.
target/debug/program > /tmp/program.out
[ ! diff /tmp/program.out /tmp/program.snapshot ] && exit 1
```

You'll need to place this script somewhere outside of the current git repository
as it will mess up checkouts between commits. Another pitfall that can hurt is
how you structure your git history; if you use merge styled commits, as is the
default for GitHub, then you will probably not care if the commits in between
the range fail but only if the primary, merge commits fail. You can do one of
two things: output the list of all merge commits that match a particular
pattern, e.g., the way GitHub does it, or you could also, if your history is
clean enough, use `git show --no-patch --format="%P" <commit hash>` to determine
if a commit has more than one parent.

In the above example I show testing against a snapshot given some program
output, but really the predicate could be *anything*. Using `git bisect` to
drive things like textual search has better alternatives like the "pickaxe" with
`-S` in `git log`, but if you want to find the first commit where something
happened and it isn't part of the data that git saves, such as program behavior,
then `git bisect` will let you find it far faster. I've also used this in the
past to whip up quick, minimal tests that I can inject after the checkout and
run some test suite against. `git bisect run` takes any binary, too, meaning you
don't *have* to use a shell script like I have in the example above. The real
aim is not to think of the `predicate` script or program as something that has
to be about failures; you can easily use it to discover first instances of any
kind of particular behavior a program may exhibit, as long as it is reproducible
locally.

Granted, a system may be so complex in it's operation that there is no way for
you to locally verify the offending commit. Mitigating or "stopping the
bleeding" is something that needs to happen quick. With that said, `git bisect`
might be a better tool for analysis later, when the pressure is low and you can
better craft a test or predicate to find where the fault first occurred.
