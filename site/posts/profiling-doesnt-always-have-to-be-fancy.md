---
title: Profiling Doesn't Always Have To Be Fancy
author: Ryan James Spencer
date: 2020-08-13T23:30:07.648134677+00:00
tags:
  - rust
  - performance
summary: >-
    Not all profilers offer a frictionless approach to collecting data.
    Sometimes doing the crude approach works wonders.
---

Not all profiling experiences are alike. Some are filled with friction around the tooling. Others are around doubt about whether or not intermediate layers are inflating or shifting numbers in unfair ways. Perhaps you work in a security or data-compliance critical environment and all you want is numbers on what is running in production without having to breach agreements by downloading live data to your work environment. Benchmarks are fantastic for tracking numbers of common or pathological cases over time, but they may still be unrealistic in comparison to the undiscovered cases in prod. When I find that I can't easily wedge in a profile, I get a bit sad and then turn to crude solutions.

Admittedly, performance counters with tools like `perf` can far better track the performance of resource usage rather than wall times. A wall time may include all the time taken during a context switch out to other running processes, for example, but wall times are important because they are the latency humans feel when they use a user interface to a system or tool.

The simplest way to crudely examine timings is to do what benchmark tooling does; record the wall time before and after the code runs. Benchmarks can track different measures of "center" and calculate statistics around outliers and deviations from these markers of center, but ultimately they take two times and output the difference. I want to reduce friction as much as possible, and as such I'll write this as a macro to do the code generation around the value and still return the value. [This is exactly what `dbg` does](https://doc.rust-lang.org/src/std/macros.rs.html#285-305).

```rust
macro_rules! time {
    ($val:expr) => {
        let beg = std::time::Instant::now();
        match $val {
            tmp => {
                let end = std::time::Instant::now();
                let time = (end - beg);
                println!(
                    "[{}:{}] `{}' took {:?}",
                    std::file!(),
                    std::line!(),
                    std::stringify!($val),
                    time
                );
                tmp
            }
        }
    };
}
```

We print out the timings given the file number, line number, expression under timing, and time taken, just like `dbg` does. You'll note that I am using the `Debug` format specifier for the time itself and not converting the time into a canonical format such as nanoseconds. I do this because the formatting is usually clearer this way; if we always returned milliseconds but the code under inspection took only a handful of nanoseconds, I'd have to go back and change the code again, or if the code is taking several seconds, then telling me the nanoseconds will require me to do the math in my head to convert.

Times like this are handy for things such as router endpoints. This same stopwatch style is what is also used for distributed tracing libraries. A library will start a clock on function entry, and on exit calculate the result, create a "span", and add it to a list of spans. Normally spans are identified by some unique trace id and have parent/child identifiers allowing you to shove all the spans up to an aggregator and let them figure out how to stitch the values together, saving you the cost of doing it on the application.

With this macro, we can  go from this code:

```rust
use std::time::Instant;

#[get("/resource")]
fn index() -> Result<Json<Resource>, HttpError> {
    let beg = Instant::now();
    let rsrc = resource().map(|x| Json(x));
    let end = Instant::now();
    // or, dump as json to logger, stdout, etc. for aggregation...
    eprintln!("/resource took {} ms", (beg - end).as_millis());
    rsrc
}
```

to this code:

```rust
#[get("/resource")]
fn index() -> Result<Json<Resource>, HttpError> {
    time!(resource().map(|x| Json(x)))
}
```

If you wanted to you could change the macro to dump structured logs rather than free text, or you could push metrics out to a provider under a name. Regardless of where you aggregate the values for inspection, making this like `dbg` means we are being unobtrusive with our code, allowing us to put in timings and take them out when ready, which is especially handy when you are trying to check the time of a particular chunk of code while developing.

To be complete to the `dbg` implementation, maybe you want to pass several things separated by commas to the `time` macro in the same way you can pass multiple things to `dbg`. Taking a note from the source code of the dbg macro we can see what to add [near the end](https://doc.rust-lang.org/src/std/macros.rs.html#302-304):

```rust
macro_rules! time {
    ($val:expr) => {
        {
            let beg = std::time::Instant::now();
            match $val {
                tmp => {
                    let end = std::time::Instant::now();
                    let time = (end - beg);
                    println!("[{}:{}] `{}' took {:?}", std::file!(), std::line!(), std::stringify!($val), time);
                    tmp
                }
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(time!($val)),+,)
    };
}
```

This change uses the repeat pattern matches of macros to consistently repeat a pattern for as many times as it is mentioned. what this pattern says is "there may be several (thus the + sign) comma separated expressions, followed finally by an optional (thus the ? sign) comma". then, when you use the plus sign in the body, it will repeat the same number of times as the pattern was found. The body says "do the time macro on each captured value for as many times the patterns were captured and put them all in a tuple that fits them, followed by a comma just in case there are is only one value passed and this pattern fires", at least that's how I read that last comma.

You can push this idea further if you want by [writing a benchmark macro](https://gist.github.com/justanotherdot/fe4bf2024d2c13e3eace4f8d6730c3d1) that did the timing across runs, perhaps including mean average and standard devation and maybe even warmups. The point is not to get lost in recreating a benchmark or profiling suite inside of macros but to find ways to unobtrusively provide results in such a way that you can quickly get relative sizes between elements in a system or program. Also, be aware that timings can blow out if you are timing the code that also has timing code in it. Ideally you time independent segments of a function.
