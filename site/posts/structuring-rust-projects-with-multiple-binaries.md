---
title: Structuring Rust Projects With Multiple Binaries
author: Ryan James Spencer
date: 2020-01-30T06:31:53.002306383+00:00
tags:
  - scripting
  - automation
  - shell
  - rust
  - linux
---

How do you organize Rust projects with multiple binaries so that the build
output winds up in a common subdirectory? Should you be looking for a solution
other than cargo? Regardless of whether you are using nested crates within a
workspace or simply a mixture of `.rs` files under `src/bin/`, **you absolutely
should be looking for something other than cargo.** What you need is a proper
task runner and the most portable task runner ships with every unix
flavored operating system; `sh`.

People seem to conflate task runners with build tools. Build tools generate
artifacts such as binaries or libraries whereas task runners act as the glue for
teams to share ways to achieve particular chores. Some people use tools like
`make` to do both jobs and the crossed responsibility brings a lot of pain and
maintenance burden. People need to be aware of the many nuances of `make` such
as the fact that tabs for indenting are semantic, rules for tasks need to be
marked as `.PHONY` if there is a target they relate to, and so on. Others end up
using scripting languages such as python or javascript or they may use some
hybrid domain specific language that mixes a bit of programming and
configuration to specify how tasks are run, e.g. `gulp`. You don't need any of
these options.

I'll call this script `bin/build`. We will assume there are several crates in a
workspace for this example and that we use `git` since cargo bootstraps projects
with it by default.

```
#!/bin/sh -eux

ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
mkdir -p dist/bin
for crate in crate1 crate2 crate3; do
  cd "$crate"
  cargo build --release
  cp target/release/crate1-binary "$ROOT/dist/bin/"
  cd "$ROOT"
done
```

This script is dead-simple. It shoots to the root of the project, makes the
directories `dist` and its subdirectory `bin`. We have a list of crates in a
loop we iterate across but we could make this dynamic, as well. Then, in each
crate we create a release build and copy the binary from the project up to the
common subdirectory. Then, we shoot back to the root directory again and repeat.
All we have to do now to do now is make the script executable and call it:

```
$ chmod +x bin/build
$ bin/build
```

You don't need to let scripts grow out of control, either. What's awesome about
keeping scripts, and, more generally, programs small means you can compose
things like this:

```

bin/init
bin/run
```

Where `init` might do some stubbing or setup work and `run` might launch a
service, whatever those tasks may be.

`sh` is POSIX compliant, which means it's portable, so long as you are careful
you do things that are portable. Hope isn't really an option so how is this a
reasonable suggestion? To fix this you can use `shellcheck`[1] which is probably
the only linter I recommend. Every shell script you write should have the
following

```
#!/bin/sh -eux
```

Which says to use `sh` instead of, say, `bash`. `shellcheck` will actually
recommend things intelligently based on which shell you specify. `bash` is not
ideal here because support for particular features differs between versions and
we are aiming to have something pretty much anyone on a team can use at a
moment's notice so long as they are using linux, bsd, darwin, or any other *nix
flavor. This prelude also turns on some common flags.

1. **e** to stop on the first **e**rror
2. **u** to stop if an variable is **u**nset
3. **x** to print tracing output of each e**x**ecuted statement

Some caveats are that (1) doesn't work for pipes, for that you can use `set -o
pipefail`. If you want cleaner output or you don't wan to leak details you can
drop (3). The last convention is to keep scripts in a common `bin` directory at
the root of your project which enhances discoverability of scripts for others.
If people go looking for scripts from project to project its far easier to look
in a place like `bin` without guessing a few options. The reason for why its
called `bin` is that they are executables!

In summary, for shell script success all you need is:

1. A common prelude that uses `sh` and some options set
2. Using `shellcheck` to ensure you're writing sensible and POSIX compliant scripts
3. A common directory for scripts that is the same for all projects

[1]: https://github.com/koalaman/shellcheck "shellcheck"
