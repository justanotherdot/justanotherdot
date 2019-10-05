---
title: Actually Using Git Worktrees
author: Ryan James Spencer
date: 2019-10-05T03:48:54.816375278+00:00
tags: [git, version control]
---

Let's say you are expected to do code review and you are also expected to code.
When you do either a certain set of changes is in place. Switching because you
are blocking someone means you _have_ to do a dance with stashing changes,
checking out a branch, perhaps cleaning temporary files, restarting tooling,
etc. Bar changing your codebase, workflow, and job requirements, here's an
approach that uses `git` [`worktrees`](https://git-scm.com/docs/git-worktree) to
ease the cost of these context-switches.

`git` uses `worktree`s to track changes in a repository. `git` gives us the
ability to make more than one `worktree` at a time that are checked out to
potentially different sets of changes. This means we can effectively split up
our codebase into review and development environments:

```
$ git worktree add ../foo-review --checkout master # where `foo` is the name of your project
$ cd ../foo-review
$ git clean -fddx
$ git checkout branch-name
```

You can tuck the change-directory code into a script if there's a slew of other
steps needed to get into a good-known state. If you are using GitHub, here's an
added bonus for checking out pull requests by number rather than by branch name:

```
[alias]
  <snip>
  copr = "!f() { git fetch origin pull/$1/head && git checkout pr/$1; }; f"
  <snip>
```

_N.B. There are alternative ways of [fetching all remote pull requests from
GitHub](https://gist.github.com/piscisaureus/3342247) which might be preferable
to the above alias._

GitHub assigns this special remote tracking branch to your PR, but it's
read-only so if you want to contribute changes you will need to know the name of
the original branch.

With this setup the context-switch dance is reduced. The workflow could be like
this:

1. Someone asks for a review or perhaps you're done and want to get back to work
2. Calling `work` might get you back into your development environment where you
   left off
3. `review branch-name` will go the other direction preparing the pull-request
   for inspection

Git aliases are a neat way to remap the surface area of `git`. I actually think
this is a utility for configuration I don't see more CLI tooling using that
probably could to great effect. In the context of `git` it allows me to get
around some particular ergonomic warts. Also, I don't do this workflow anymore
as I leverage pull-requests largely for communicating changes more than
gate-keeping these days, but I understand not all circumstances are the same.
Worktrees could be used to keep a reference implementation around for quickly
inspecting without having to switch branches, for example. Little things like
this that help reduce toil are worth their weight in platinum so it pays to keep
your eye open to automation opportunities!
