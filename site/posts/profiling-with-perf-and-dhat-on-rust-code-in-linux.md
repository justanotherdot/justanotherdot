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

You've possibly heard of `perf`, a CPU profiler for linux, likely from the term
flamegraph thrown around. `perf` isn't the only tool that can produce
flamegraphs, but it is certainly the first I associated with learning about
flamegraphs (the opposite of a flamegraph is an 'icicle', which is like a
flamegraph but upside down, and usually colored blue).

`perf` works by using Performance Monitoring Counters or PMCs (also known as
HMCs for Hardware Performance Counters). These counters track various metrics in
hardware rather than in software, which can carry its own performance penalty.
`perf` is generally a CPU oriented profiler, but it can track some non-CPU
related metrics. Try giving `perf list` a try in your terminal and have a look.
All the events listed can be explicitly tracked by passing `-e name` to the bulk
of the `perf` subcommands. In this case `name` means any of the names listed in
`perf list`, but it can also be exact counter names if you are working directly
with a specific CPU architecture, but don't worry about this

Here's what we'll cover in this post:

* Why we want instrumentation to gain clarity on where to spend our efforts
* Establishing benchmarks to drive our understanding and comparison of improvements
* A quick intro to `perf`, and where it shines as a profiling tool
* Checking memory usage via `dhat` without having to use `valgrind`

Let's begin with the problem; we have a program that consumes a CSV, and processes
every row to aggregate some result. It's noticeably a bit sluggish, and gets
slower the bigger the input.

In this initial implementation, we deserialize every row into a HashMap of the
headers (`String`s) to a designated value as a series of bytes. `ByteBuf`. We
want to later parse the bytes into something we care about. We could likewise
use `&'a [u8]` but `ByteBuf` gives us the ability to own the resulting bytes,
rather than parsing them immediately into unicode, which they may not be!

The headers are `String`s rather than pre-encoding the struct names as the
headers because we want this code to run on any sort of CSV we hand it,
regardless of the number and name of the headers.

The point of this example is to show that there may be multiple places where
bottlenecks live, and of different types, each of which we'll use various forms
of profiling or assertions to check.

Astute readers might catch the main inefficiency while checking the code.

* starting first with debug, then going to release
* addressing `--release` and related params

    ```
    [profile.release]
    incremental = true
    debug = true
    lto = "fat"
    ```

  as well as compiling with `RUSTFLAGS="-C target-cpu=native"`

* hyperfine between debug and release for example

`cargo install --force hyperfine`

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

