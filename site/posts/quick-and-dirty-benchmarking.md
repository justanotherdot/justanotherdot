---
title: Quick and Dirty Benchmarking
author: Ryan James Spencer
date: 2021-01-10T21:19:45.994786144+00:00
tags:
  - rust
  - performance
summary: >-
    Benchmarking with more rigorous means can be great for reliability of the
    numbers, but sometimes you just want a quick and dirty way to track the
    immediate performance results of changes to your program's code and you want
    to do it quickly.
---

In the past I've advocated for the default use of criterion in Rust projects as
the but it isn't always the fastest to run in a development loop for quick
feedback on optimisations. First, let's make our release builds incremental by
specifying the following in your project's `Cargo.toml` file.

```
[profile.release]
incremental = true
```

Next, let's setup a baseline benchmark. I've made a template
[here](https://gist.github.com/justanotherdot/0b0051f96bdeb44c25ad58998910f6a1)
you can dump into a pre-existing module or give it it's own module. We use
`cargo watch` here but it could just as well be any other tool that does the
same job, such as `entr`. This benchmarking suite is bundled only with nightly,
as it comes from the `libtest` crate.

`cargo +nightly watch -x bench`

Remember, the aim here is to get a really fast local feedback loop and not to be
producing rigorous, publishable results. We want to know if changes have general
speedups or slowdowns without having to wait for excessive periods of time.
Granted, the runtime of the the code the benchmark is executing plays a lot into
this, regardless of choice of harness. To minimize, focus on reducing:

* Number of iterations - as a rule of thumb, try to pick somewhere between 5 to
  100 iterations depending on the code under inspection. You want to have some
  confidence of an average between runs, but also not spend too long honing that
  average.

* Size of input - try to pick input sizes that are neither trivial nor massive,
  as you want to ensure the code is getting properly exercised but also reduce
  the time to completing a benchmark in general.

It's worth stressing again that this isn't about building rigorous benchmarks
for comparisons to other projects but to build benchmarks that help you
understand the general trend of whether or not your changes are making
improvements or regressing.


### Alternative Approaches

Sometimes a benchmark like the above may be a bit awkward given the way the code
is laid out, and if you have a binary or can cake the logic into a binary, it
may be alright to record the respective wall times across invocations with a
process. From scratch, let's build out a tester binary for us to run. First,
we'll put in `structopt` for easily switching between changes we want to
experiment against. As `structopt` is a nice veneer over `clap`, there's really
no advantage to either except it might help you get results faster.


```
#[derive(Debug, StructOpt)]
#[structopt(name = "cli", about = "Benchmark harness for X.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(long = "x1")]
    x1: bool,

    #[structopt(long = "x2")]
    x2: bool,
}

<snip>

fn main() {
    let opt = Opt::from_args();
    if opt.x1 {
        example1(&opt.input).expect("[cli] example1 failure");
    } else if opt.x2 {
        example2(&opt.input).expect("[cli] example2 failure");
    } else {
        baseline(&opt.input).expect("[cli] example failure");
    }
}
```

We use an input file above, but we could just as easily take input from
anywhere, either embedded in the program or even from stdin, for example. We are
going to run the program using `hyperfine` which wraps up `criterion` into a
neat bundle and is infinitely useful for comparing wall time averages versus
manually invoking `time` multiple times and performing the aggregations
yourself:

```
; cargo build --release
; hyperfine "cli test.in" "cli --x1 test.in" "cli --x2 test.in"
```

Which gives us some nice output **and a summary** of the fastest variant in
relation to the others:

```
; hyperfine "target/release/cli test.in" "target/release/cli --x1 test.c
sv" "target/release/cli --x2 test.in"
Benchmark #1: target/release/cli test.in
  Time (mean ± σ):       7.7 ms ±   0.5 ms    [User: 6.5 ms, System: 1.2 ms]
  Range (min … max):     7.1 ms …  11.2 ms    385 runs

Benchmark #2: target/release/cli --x1 test.in
  Time (mean ± σ):      16.3 ms ±   0.7 ms    [User: 13.9 ms, System: 3.7 ms]
  Range (min … max):    15.2 ms …  20.7 ms    171 runs

Benchmark #3: target/release/cli --x2 test.in
  Time (mean ± σ):      18.0 ms ±   2.3 ms    [User: 58.9 ms, System: 2.9 ms]
  Range (min … max):    14.7 ms …  29.4 ms    155 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Summary
  'target/release/cli test.in' ran
    2.13 ± 0.17 times faster than 'target/release/cli --x1 test.in'
    2.35 ± 0.34 times faster than 'target/release/cli --x2 test.in'
```

In my messy use of `hyperfine` above, it recommends it notes that it detected
outliers and I might consider running this on a quieter system, which is a good
suggestion and one that shouldn't simply be ignored, especially if the changes
you are performing are producing rather minimal gains or reductions to
performance.

You don't need to rig up an explicit benchmark harness program for the purposes
of this, either. I've had luck using on-hand binaries from previous builds and
newer binaries simply renamed or at different locations on a filesystem to
compare relative performance. If a build from three months ago felt a lot
faster and I can easily do a build of the latest version off my main branch, I
can chuck those into `hyperfine`, too.

One last thing I'll recommend is that it can be handy to rig up profiling tools
in scripts to get numbers across changes. For a variety of tooling, you are
likely to get somewhat unstable numbers across runs on a target. If you want
something rock-solid across runs, you might consider chucking `valgrind` into a
script and comparing output across commands, such as the following:

```
#!/bin/sh -eux

cargo build --release
COMMAND="target/release/cli test.in"

valgrind --tool=cachegrind "$COMMAND" 2>&1 | rg '^=='
valgrind --tool=cachegrind "$COMMAND" --x1 2>&1 | rg '^=='
valgrind --tool=cachegrind "$COMMAND" --x2 2>&1 | rg '^=='
```

The `rg '^=='` and stream redirection will make sure we only see output from
valgrind and not our tools (unless our tools are emitting lines with two or more
equal signs). Cachegrind has a `I ref` field which stands for instruction
references recorded. valgrind runs your program in a sandbox where it can do
checking of various actions, hence numbers should not change depending on noisy
neighbors. If you want something more direct from, say, PMC (performance
monitoring counter), you could plug in `perf stat -ad`, or rig up a flamegraph
to be generated and reloaded into a browser or preview tool each time you make a
change.
