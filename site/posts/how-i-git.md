---
title: How I Git
author: Ryan James Spencer
date: 2019-10-07T10:02:46.655278266+00:00
tags: [git, version control, pattern]
---

I thought it might be worth having a look at two things `git` allows I've abused
to remove some warts and toil from my day-to-day flow.

One thing `git` does is alias support. Anything under the `[alias]` key in ones
`$HOME/.gitconfig` is treated as a valid subcommand. This is fine for quick
things, like `r` as `rebase` or `a` for `add`, but you can also alias one-line
scripts, for example, here's a snippet from my `.gitconfig`.

```
  it = "!f() { git fp && git r origin/master; }; f"
```

This demonstrates defining an ad hoc shell function named `f` and calling it
immediately. What's notable about this is that it is _also_ calling a `git`
alias. `git fp` in this case is an alias for `git fetch --prune` and `r` we've
already mentioned is `rebase`, so this, verbosely, is,

```
$ git fetch --prune && git rebase origin/master
```

Another thing `git` let's you do is invoke arbitrary scripts that are named in
the format `git-name`. If the script is on the path, you can call `git name` and
the script, `git-name`, will run. My old process for pushing to a branch I had
authored was a bit verbose,

```
# on first push
$ git push -u origin/master current-branch

# afterwards ...
$ git fetch --prune

# and, after hacking, changes both behind + ahead on branch (rewritten history)
$ git push --force-with-lease

# or, if simply, without any `--force*` flag
$ git push
```

I wrote a script that does all of this, automatically, called
[`git-p`](https://github.com/justanotherdot/gits/blob/master/scripts/git-p),
which lets me call `git p`. It's doesn't work for all corner cases, and could be
extended to, but this fits ninety-nine percent of my use case. This worked well
for awhile, but I needed to build on it. I eventually wrote an alias called `git
up`,

```
  up = "!f() { git it && git p; }; f"
```

The point of `up` is to ensure my changes are always rebased on master before I
push. This is pretty handy but I've recently added yet another alias called
`raise` (also aliased as `pr`),

```
  raise = "!f() { git up 2>&1 | awk '/http/ { print $2 }' | xargs open; }; f"
```

This scrapes out the remote output with the PR creation link that GitHub
provides after a branch is first pushed to the remote repository and funnels it
into `open`. MacOS X has `open` as the default way to open mime-type related
files to respective 'default' applications. On linux, where I use the gnome
windows manager, I have the shell alias,

```
$ which open
open: aliased to xdg-open
```

to try to bridge the gap, which just goes to show aliases and scripts that use
this same format can be really handy for hiding away toil! I don't know if it's
ideal for all CLI tooling but I think this approach is certainly an interesting
approach to let people slip in their own functionality and 'rewire' an interface
to better suit their needs.
