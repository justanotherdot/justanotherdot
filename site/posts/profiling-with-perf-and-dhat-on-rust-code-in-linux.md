---
title: Profiling with perf and DHAT on Rust code in Linux
author: Ryan James Spencer
date: 2021-07-12T00:18:23.156780952+00:00
tags:
  - rust
  - perf
  - performance
summary: >-
---

You've got a slow Rust program you're sure you can improve, but you have no idea
where to start. If you begin changing things and rerunning the program to test
with something like `time`, it will take you ages. Isn't there anything better?

Profilers and hardware architectures are complex beasts. It can take awhile to
understand how everything connects together and what metrics a profiling tool is
reporting are worth investigating. It doesn't help that programmers tend to act
like profiling is a dark art, left only for the elite.

At its core, profilers inform you about how your program utilizes hardware,
usually paired with some information about _where_ in the program this specific
utilization is occurring. Is the service slow when it talks to the database or
when talking over the network to the client? Is disk access a slog or are
needless cycles being wasted doing duplicated work? We _want_ our programs to
utilize hardware, but we want to do it in a way that is taking full advantage of
of the potential of the hardware in question.

`perf` works by using Performance Monitoring Counters or PMCs (also known as
HMCs for Hardware Performance Counters). These counters track various metrics in
hardware rather than in software, which can carry its own performance penalty.
`perf` is generally a CPU oriented profiler, but it can track some non-CPU
related metrics. Try giving `perf list` a try in your terminal and have a look
at what's available your target machine. All the events listed can be explicitly
tracked by passing `-e NAME` to the bulk of the `perf` subcommands. In this case
`NAME` means any of the names listed in `perf list`, but it can also be exact
counter names if you are working directly with a specific CPU architecture, but
don't worry about this for now.

**Here's what we'll cover in this post:**

* Why we want instrumentation to gain clarity on where to spend our efforts
* Problem description ( you are here )
* Establishing benchmarks to drive our understanding and comparison of improvements
* A quick intro to `perf`, and where it shines as a profiling tool
* Checking memory usage via `dhat` without having to use `valgrind`

