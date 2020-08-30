---
title: Avoid Build Cache Bloat By Sweeping Away Artifacts
author: Ryan James Spencer
date: 2020-08-14T01:08:21.711142397+00:00
tags:
  - rust
summary: >-
  With regular pruning, Rust build artifacts can become a nuisance for CI
  pipelines and developer workstations. With a little bit of shell scripting and
  the aid of a cargo subcommand, we can automate away the toil of cleaning up
  the various projects we hack at.
---

Incremental builds can get huge in Rust and there is no builtin way for them to get pruned over time. This can be particularly important for developers or build bots working with a limited sized disk drive. I have seen some build caches on projects I work on in production into the range of several gigabytes and this is the size *after* compression!

The easiest way to keep these projects clean is to [instrument `cargo-sweep` to run regularly](https://github.com/holmgr/cargo-sweep). You can dump the command into a crontab to run at regular periods. For build bots it can pay to pull down a shared cache or caches and clean them at regular intervals, either as part of preexisting steps or as dedicated steps.

We can tell the crontab to run a script and stuff our logic in the script; this way we don't have to change the crontab if we want to change the behavior. Our script will run `cargo sweep` recursively across all our projects, pruning objects older than thirty days.

```bash
#/bin/sh -eux
if ! cargo sweep >/dev/null 2>&1 ; then
  cargo install cargo-sweep
fi
export PATH="$PATH:$HOME/.cargo/bin"
cd "$HOME"
cargo sweep -r -t 30
```

You can write to a crontab with a one-liner. This  will make our script run every hour every day. You can adjust the period based on the expression given. If you are unfamiliar with crontab expressions, you can play with crontab expressions to produce human readable output, [such as this site](https://crontab.guru/).

```bash
# Assuming you've named the above script `clean-build-artifacts`
# and have put it under /usr/local/bin/
echo '0 */1 * * * /usr/local/bin/clean-build-artifacts' | sudo crontab -u $(whomami) -
```

we could replicate the same behavior that `cargo sweep` does with a shell script, but we would need to replicate a lot of the behavior that it offers. By default `cargo clean` is a sledgehammer that will remove *everything.* This may, in fact, be what you want; perhaps every thirty days you want to build everything fresh again. This can be particularly handy if you are using scheduled CI builds to check for broken windows across your pipelines and you want the sanity that comes with a fresh build.

Which leads me into another use for `cargo sweep` that isn't time based: sweeping toolchains. You could additionally add a cronjob to sweep toolchains every month or so, like this:

```bash
#/bin/sh -eux
if ! cargo sweep >/dev/null 2>&1 ; then
  cargo install cargo-sweep
fi
export PATH="$PATH:$HOME/.cargo/bin"
cd "$HOME"
for project in $(find . -name "Cargo.toml" -type f | grep -vE "(registry|.rustup|.cargo|target)" | xargs dirname); do
  cd "$project"
  current_toolchain=$(rustup toolchain list | grep "override" | awk '{print $1}')
  if [ "$current_toolchain" ]; then
    cargo sweep --toolchains="$current_toolchain"
  else
    rustup toolchain list | grep "default" | awk '{print $1}' | xargs -I{} cargo sweep --toolchains="{}"
  fi
done
```

What does this script do? We have the same preamble as before that installs `cargo-sweep` if it's not present on the system and sets the correct `PATH` environment variable, just in case. We change into our `HOME` directory and then look for all the projects that are only present for our projects. Then, we go into each project and try to see if there is an override for the toolchain, usually caused by the presence of a `rust-toolchain` file, and if there is none, we will clean the default toolchain, instead. When this script is done, all the projects will be cleaned of build artifacts created by unsupported or non-default compilers. Running this on a project of mine of mine that includes a `rust-toolchain` gives me:

```bash
<snip>
[INFO] Cleaned 761.8599634170532 MiB
```

which is a pretty hefty savings all things considered! I can name the above script `clean-build-artifacts-toolchains` and install the script to run monthly by running:

```bash
echo '0 0 1 * * /usr/local/bin/clean-build-artifacts-toolchains' | sudo crontab -u $(whomami) -
```

This script will run on the first of every month. The best part of dumping these sorts of things into scripts is that they can easily be reused by name, rather than lots of copy-pasting. In particular, we can now take these scripts and have a job run it over our build caches like we mentioned before as a step in a pipeline or even as a dedicated monthly schedule build that will clean out old cruft that we are unlikely to need again. Reducing the size of your build caches means the total time taken to pull down the cache and unpack it is less, speeding up overall build times.
