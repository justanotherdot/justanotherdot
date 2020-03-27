---
title: Catching panics in dependencies
author: Ryan James Spencer
date: 2020-03-27T01:55:26.675692931+00:00
tags:
  - rust
summary: >-
  Having crates panic on you feels random because the specific conditions that
  trigger the panic may not seem clear. Having external crates bring down your
  program is a pain, but there is currently no static analysis tool to help us
  easily find panics in external crates. You can get pretty close with fuzzing,
  though!
---

Having crates panic on you feels random because the specific conditions that
trigger the panic may not seem clear. Having external crates bring down your
program is a pain, but there is currently no static analysis tool to help us
easily find panics in external crates. You can get pretty close with fuzzing,
though!

The format for fuzzing is generally:

1. Get some random bytes
2. Shape them into the right shape needed for our interface
3. Run the interface with the random data and see if it blows up

Some fuzzing libraries take the input that leads to a crash and continually mutates it
to find other cases where it might crash as well. I'm going to use `cargo fuzz`
to reproduce finding a panic in an external dependency. [Here's a
case](https://github.com/rust-num/num/issues/268) taken from the `rust-fuzz`
organisations [trophy case](https://github.com/rust-fuzz/trophy-case). I'll use
`num` v0.1.31 which panics when parsing `BigInt`s as per the linked issue. I'll
add it to the `Cargo.toml` of our project:

```
[dependencies]
num = "=0.1.31"
```

Then, I'll install `cargo fuzz`.

```
cargo install cargo-fuzz
```

then in our project, we can initialize cargo fuzz.

```
cargo fuzz init
```

Which gives me a single fuzz target which I can rename if I want. I'll rename it
to `parse.rs` so the name is `parse` when listed but you can call it anything
that fits. To do this I will change the `fuzz` subdirectories `Cargo.toml`. A
`[[bin]]` key is in there that designates what targets are available. Since
we're moving `fuzz_target_1` to `parse`, we have to do this in the TOML file
since the `cargo fuzz` subcommand doesn't have this ability, but we _can_ use
the `cargo fuzz add` subcommand to add extra targets with custom names in the
future.

The new key should look like this

```
[[bin]]
name = "parse"
path = "fuzz_targets/parse.rs"
```

with the rest of the `fuzz/Cargo.toml` left as-is.

Then we can write a simple case for our function. This can be a little tricky
because the fuzzer will hand us raw bytes and it's up to us to shape them into
the right format, whether that be a struct, i64, f32, and so on. For this case
I'll make a string from the random bytes and feed it into our project's `parse`
function:

```
#![no_main]
use our_project::parse;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse(&s);
    }
});
```

the `parse` function might look something like this (lifted from the issue):

```
use num::Num;

pub fn parse(str: &str) {
    num::BigUint::from_str_radix(str, 10);
}
```

and then I'll run the fuzzer for this target:

```
cargo fuzz run parse
```

This finds offending strings similar to the trophy case example quite quickly:


```
<snip>
────────────────────────────────────────────────────────────────────────────────

Failing input:

        fuzz/artifacts/parse/crash-00a78c613a00b21ea723de12e16f32a0385d9bdc

Output of `std::fmt::Debug`:

        [48, 43, 49]

Reproduce with:

        cargo fuzz run parse fuzz/artifacts/parse/crash-00a78c613a00b21ea723de12e16f32a0385d9bdc

Minimize test case with:

        cargo fuzz tmin parse fuzz/artifacts/parse/crash-00a78c613a00b21ea723de12e16f32a0385d9bdc

────────────────────────────────────────────────────────────────────────────────

Error: Fuzz target exited with exit code: 77
```

Hang on, that debug output doesn't look like the offending case mentioned in the
issue we linked above from the trophy case, does it? Again, this is because it's
the raw bytes. A handy trick I use when running regressions is to utilize the
stored failing input. In this case it's stored at
`fuzz/artifacts/parse/crash-00a78c613a00b21ea723de12e16f32a0385d9bdc` per the
output above. We can write a regression that uses this directly with the macro
`include_bytes!`:

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzz_regression_01() {
        let data = include_bytes!(
            "../fuzz/artifacts/parse/crash-00a78c613a00b21ea723de12e16f32a0385d9bdc"
        );
        let s = std::str::from_utf8(data).expect("should be able to make test input");
        dbg!(&s);
        parse(&s);
    }
}
```

You can run the tests the usual way with `cargo test` which should panic. I've
lobbed a `dbg!` in there of the transformed raw bytes into the string. When the
test panics we see:

```
running 1 test
test tests::fuzz_regression_01 ... FAILED

failures:

---- tests::fuzz_regression_01 stdout ----
[src/lib.rs:17] &s = "0+1"
thread 'tests::fuzz_regression_01' panicked at 'called `Result::unwrap_err()` on an `Ok` value: 1', /home/rjs/.cargo/registry/src/github.com-1ecc6299db9ec823/num-0.1.31/src/bigint.rs:388:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::fuzz_regression_01

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

And there is our offending input, `"0+1"`! Beautiful. I can use these specific
cases in issues for upstream projects instead of wasting time trying to find
exact cases on my own. Switching to `num` v0.1.41 avoids the panic and we can
run `cargo fuzz` again. We can also change the fuzzing library used, such as
libfuzz or [afl](https://rust-fuzz.github.io/book/afl.html), which `cargo fuzz`
supports. You can also use `honggfuzz` via the `honggfuzz.rs` library over at
[honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs). Different fuzzers
have different features and guarantees but most are relatively easy to write
targets for and get fuzzing so it can pay to try a few alternatives to see if
other cases are lurking around.

Before I go, I want to talk about a related concept known as "property based
testing" where we define how random data should be generated. We'll dig more
into that another day but for now, it suffices to say that both approaches help
to drive out test cases you might not have imagined! I am in the habit of making
my own regression suites from cases I find from either method, but you don't
_have_ to do this as some fuzzers and property-based testing libraries will keep
a "corpus" of data that has failed that it can try again on future runs, but I
find it helpful to have the regressions as unit tests so there is a fast way to
verify earlier failures aren't still happening.