* `perf stat -r 100 -ad`
also add pre/post measurement hooks.
```
; perf stat -ad -r 100 target/release/perf-and-dhat-profiling-example test.csv
<snip>

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

`-r` is important here because it will make sure the measurements are run
multiple times. Given that the numbers `perf` will report will be unstable plus
or minus some margin of error (see the fourth column), this will help us
ascertain which is the most trustworthy of metrics (the ones that don't change a
lot over time). The last column shows us how much of the program is built up of
this particular metric. Hence we can see about half of our time we are doing
successful L1 (first level) cache hits, but also half of the time we are missing
the L1 cache, _however_, it's only a tiny proportion of all the L1 cache
accesses.

note that the LLC-load-misses is highlighted in your output, and that it's
nearly 71% of all LL (last-level) cache accesses! The highlighting is just perf
trying to be helpful by pointing out something that seems suss, which is good
for us because it gives us a lead. So we'll do a recording to try to get
symbol-by-symbol updates on what is happening.

* `perf record -e --call-graph dwarf -- program` followed by `perf report`

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

The above is giving us a hint of how the program is running and how it _might_
be inefficient. On its own, missed cache accesses for reading data isn't
necessarily bad, but hardware ought to be able to read up chunks of data at a
time, leveraging a thing called *spatial locality*, meaning "if you want
something, you're likely to want the things around it, too". In this case, we
are seeing us trying to hit data specifically taking a byte record and
deserializing it into some other memory for later usage. Check out all the
malloc and insert calls; we are spending a lot of time building up HashMaps!

* making it more visual with flamegraphs

`cargo install --force inferno`

```
# `perf script` will use the `perf.data` that you generated after `perf record`
perf script | inferno-collapse-perf > stacks.folded
# produce a flamegraph as an interactive SVG to open
cat stacks.folded | inferno-flamegraph > profile.svg
```

This visually shows us that `read_csv` is the largest cost center,
all things considered. This shows us, at a glance, that there is a fair amount
of allocation from the byte record iteration that is getting dropped, possibly
needlessly, and that the deserialize code is also spending a fair bit of time
allocating memory and constructing hashmaps. If we look at the code this makes
sense; we are doing this for every single record!

To get an alternative sense of this we can realise that `perf` is telling us
about CPU metrics, and all this churn on memory results in stalled cycles, or,
in other words, places where the CPU can't advance work because it's stuck
waiting. We can additionally think about this in the context of _instructions
per cycle_ which will tell us how much utilization of the CPU we are getting. If
everything is going at full utilization we would generally see an instructions
per cycle value in `perf stat` of about 3 or 4 (depending on the number of
parallel instructions the pipeline can feed through it's pipeline at a given
time). Stalls are latency, and we want to reduce latency, hence it makes sense
for us to target this area. But we aren't going to get a full picture of what is
happening without verifying the specifics of allocations. We want to make sure
which allocations are rather senseless versus those that are required. In this
case, reading the bytes into memory once makes sense, but doing it more than
that does not. Nor does it make sense for us to be allocating on every read when
it isn't going to be the memory we are intending to keep.

Let's pull out `DHAT` for that next:

* DHAT and heap analysis

    Ok, so we know that there is a deal of memory pressure going on with creating
    HashMap's, but we want to get extra numbers on what particular allocations are
    the most wasteful. When optimising anything, you can ride hunches, but it's best
    to be sure that your hunches are verified and don't stay as blind or educated
    guesses.

    We'll plug in the dhat stuff under a feature flag so we can easily turn it on
    later when we want, and leave it out of the way when we don't. I'll be
    unimaginative and call this feature flag `dhat-on`, thus in our manifest file,

    ```
    [features]
    default = []
    dhat-on = []
    ```

    plus, you'll need to make sure you turn off `lto` that we had on before. We
    want to be able to see the stack traces for the allocations, and
    optimisations such as `lto` will do as much inlining across dependencies as
    possible.

    and also, here's the patch plugging it in,

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
    <snip>
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
    * the hashbrown calls end up producing more total bytes than the actual
      deserialize calls themselves, but
      the byterecord iteration seem to be broken up, likely because the byterecord
      needs to allocate for the individual values or handing back the ByteRecord
      value itself.

* putting in a few simple benchmarks to confirm our updates

Cool, we've confirmed we can clearly fix one thing that is under our control,
the HashMap construction. If we take a step back and think about this program,
we don't need to have the HashMap at all! Reading through the `csv` docs it
looks like we can reuse a single `ByteRecord`. Before we go about making chnages
though, we want some _reproducible_ ways to confirm to ourselves, and especially
to others, that we've made a reasonable optimisation.


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

you likely want to make a top-level `bin` directory and include some scripts
that allow anyone else to reproduce the `perf` and `dhat` checks as well.


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

Which is a ~4x improvement in terms of timing. Let's confirm it in a few ways:


perf stats with stalled cycles:

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

where we can see LLC-load-misses are not even counted.

`perf record` and friends, which show that read_byte_record takes just as long
as our `parse` code.

DHAT which shows our overall memory footprint is drastically reduced,

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
  │   ├─▼ PP 1.1.2/3 (2 children) {
  │   │     Total:     80 bytes (0%, 2,616/s) in 5 blocks (12.5%, 163.5/s), avg size 16 bytes, avg lifetime 8,096.8 µs (26.48% of program duration)
  │   │     At t-gmax: 48 bytes (0%) in 2 blocks (12.5%), avg size 24 bytes
  │   │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │   │     Allocated at {
  │   │       ^1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │   │       ^2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │   │       #3: 0x565479a30754: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:442:19)
  │   │       #4: 0x565479a30754: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:333:28)
  │   │     }
  │   │   }
  │   │   ├── PP 1.1.2.1/2 {
  │   │   │     Total:     56 bytes (0%, 1,831.2/s) in 3 blocks (7.5%, 98.1/s), avg size 18.67 bytes, avg lifetime 6,708.67 µs (21.94% of program duration)
  │   │   │     Max:       32 bytes in 1 blocks, avg size 32 bytes
  │   │   │     At t-gmax: 32 bytes (0%) in 1 blocks (6.25%), avg size 32 bytes
  │   │   │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │   │   │     Allocated at {
  │   │   │       ^1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │   │   │       ^2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │   │   │       ^3: 0x565479a30754: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:442:19)
  │   │   │       ^4: 0x565479a30754: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:333:28)
  │   │   │       #5: 0x565479a38d21: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:337:13)
  │   │   │       #6: 0x565479a38d21: alloc::vec::Vec<T,A>::reserve (mod.rs:803:9)
  │   │   │       #7: 0x565479a38d21: alloc::vec::Vec<T,A>::extend_with (mod.rs:2226:9)
  │   │   │       #8: 0x565479a38d21: alloc::vec::Vec<T,A>::resize (mod.rs:2122:13)
  │   │   │       #9: 0x565479a38d21: csv::byte_record::ByteRecord::expand_fields (byte_record.rs:529:9)
  │   │   │       #10: 0x565479a38d21: csv::reader::Reader<R>::read_byte_record_impl (reader.rs:1661:21)
  │   │   │       #11: 0x565479a38d21: csv::reader::Reader<R>::read_byte_record (reader.rs:1601:18)
  │   │   │       #12: 0x565479a38060: perf_and_dhat_profiling_example::read_csv (main.rs:44:9)
  │   │   │     }
  │   │   │   }
  │   │   └── PP 1.1.2.2/2 {
  │   │         Total:     24 bytes (0%, 784.8/s) in 2 blocks (5%, 65.4/s), avg size 12 bytes, avg lifetime 10,179 µs (33.29% of program duration)
  │   │         Max:       16 bytes in 1 blocks, avg size 16 bytes
  │   │         At t-gmax: 16 bytes (0%) in 1 blocks (6.25%), avg size 16 bytes
  │   │         At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │   │         Allocated at {
  │   │           ^1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │   │           ^2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │   │           ^3: 0x565479a30754: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:442:19)
  │   │           ^4: 0x565479a30754: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:333:28)
  │   │           #5: 0x565479a39fe6: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:337:13)
  │   │           #6: 0x565479a39fe6: alloc::vec::Vec<T,A>::reserve (mod.rs:803:9)
  │   │           #7: 0x565479a39fe6: alloc::vec::Vec<T,A>::extend_with (mod.rs:2226:9)
  │   │           #8: 0x565479a39fe6: alloc::vec::Vec<T,A>::resize (mod.rs:2122:13)
  │   │           #9: 0x565479a39fe6: csv::byte_record::ByteRecord::expand_fields (byte_record.rs:529:9)
  │   │           #10: 0x565479a39fe6: csv::reader::Reader<R>::read_byte_record_impl (reader.rs:1661:21)
  │   │           #11: 0x565479a39fe6: csv::reader::Reader<R>::headers (reader.rs:1332:13)
  │   │           #12: 0x565479a37d70: perf_and_dhat_profiling_example::read_csv (main.rs:40:20)
  │   │         }
  │   │       }
  │   └─▼ PP 1.1.3/3 (2 children) {
  │         Total:     64 bytes (0%, 2,092.8/s) in 2 blocks (5%, 65.4/s), avg size 32 bytes, avg lifetime 20,246 µs (66.2% of program duration)
  │         At t-gmax: 64 bytes (0%) in 2 blocks (12.5%), avg size 32 bytes
  │         At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │         Allocated at {
  │           ^1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │           ^2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │           #3: 0x565479a3069f: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:442:19)
  │           #4: 0x565479a3069f: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:333:28)
  │         }
  │       }
  │       ├── PP 1.1.3.1/2 {
  │       │     Total:     32 bytes (0%, 1,046.4/s) in 1 blocks (2.5%, 32.7/s), avg size 32 bytes, avg lifetime 20,132 µs (65.83% of program duration)
  │       │     Max:       32 bytes in 1 blocks, avg size 32 bytes
  │       │     At t-gmax: 32 bytes (0%) in 1 blocks (6.25%), avg size 32 bytes
  │       │     At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │       │     Allocated at {
  │       │       ^1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │       │       ^2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │       │       ^3: 0x565479a3069f: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:442:19)
  │       │       ^4: 0x565479a3069f: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:333:28)
  │       │       #5: 0x565479a38d50: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:337:13)
  │       │       #6: 0x565479a38d50: alloc::vec::Vec<T,A>::reserve (mod.rs:803:9)
  │       │       #7: 0x565479a38d50: alloc::vec::Vec<T,A>::extend_with (mod.rs:2226:9)
  │       │       #8: 0x565479a38d50: alloc::vec::Vec<T,A>::resize (mod.rs:2122:13)
  │       │       #9: 0x565479a38d50: csv::byte_record::Bounds::expand (byte_record.rs:711:9)
  │       │       #10: 0x565479a38d50: csv::byte_record::ByteRecord::expand_ends (byte_record.rs:535:9)
  │       │       #11: 0x565479a38d50: csv::reader::Reader<R>::read_byte_record_impl (reader.rs:1665:21)
  │       │       #12: 0x565479a38d50: csv::reader::Reader<R>::read_byte_record (reader.rs:1601:18)
  │       │       #13: 0x565479a38060: perf_and_dhat_profiling_example::read_csv (main.rs:44:9)
  │       │     }
  │       │   }
  │       └── PP 1.1.3.2/2 {
  │             Total:     32 bytes (0%, 1,046.4/s) in 1 blocks (2.5%, 32.7/s), avg size 32 bytes, avg lifetime 20,360 µs (66.58% of program duration)
  │             Max:       32 bytes in 1 blocks, avg size 32 bytes
  │             At t-gmax: 32 bytes (0%) in 1 blocks (6.25%), avg size 32 bytes
  │             At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
  │             Allocated at {
  │               ^1: 0x565479a3edc2: <dhat::DhatAlloc as core::alloc::global::GlobalAlloc>::alloc (lib.rs:244:9)
  │               ^2: 0x565479a37aa5: alloc::raw_vec::finish_grow (result.rs:0:23)
  │               ^3: 0x565479a3069f: alloc::raw_vec::RawVec<T,A>::grow_amortized (raw_vec.rs:442:19)
  │               ^4: 0x565479a3069f: alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle (raw_vec.rs:333:28)
  │               #5: 0x565479a3a048: alloc::raw_vec::RawVec<T,A>::reserve (raw_vec.rs:337:13)
  │               #6: 0x565479a3a048: alloc::vec::Vec<T,A>::reserve (mod.rs:803:9)
  │               #7: 0x565479a3a048: alloc::vec::Vec<T,A>::extend_with (mod.rs:2226:9)
  │               #8: 0x565479a3a048: alloc::vec::Vec<T,A>::resize (mod.rs:2122:13)
  │               #9: 0x565479a3a048: csv::byte_record::Bounds::expand (byte_record.rs:711:9)
  │               #10: 0x565479a3a048: csv::byte_record::ByteRecord::expand_ends (byte_record.rs:535:9)
  │               #11: 0x565479a3a048: csv::reader::Reader<R>::read_byte_record_impl (reader.rs:1665:21)
  │               #12: 0x565479a3a048: csv::reader::Reader<R>::headers (reader.rs:1332:13)
  │               #13: 0x565479a37d70: perf_and_dhat_profiling_example::read_csv (main.rs:40:20)
  │             }
<snip>
```

and finally `hyperfine`.

```
Benchmark #1: target/release/perf-and-dhat-profiling-example test.csv
  Time (mean ± σ):       8.8 ms ±   0.5 ms    [User: 7.9 ms, System: 1.0 ms]
  Range (min … max):     8.4 ms …  11.7 ms    293 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' op
tions.
```

which shows an improvement of ~2.4x in terms of latency.

"Trivial allocations" are not so trivial!


## Conclusion

Using `perf` and a memory analysis tool like `dhat` can give you a pretty solid
picture of what is going on in terms of hardware utilization. Obviously it's not
the only profilers at your disposal, but discovering issues backed by numbers is
crucial.