If you are the eager sort, [have a look at the code up
front](https://github.com/justanotherdot/perf-and-dhat-profiling-example). The
commits are broken up to allow easily following the below tutorial.

### Problem Description

Let's begin with a problem; we have a program that consumes a CSV, and processes
every row to aggregate some result. It's noticeably a bit sluggish, and gets
slower the bigger the input.

In this initial implementation, we deserialize every row into a `HashMap` of the
headers (`String`s) to a designated value as a series of bytes (`ByteBuf`). We
want to later parse the bytes into something we care about. We want raw bytes to
avoid parsing them immediately into unicode, which they may not be!

The headers are `String`s rather than pre-encoding the struct names as the
headers because we want this code to run on any sort of CSV we hand it,
regardless of the number and name of the headers.

The point of this example is to show that there may be multiple places where
bottlenecks live, and of different types, each of which we'll use various forms
of profiling or assertions to check.

Astute readers might catch the main inefficiency while checking the code, but
the point here is to size up problems with actual data backing the necessary
changes we are looking to take, rather than blindly stabbing in the dark and
seeing if things work or not. Often this blind approach can undo perfectly good
performance gains from other work!

#### Setup and prerequisites

Unfortunately, `perf` is a linux-only tool. We will cover other profilers in
other articles, and if you are using something like `dtrace` on a BSD, you can
similarly flume output into something like `flamegraph`, but this article will
focus on `perf` exclusively. `perf` should come equipped with most linux
distros. If it's not, you may have to do some googling on how to get it for your
current machine.

When running `perf` you'll see some output about changing settings for the
purposes of security. If this is a local machine, you're likely safe to simply
run `echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid`. This setting is
to ensure that other unprivileged processes can't see what they're not supposed
to see.

As with any optimizing adventure, we need to always ensure we are actually
running comparisons on release builds.

```
cargo build --release
```

Artifacts are generated under `target/release/*`.

Next, we'll tweak a few common things in our cargo manifest (`Cargo.toml`):

```
[profile.release]
incremental = true
debug = true
lto = "fat"
```

Sometimes you may want to try `codegen-units = 1`, but it can aversely affect
compile times and may not yield similar gains in performance. Ditty for `lto =
"fat"`, which you can also try `lto = "thin"` alternatively (or just leave it
off). As we'll see with our future steps, it pays to verify! You'll notice that
`debug = true` is also noted; this is going to be necessary when we get to using
`perf record`.

Lastly, it helps to be running with `export RUSTFLAGS="-C target-cpu=native"` to
ensure that the compiler is choosing instructions that are more appropriate for
the actual CPU you are using. `native` isn't a guarantee you will get the exact
machine you are on, and if you do know the specific architecture you are on it's
better to put that in there, instead. You can list supported names with `rustc
--print target-cpus`.

#### Checking wall times

`time` is a trusty tool, but `hyperfine` takes the idea of the tool and augments
it with the statistical finesses and reporting of the `criterion` benchmarking.
Let's install it if you don't have it:

```
cargo install --force hyperfine
```

It's worth showing the difference in performance between `--release` and debug
if you've never done this. Here's a useful experiment, and a template for how to
use the tool `hyperfine` to compare multiple targets of measurement at the same
time:

`hyperfine "target/debug/perf-and-dhat-profiling-example test.csv" "target/release/perf-and-dhat-profiling-example test.csv"`

```
Benchmark #1: target/debug/perf-and-dhat-profiling-example test.csv
  Time (mean ± σ):     443.2 ms ±  10.8 ms    [User: 437.2 ms, System: 1.6 ms]
  Range (min … max):   431.4 ms … 467.4 ms    10 runs

Benchmark #2: target/release/perf-and-dhat-profiling-example test.csv
  Time (mean ± σ):      21.2 ms ±   1.2 ms    [User: 19.9 ms, System: 1.0 ms]
  Range (min … max):    20.1 ms …  31.0 ms    139 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' op
tions.

Summary
  'target/release/perf-and-dhat-profiling-example test.csv' ran
   20.87 ± 1.30 times faster than 'target/debug/perf-and-dhat-profiling-example test.csv'
```

To describe the above output we see:

* the first benchmark's report
* the second benchmark's report
* the summary of which was fastest

In this case running in release mode led to a 20.87 (give or take 1.30) times
improvement over non-release (debug).

#### Describing key metrics with `perf stat`

`perf stat` gives you a summary of common hardware statistics. You almost always
want to run it with `-d` and you can add more `d`s to increase the number of
useful metrics it provides, but I find one is sufficient.

Like any good profiling or benchmarking tool, we are unlikely to observe the
same sample every time, thus variations could be due to any number of factors,
and we can generally describe these as noise. Noise in data is normal, and
unless you are using a profiler like `cachegrind` which runs your code in a
sandbox and records its own metrics, you re likely to see variations of all
sorts. What we want to know is

* are the variations extreme or minimal?
* what is center across all the variations?

Thus it's a good idea to run our program with `-r 100` which will rerun the
program under `perf stat` 100 times.

```
; perf stat -ad -r 100 target/release/perf-and-dhat-profiling-example test.csv
<snip, lots of output from the program itself>

 Performance counter stats for 'system wide' (100 runs):

             84.14 msec cpu-clock                 #    3.838 CPUs utilized            ( +-  0.68%)
               130      context-switches          #    0.002 M/sec                    ( +-  2.67%)
                13      cpu-migrations            #    0.153 K/sec                    ( +-  4.42%)
               301      page-faults               #    0.004 M/sec                    ( +-  5.69%)
        72,827,084      cycles                    #    0.866 GHz                      ( +-  4.81%)  (10.15%)
       139,646,188      instructions              #    1.92  insn per cycle           ( +-  5.22%)  (25.71%)
        36,171,391      branches                  #  429.892 M/sec                    ( +-  4.06%)  (44.76%)
           108,807      branch-misses             #    0.30% of all branches          ( +-  2.81%)  (56.11%)
        48,664,574      L1-dcache-loads           #  578.372 M/sec                    ( +-  1.78%)  (56.60%)
           170,803      L1-dcache-load-misses     #    0.35% of all L1-dcache accesses  ( +-  4.5 1% )  (52.99%)
            67,067      LLC-loads                 #    0.797 M/sec                    ( +-  3.97%)  (36.22%)
            47,521      LLC-load-misses           #   70.86% of all LL-cache accesses  ( +-  4.80 % )  (17.61%)

         0.0219215 +- 0.0000420 seconds time elapsed  ( +-  0.19% )
```

**A handy tip:** _`perf` subcommands often take a `pre` and `post` argument, which
you can use to plug in setup and teardown actions, such as `--pre=cargo build
--release`, which is particularly handy to ensure you are getting results on the
latest results._

`perf stat` will report output with bolded items of note. In my code above you
won't notice this given how I format things but on your terminal you might. The
output can be deciphered as such, per column, in order, we have:

1. The arithmetic mean of the counted values
2. The unit, if known
3. The event name
4. Summary of event in human terms (sometimes referred to as a shadow metric)
5. Deviation across samples

_The sixth column is a bit of a mystery to me. I think it has to do with scaling
the metrics somehow, but if you happen to know definitely, please get in touch!_

It definitely helps to know a little bit about hardware architecture to get a
sense of where problems can arise. You can [read my longer
summary](https://www.justanotherdot.com/posts/what-makes-up-a-cpu.html) of a
basic architecture, but what we care about here, in essence, is about
utilization. For example we want to make sure,

* Is the CPU being utilized as effectively as it can be?
    * `insn per cycle` in the above output next to `instructions` designates
      roughly how many "instructions per cycle" are being executed on average.
      If a CPU can run, say, four instructions at the same time in its pipeline,
      then we'd hopefully see a value of around 4, but anything from one and
      above is good, too.
* Are we switching between threads or are threads switching cores often?
    * `context-switches` and `cpu-migrations` respectively show how often
      how commonly threads are being scheduled or run on cores other than the
      ones they were originally running on.
* Are the CPUs caches being maximized or we having to go to main memory a lot?
    * [Caches](https://www.justanotherdot.com/posts/making-friends-with-caches.html)
      are about reuse and speculation. If you want to get water from a well, it
      would take you less time on average if you could bring a lot of water
      closer to home. If we are frequently accessing data, it's better if we can
      architect a program that doesn't have to communicate excessively with main
      memory, although this may not always be possible, depending on context, of
      course. All the `L1-*` and `LLC-*` metrics describe this, but there is a
      difference between the `dcache` and `icache` which stand for `data cache`
      and `instruction cache` respectively. In the same way we can utilize a
      cache for data, we can do the same for instructions so we don't have to
      decode them repeatedly.
* Are we trashing useful work in the CPU?
    * Speculative branch prediction tries to take a best-guess at which path on
      a branch can be taken. This allows the CPU to do as much work as possible,
      as early as possible, but if the CPU makes the wrong guess, it will have
      to give up everything it's doing at that moment and start over.
      `branch-misses` tells us how many of `branches` have been wrong out of
      these best-guesses.

We see that `L1` (first level) cache hits for data (`dcache`) aren't that great,
and that the last level cache (`LLC`) is missing a _lot_. This implies that we
are having to go out to main memory often, which is a good lead on something to
improve! However, this output is failing to tell us _where_ we should be looking
in our program to see this manifest itself. For this we'll use `perf record` to
associate metrics to symbols in our program.

#### Digging deeper with `perf record` and `perf report`

`perf record` takes samples of events over a given period of time, and lines
them up with the specific instructions (and, therefore, symbols those
instructions belong to) giving us insight into which particular part of our code
is underutilizing or misusing our hardware,

```
perf record -e L1-dcache-loads,LLC-load-misses --call-graph dwarf -- target/release/perf-and-dhat-profiling-example test.csv
```

The above is going to track the two events `L1-dcache-loads` and
`LLC-load-misses` on the command, mentioned after the double hyphen `--`. It is
going to use a `dwarf` debug symbol format, which we enabled output of for
release builds above in our setup under our cargo manifest.

When we run `perf report`, we'll be able to interactively explore hot spots in
our code. We should start with the biggest ones first. By digging into the
report we see that event `L1-dcache-loads` has 98.42% of the event happening
around lots of code that looks to be `deserializing` and allocating memory:

```
Samples: 91  of event 'L1-dcache-loads', Event count (approx.): 44200363
  Children      Self  Command          Shared Object                    Symbol
-   98.42%    41.08%  perf-and-dhat-p  perf-and-dhat-profiling-example  [.] perf_and_dhat_profiling_example::main                                                                                  ◆
   - 57.33% perf_and_dhat_profiling_example::main                                                                                                                                                  ▒
      - perf_and_dhat_profiling_example::go (inlined)                                                                                                                                              ▒
         - 52.78% perf_and_dhat_profiling_example::read_csv (inlined)                                                                                                                              ▒
            - 18.41% <csv::reader::ByteRecordsIter<R> as core::iter::traits::iterator::Iterator>::next (inlined)                                                                                   ▒
               - 11.44% csv::reader::Reader<R>::read_byte_record (inlined)                                                                                                                         ▒
                    csv::reader::Reader<R>::read_byte_record_impl (inlined)                                                                                                                        ▒
                  - csv_core::reader::Reader::read_record                                                                                                                                          ▒
                     - csv_core::reader::Reader::read_record_dfa (inlined)                                                                                                                         ▒
                          csv_core::reader::DfaClasses::scan_and_copy (inlined)                                                                                                                    ▒
               - 6.97% csv::byte_record::ByteRecord::clone_truncated (inlined)                                                                                                                     ▒
                    csv::byte_record::ByteRecord::new (inlined)                                                                                                                                    ▒
                    csv::byte_record::ByteRecord::with_capacity (inlined)                                                                                                                          ▒
                    alloc::boxed::Box<T>::new (inlined)                                                                                                                                            ▒
                    alloc::alloc::exchange_malloc (inlined)                                                                                                                                        ▒
                    <alloc::alloc::Global as core::alloc::Allocator>::allocate (inlined)                                                                                                           ▒
                    alloc::alloc::Global::alloc_impl (inlined)                                                                                                                                     ▒
                    alloc::alloc::alloc (inlined)                                                                                                                                                  ▒
                    __rdl_alloc (inlined)                                                                                                                                                          ▒
                    std::sys::unix::alloc::<impl core::alloc::global::GlobalAlloc for std::alloc::System>::alloc (inlined)                                                                         ▒
                  - __GI___libc_malloc (inlined)                                                                                                                                                   ▒
                       2.77% tcache_get (inlined)                                                                                                                                                  ▒
                       1.43% checked_request2size (inlined)                                                                                                                                        ▒
            - 12.87% csv::byte_record::ByteRecord::deserialize (inlined)                                                                                                                           ▒
                 csv::deserializer::deserialize_byte_record (inlined)                                                                                                                              ▒
                 serde::de::impls::<impl serde::de::Deserialize for std::collections::hash::map::HashMap<K,V,S>>::deserialize (inlined)                                                            ▒
                 <&mut csv::deserializer::DeRecordWrap<T> as serde::de::Deserializer>::deserialize_map (inlined)                                                                                   ▒
               - <serde::de::impls::<impl serde::de::Deserialize for std::collections::hash::map::HashMap<K,V,S>>::deserialize::MapVisitor<K,V,S> as serde::de::Visitor>::visit_map (inlined)      ▒
                  + 7.33% serde::de::MapAccess::next_entry (inlined)                                                                                                                               ▒
                  + 5.55% std::collections::hash::map::HashMap<K,V,S>::insert (inlined)                                                                                                            ▒
            + 5.31% core::ptr::drop_in_place<std::collections::hash::map::IntoIter<alloc::string::String,serde_bytes::bytebuf::ByteBuf>> (inlined)                                                 ▒
            + 4.99% core::ptr::drop_in_place<csv::byte_record::ByteRecord> (inlined)                                                                                                               ▒
            + 4.26% perf_and_dhat_profiling_example::parse (inlined)                                                                                                                               ▒
            + 4.17% core::ptr::drop_in_place<alloc::string::String> (inlined)                                                                                                                      ▒
            + 1.70% core::ptr::drop_in_place<serde_bytes::bytebuf::ByteBuf> (inlined)                                                                                                              ▒
              1.08% <alloc::vec::Vec<T,A> as core::ops::deref::Deref>::deref                                                                                                                       ▒
         + 4.03% perf_and_dhat_profiling_example::histogram (inlined)                                                                                                                              ▒
         + 0.53% perf_and_dhat_profiling_example::read_file (inlined)                                                                                                                              ▒
Tip: Show individual samples with: perf script                                                                                                                                                     ▒
```

Specifically, check out these lines:

```
<snip>
-   98.42%    41.08%  perf-and-dhat-p  perf-and-dhat-profiling-example  [.] perf_and_dhat_profiling_example::main                                                                                  ◆
<snip>
         - 52.78% perf_and_dhat_profiling_example::read_csv (inlined)                                                                                                                              ▒
<snip>
            - 18.41% <csv::reader::ByteRecordsIter<R> as core::iter::traits::iterator::Iterator>::next (inlined)                                                                                   ▒
<snip>
               - 11.44% csv::reader::Reader<R>::read_byte_record (inlined)                                                                                                                         ▒
<snip>
            - 12.87% csv::byte_record::ByteRecord::deserialize (inlined)                                                                                                                           ▒
<snip>
               - <serde::de::impls::<impl serde::de::Deserialize for std::collections::hash::map::HashMap<K,V,S>>::deserialize::MapVisitor<K,V,S> as serde::de::Visitor>::visit_map (inlined)      ▒
<snip>
            + 5.31% core::ptr::drop_in_place<std::collections::hash::map::IntoIter<alloc::string::String,serde_bytes::bytebuf::ByteBuf>> (inlined)                                                 ▒
            + 4.99% core::ptr::drop_in_place<csv::byte_record::ByteRecord> (inlined)                                                                                                               ▒
<snip>
```

Here's how I read this:

* `read_csv` is taking up half the number of these last level cache load misses
* Part (18.41%) of that is reading a byte record; this is expected.
* another part (12.87%) is deserializing each byte record into a HashMap
* a small, but still sizeable, part is dropping the HashMap (5.31%), as well as the
  ByteRecord (4.99%)


Which leads us to ask; can we get rid of all this deserialization and HashMap
construction work? A principle of performance is to be lazy, and the best way to
be lazy is to avoid doing work you don't need to do! Before we get to tweaking
the code, let's analyze this outside of the terminal with flamegraphs.

#### Making it more visual with flamegraphs

`inferno` is a collection of flamegraph related tooling that Jon Gjenset built
to produce different types of flamegraphs. The original flamegraph generation
was a perl script but I find it easier to install and reuse a compiled tool, so
we'll use `inferno`.

```
cargo install --force inferno
```

In order to generate a flamegraph, we have to take the `perf.data` file that is
generated as part of `perf record` and run it through `perf script`. Then we'll
take `inferno-collapse-perf` to turn the stack traces into a "folded" format.

```
perf script | inferno-collapse-perf > stacks.folded
```

Then, we'll spit the folded format into `inferno-flamegraph` and dump the output
into an SVG.

```
cat stacks.folded | inferno-flamegraph > profile.svg
```

You can load this into a browser to interactively zoom into sections.
flamgegraphs are purely shown in terms of scale of magnitude. The x-axis has no
bearing on chronological ordering, but the relative sizes of elements is
comparable.

<figure>
  <img
    src="/assets/images/perf-flamegraph-screenshot-before.jpg"
    alt="A flamegraph of our unoptimised program"
    title="A flamegraph of our unoptimised program">
  </img>
</figure>

This visually shows us that `read_csv` is the largest cost center. At a glance,
there is a fair amount of allocation from the byte record iteration that is
getting dropped, possibly needlessly, and that the deserialize code is also
spending a fair bit of time allocating memory and constructing HashMaps, as we
saw with the terminal. If we look at the code this makes sense; we are doing
this for every single record!

What about specific allocations? `perf` isn't going to be able to tell us
specifics about allocations, and for that we'll turn to Nicholas Nethercoate's
wonderful `dhat` Rust library, built out of the same tooling for `dhat` on
valgrind.

#### Allocation analysis

We'll plug in the `dhat` stuff under a feature flag so we can easily turn it on
later when we want, and leave it out of the way when we don't. I'll be
unimaginative and call this feature flag `dhat-on`, thus in our cargo manifest,

```
[features]
default = []
dhat-on = []
```

Plus, you'll need to make sure you turn off `lto` that we had on before with
`lto = false`. We want to be able to see the stack traces for the allocations,
and optimisations such as `lto` will do as much inlining across dependencies as
possible.

Here's the patch plugging it in to the codebase,

```
diff --git a/src/main.rs b/src/main.rs
index bd43846..b140355 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,4 +1,6 @@
 use csv::Reader;
+#[cfg(feature = "dhat-on")]
+use dhat;
 use serde_bytes::ByteBuf;
 use std::collections::HashMap;
 use std::convert::TryInto;
@@ -7,6 +9,12 @@ use std::fs::File;
 use std::io::{BufReader, Read};
 use std::path::PathBuf;

+#[cfg(feature = "dhat-on")]
+use dhat::{Dhat, DhatAlloc};
+#[cfg(feature = "dhat-on")]
+#[global_allocator]
+static ALLOCATOR: DhatAlloc = DhatAlloc;
+
 type Record = HashMap<String, ByteBuf>;

 const NULL: &'static str = "NULL"; // or whatever.
@@ -77,6 +85,9 @@ fn go(input: &str) -> Result<(), Box<dyn Error>> {
 }

 fn main() {
+    #[cfg(feature = "dhat-on")]
+    let _dhat = Dhat::start_heap_profiling();
+
     go("test.csv").unwrap_or_else(|e| {
         eprintln!("[csv-count] {}", e);
         std::process::exit(1);
```


Then, just run the program as you would. The resulting analysis will dump out a
`dhat-heap.json` that we can take and load into a
[viewer](https://nnethercote.github.io/dh_view/dh_view.html). You can also clone
the valgrind project to get access to the same HTML to view the output.

```
▼ PP 1/1 (7 children) {
    Total:     18,443,933 bytes (100%, 8,095,768.23/s) in 374,530 blocks (100%, 164,395.96/s), avg size 49.25 bytes, avg lifetime 121.47 µs (0.01% of program duration)
    At t-gmax: 2,312,738 bytes (100%) in 26 blocks (100%), avg size 88,951.46 bytes
    At t-end:  1,024 bytes (100%) in 1 blocks (100%), avg size 1,024 bytes
    Allocated at {
      #0: [root]
    }
  }
  ├── PP 1.1/7 {
  │     Total:     973,674 bytes (5.28%, 469,997.01/s) in 112,347 blocks (30%, 54,230.42/s), avg size 8.67 bytes, avg lifetime 13.41 µs (0% of program duration)
  │     Max:       26 bytes in 3 blocks, avg size 8.67 bytes
  │     At t-gmax: 26 bytes (0%) in 3 blocks (11.54%), avg size 8.67 bytes
  │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │     Allocated at {
  │       #1: 0x556ef23a8ab5: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:226:9)
  │       #2: 0x556ef23a8ab5: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:195:45)
  │       #3: 0x556ef23a8ab5: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:136:9)
  │       #4: 0x556ef23a8ab5: alloc::vec::Vec<T,A>::with_capacity_in (mod.rs:580:20)
  │       #5: 0x556ef23a8ab5: <T as alloc::slice::hack::ConvertVec>::to_vec (slice.rs:211:25)
  │       #6: 0x556ef23a8ab5: alloc::slice::hack::to_vec (slice.rs:163:9)
  │       #7: 0x556ef23a8ab5: alloc::slice::<impl [T]>::to_vec_in (slice.rs:476:9)
  │       #8: 0x556ef23a8ab5: alloc::slice::<impl [T]>::to_vec (slice.rs:453:9)
  │       #9: 0x556ef23a8ab5: <&mut csv::deserializer::DeRecordWrap<T> as serde::de::Deserializer>::deserialize_byte_buf::{{closure}} (deserializer.rs:449:50)
  │       #10: 0x556ef23a8ab5: core::result::Result<T,E>::and_then (result.rs:704:22)
  │       #11: 0x556ef23a8ab5: <&mut csv::deserializer::DeRecordWrap<T> as serde::de::Deserializer>::deserialize_byte_buf (deserializer.rs:448:9)
  │       #12: 0x556ef23a87a7: <serde_bytes::bytebuf::ByteBuf as serde::de::Deserialize>::deserialize (bytebuf.rs:251:9)
  │       #13: 0x556ef23a87a7: <core::marker::PhantomData<T> as serde::de::DeserializeSeed>::deserialize (mod.rs:785:9)
  │       #14: 0x556ef23a87a7: <&mut csv::deserializer::DeRecordWrap<T> as serde::de::MapAccess>::next_value_seed (deserializer.rs:654:9)
  │       #15: 0x556ef23a87a7: serde::de::MapAccess::next_entry_seed (mod.rs:1812:34)
  │       #16: 0x556ef23a87a7: serde::de::MapAccess::next_entry (mod.rs:1860:9)
  │       #17: 0x556ef23a87a7: <serde::de::impls::<impl serde::de::Deserialize for std::collections::hash::map::HashMap<K,V,S>>::deserialize::MapVisitor<K,V,S> as serde::de::Visitor>::visit_map (impls.rs:1284:61)
  │       #18: 0x556ef23a87a7: <&mut csv::deserializer::DeRecordWrap<T> as serde::de::Deserializer>::deserialize_map (deserializer.rs:520:13)
  │       #19: 0x556ef23a84f6: serde::de::impls::<impl serde::de::Deserialize for std::collections::hash::map::HashMap<K,V,S>>::deserialize (impls.rs:1293:17)
  │       #20: 0x556ef23a84f6: csv::deserializer::deserialize_byte_record (deserializer.rs:47:5)
  │       #21: 0x556ef23aa85a: csv::byte_record::ByteRecord::deserialize (byte_record.rs:233:9)
  │     }
  │   }
<snip>
  ├─▼ PP 1.3/7 (2 children) {
  │     Total:     7,939,272 bytes (43.05%, 3,832,323.84/s) in 37,450 blocks (10%, 18,077.29/s), avg size 212 bytes, avg lifetime 22.18 µs (0% of program duration)
  │     At t-gmax: 212 bytes (0.01%) in 1 blocks (3.85%), avg size 212 bytes
  │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │     Allocated at {
  │       #1: 0x556ef23a6e3b: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:226:9)
  │       #2: 0x556ef23a6e3b: hashbrown::raw::alloc::inner::do_alloc (alloc.rs:11:9)
  │       #3: 0x556ef23a6e3b: hashbrown::raw::RawTableInner<A>::new_uninitialized (mod.rs:1157:38)
  │       #4: 0x556ef23a6e3b: hashbrown::raw::RawTableInner<A>::fallible_with_capacity (mod.rs:1186:30)
  │     }
  │   }
  │   ├── PP 1.3.1/2 {
  │   │     Total:     7,939,188 bytes (43.04%, 3,832,283.29/s) in 37,449 blocks (10%, 18,076.81/s), avg size 212 bytes, avg lifetime 22.1 µs (0% of program duration)
  │   │     Max:       212 bytes in 1 blocks, avg size 212 bytes
  │   │     At t-gmax: 212 bytes (0.01%) in 1 blocks (3.85%), avg size 212 bytes
  │   │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │   │     Allocated at {
  │   │       ^1: 0x556ef23a6e3b: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:226:9)
  │   │       ^2: 0x556ef23a6e3b: hashbrown::raw::alloc::inner::do_alloc (alloc.rs:11:9)
  │   │       ^3: 0x556ef23a6e3b: hashbrown::raw::RawTableInner<A>::new_uninitialized (mod.rs:1157:38)
  │   │       ^4: 0x556ef23a6e3b: hashbrown::raw::RawTableInner<A>::fallible_with_capacity (mod.rs:1186:30)
  │   │       #5: 0x556ef23a0102: hashbrown::raw::RawTableInner<A>::prepare_resize (mod.rs:1396:29)
  │   │       #6: 0x556ef23a0102: hashbrown::raw::RawTable<T,A>::resize (mod.rs:788:17)
  │   │       #7: 0x556ef23a0102: hashbrown::raw::RawTable<T,A>::reserve_rehash (mod.rs:693:13)
  │   │       #8: 0x556ef23a6ca7: hashbrown::raw::RawTable<T,A>::reserve (mod.rs:646:16)
  │   │       #9: 0x556ef23a6ca7: hashbrown::raw::RawTable<T,A>::insert (mod.rs:827:17)
  │   │       #10: 0x556ef23a7d1e: hashbrown::map::HashMap<K,V,S,A>::insert (map.rs:1266:13)
  │   │       #11: 0x556ef23a87ff: std::collections::hash::map::HashMap<K,V,S>::insert (map.rs:845:9)
  │   │       #12: 0x556ef23a87ff: <serde::de::impls::<impl serde::de::Deserialize for std::collections::hash::map::HashMap<K,V,S>>::deserialize::MapVisitor<K,V,S> as serde::de::Visitor>::visit_map (impls.rs:1285:29)
  │   │       #13: 0x556ef23a87ff: <&mut csv::deserializer::DeRecordWrap<T> as serde::de::Deserializer>::deserialize_map (deserializer.rs:520:13)
  │   │       #14: 0x556ef23a84f6: serde::de::impls::<impl serde::de::Deserialize for std::collections::hash::map::HashMap<K,V,S>>::deserialize (impls.rs:1293:17)
  │   │       #15: 0x556ef23a84f6: csv::deserializer::deserialize_byte_record (deserializer.rs:47:5)
  │   │       #16: 0x556ef23aa85a: csv::byte_record::ByteRecord::deserialize (byte_record.rs:233:9)
  │   │     }
  │   │   }
<snip>
  ├── PP 1.6/7 {
  │     Total:     973,674 bytes (5.28%, 469,997.01/s) in 37,449 blocks (10%, 18,076.81/s), avg size 26 bytes, avg lifetime 39.84 µs (0% of program duration)
  │     Max:       26 bytes in 1 blocks, avg size 26 bytes
  │     At t-gmax: 26 bytes (0%) in 1 blocks (3.85%), avg size 26 bytes
  │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │     Allocated at {
  │       #1: 0x556ef23ad5e4: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:226:9)
  │       #2: 0x556ef23ad5e4: alloc::raw_vec::RawVec<T,A>::allocate_in (raw_vec.rs:195:45)
  │       #3: 0x556ef23ad5e4: alloc::raw_vec::RawVec<T,A>::with_capacity_in (raw_vec.rs:136:9)
  │       #4: 0x556ef23ad5e4: alloc::vec::Vec<T,A>::with_capacity_in (mod.rs:580:20)
  │       #5: 0x556ef23ad5e4: <T as alloc::slice::hack::ConvertVec>::to_vec (slice.rs:211:25)
  │       #6: 0x556ef23ad5e4: alloc::slice::hack::to_vec (slice.rs:163:9)
  │       #7: 0x556ef23ad5e4: alloc::slice::<impl [T]>::to_vec_in (slice.rs:476:9)
  │       #8: 0x556ef23ad5e4: alloc::slice::<impl [T]>::to_vec (slice.rs:453:9)
  │       #9: 0x556ef23ad5e4: csv::byte_record::ByteRecord::clone_truncated (byte_record.rs:508:23)
  │       #10: 0x556ef23ad5e4: <csv::reader::ByteRecordsIter<R> as core::iter::traits::iterator::Iterator>::next (reader.rs:2166:33)
  │       #11: 0x556ef23aa82f: perf_and_dhat_profiling_example::read_csv (main.rs:43:19)
  │     }
  │   }
<snip>
```

Pay attention to a few things here:

* blocks is just a specific call to an allocation, so "100 blocks" means 100
  individual allocation calls, I think.
* the hashbrown (HashMap) calls end up producing more total bytes than the
  actual deserialize calls themselves, but the byterecord iteration seem to be
  broken up, likely because the byterecord needs to allocate for the individual
  values or handing back the ByteRecord value itself.

Particularly, let's look at this output:

```
  │     Total:     973,674 bytes (5.28%, 469,997.01/s) in 112,347 blocks (30%, 54,230.42/s), avg size 8.67 bytes, avg lifetime 13.41 µs (0% of program duration)
  │     Max:       26 bytes in 3 blocks, avg size 8.67 bytes
<snip>
  │     Allocated at {
  │       #1: 0x556ef23a8ab5: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:226:9)
<snip>
  │       #21: 0x556ef23aa85a: csv::byte_record::ByteRecord::deserialize (byte_record.rs:233:9)
  │     }
  │   }
```

Although it's useful to know the total, max, and etc. what this is telling us is
that each deserialize step is leading to plenty of allocations. We had
previously estimated, based on `perf`s output, that it would be good idea to
stop deserializing if we can, but this confirms that it is definitely happening
a lot and is non-trivial in the scheme of things. Before we begin changing code,
we should put in benchmarks to let others running our code confirm results for
themselves (possibly documenting our process like how we've done in this
article, too!).

#### Benchmarks for reproduction

We now know we want to remove our `deserialize` code and that means finding
something we can recycle on every iteration of our loop. Reading through the
`csv` docs it looks like we can reuse a single `ByteRecord`. Having benchmarks
in place will help us verify for ourselves, and others, that the change we've
put in has actually made a net positive gain.

```
#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_read_csv(b: &mut Bencher) {
        let bytes = read_file(&"test.csv".into()).expect("failed to read file");
        b.iter(|| {
            for _ in 1..2 {
                black_box(read_csv(&bytes)).expect("benchmark failure");
            }
        });
    }
}
```

You'll have to chuck `#[feature(test)]` at the top of `main.rs` and run the
benches with nightly. You can pass `+nightly` to do that on the fly without
having to change the current toolchain.

```
; cargo +nightly bench
   Compiling perf-and-dhat-profiling-example v0.1.0 (/home/rjs/repos/perf-and-dhat-profiling-exam
ple)
    Finished bench [optimized] target(s) in 1.54s
     Running unittests (target/release/deps/perf_and_dhat_profiling_example-3cb69d975d903441)

running 1 test
test tests::bench_read_csv ... bench:  18,044,486 ns/iter (+/- 551,993)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 5.47s
```

This shows us an average of 18,044,486 nanoseconds per iteration. 1,000
nanoseconds is a millisecond, and 1000 milliseconds is a second, thus we have 18
seconds per iteration to run against our test case.

Now let's try the single allocation for ByteRecord,

```
diff --git a/src/main.rs b/src/main.rs
index 8d425c1..a68b0ec 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -3,7 +3,6 @@
 use csv::Reader;
 #[cfg(feature = "dhat-on")]
 use dhat;
-use serde_bytes::ByteBuf;
 use std::collections::HashMap;
 use std::convert::TryInto;
 use std::error::Error;
@@ -17,8 +16,6 @@ use dhat::{Dhat, DhatAlloc};
 #[global_allocator]
 static ALLOCATOR: DhatAlloc = DhatAlloc;

-type Record = HashMap<String, ByteBuf>;
-
 const NULL: &'static str = "NULL"; // or whatever.

 #[derive(Clone, Debug, PartialEq, Eq, Hash)]
@@ -40,12 +37,12 @@ pub fn read_file(data: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {

 pub fn read_csv(data: &[u8]) -> Result<Vec<Option<Field>>, Box<dyn Error>> {
     let mut reader = Reader::from_reader(data);
-    let headers = reader.headers().unwrap().clone().into_byte_record();
+    let _headers = reader.headers().unwrap().clone().into_byte_record();
     let mut fields = vec![];
-    for record in reader.byte_records() {
-        let record = record?;
-        let record: Record = record.deserialize(Some(&headers))?;
-        for (_, value) in record.into_iter() {
+    let mut record = csv::ByteRecord::new();
+    while !reader.is_done() {
+        reader.read_byte_record(&mut record).unwrap();
+        for value in record.iter() {
             fields.push(parse(&value));
         }
     }
```

and rerun the benchmark,

```
   Compiling perf-and-dhat-profiling-example v0.1.0 (/home/rjs/repos/perf-and-dhat-profiling-exam
ple)
    Finished bench [optimized] target(s) in 1.22s
     Running unittests (target/release/deps/perf_and_dhat_profiling_example-3cb69d975d903441)

running 1 test
test tests::bench_read_csv ... bench:   4,528,433 ns/iter (+/- 1,328,048)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 4.22s
```

We're down to four seconds! Which is a ~4x improvement in terms of timing. Let's confirm it in a few ways:

`perf stat`

```
 Performance counter stats for 'system wide' (100 runs):

             37.61 msec cpu-clock                 #    3.537 CPUs utilized            ( +-  1.93% )
                94      context-switches          #    0.002 M/sec                    ( +-  3.05% )
                11      cpu-migrations            #    0.300 K/sec                    ( +-  3.59% )
               300      page-faults               #    0.008 M/sec                    ( +-  6.57% )
        34,636,088      cycles                    #    0.921 GHz                      ( +-  6.86% )  (13.69%)
        57,927,487      instructions              #    1.67  insn per cycle           ( +-  6.59% )  (51.81%)
        15,272,168      branches                  #  406.078 M/sec                    ( +-  5.84% )  (88.26%)
            38,666      branch-misses             #    0.25% of all branches          ( +-  4.23% )  (86.61%)
        16,682,061      L1-dcache-loads           #  443.566 M/sec                    ( +-  5.51% )  (48.15%)
           452,163      L1-dcache-load-misses     #    2.71% of all L1-dcache accesses  ( +-  8.41% )  (11.68%)
             5,912      LLC-loads                 #    0.157 M/sec                    ( +- 70.99% )  (0.03%)
     <not counted>      LLC-load-misses                                               (0.00%)

         0.0106321 +- 0.0000668 seconds time elapsed  ( +-  0.63% )

```

where we can see `LLC-load-misses` aren't even counted anymore.

`perf record` and friends, which show that read_byte_record takes just as long
as our `parse` code.

<figure>
  <img
    src="/assets/images/perf-flamegraph-screenshot-after.jpg"
    alt="A flamegraph of our optimised program"
    title="A flamegraph of our optimised program">
  </img>
</figure>

`dhat` instrumentation, which shows our overall memory footprint is drastically reduced,

```
<snip>
PP 1/1 (18 children) {
    Total:     3,539,231 bytes (100%, 115,733,004.15/s) in 40 blocks (100%, 1,308/s), avg size 88,480.78 bytes, avg lifetime 8,511.5 µs (27.83% of program duration)
    At t-gmax: 2,312,340 bytes (100%) in 16 blocks (100%), avg size 144,521.25 bytes
    At t-end:  1,024 bytes (100%) in 1 blocks (100%), avg size 1,024 bytes
    Allocated at {
      #0: [root]
    }
  }
  ├─▼ PP 1.1/18 (3 children) {
  │     Total:     262,280 bytes (7.41%, 8,576,567.15/s) in 22 blocks (55%, 719.4/s), avg size 11,921.82 bytes, avg lifetime 4,408.95 µs (14.42% of program duration)
  │     At t-gmax: 131,184 bytes (5.67%) in 5 blocks (31.25%), avg size 26,236.8 bytes
  │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │     Allocated at {
  │       #1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │       #2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │     }
  │   }
  │   ├── PP 1.1.1/3 {
  │   │     Total:     262,136 bytes (7.41%, 8,571,858.34/s) in 15 blocks (37.5%, 490.5/s), avg size 17,475.73 bytes, avg lifetime 1,068.07 µs (3.49% of program duration)
  │   │     Max:       131,072 bytes in 1 blocks, avg size 131,072 bytes
  │   │     At t-gmax: 131,072 bytes (5.67%) in 1 blocks (6.25%), avg size 131,072 bytes
  │   │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │   │     Allocated at {
  │   │       ^1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │   │       ^2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │   │       #3: 0x565479a307f4: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:442:19)
  │   │       #4: 0x565479a307f4: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:333:28)
  │   │       #5: 0x565479a3819f: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:337:13)
  │   │     }
  │   │   }
<snip>
```

Which shows us that we've gone from 18,443,933 bytes to 3,539,231 bytes and we
are mostly spending time largely building `RawVec`s now.

Finally `hyperfine`.

```
; hyperfine "target/release/perf-and-dhat-profiling-example test.csv" "target/release/perf-and-dhat-profiling-example-unoptimized test.csv"
Benchmark #1: target/release/perf-and-dhat-profiling-example test.csv
  Time (mean ± σ):       8.9 ms ±   0.5 ms    [User: 8.0 ms, System: 0.9 ms]
  Range (min … max):     8.4 ms …  11.8 ms    291 runs

Benchmark #2: target/release/perf-and-dhat-profiling-example-unoptimized test.csv
  Time (mean ± σ):      23.7 ms ±   1.2 ms    [User: 22.5 ms, System: 1.1 ms]
  Range (min … max):    22.6 ms …  31.8 ms    121 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Summary
  'target/release/perf-and-dhat-profiling-example test.csv' ran
    2.66 ± 0.19 times faster than 'target/release/perf-and-dhat-profiling-example-unoptimized test.csv'
```

Lesson: "trivial allocations" are not so trivial!

## Conclusion

Using `perf` and a memory analysis tool like `dhat` can give you a pretty solid
picture of what is going on in terms of hardware utilization. Obviously it's not
the only profilers at your disposal, and we'll likely tackle some other articles
doing the same above with different tooling, but discovering issues backed by
numbers is fundamentally crucial in demonstrating to yourself and others that a
change is worth accepting.
