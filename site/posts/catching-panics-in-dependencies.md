---
title: Catching panics in dependencies
author: Ryan James Spencer
date:
tags:
  - rust
summary: >-
  If you have ever had a crate panic on you, you can feel it was random as the
  specific conditions that trigger it may seem unclear. Panics are safe in Rust
  but should be reserved for cases where we cannot recover from the mistake and
  need to tear down the entire process to avoid bad things happening in the
  future. There is no static analyzer that will currently let you find all panics
  in your own crate _plus_ all the panics that dependencies might be doing.
  However, you can get pretty close with fuzzing!
---

If you have ever had a crate panic on you, you can feel it was random as the
specific conditions that trigger it may seem unclear. Panics are safe in Rust
but should be reserved for cases where we cannot recover from the mistake and
need to tear down the entire process to avoid bad things happening in the
future. There is no static analyzer that will currently let you find all panics
in your own crate _plus_ all the panics that dependencies might be doing.
However, you can get pretty close with fuzzing!

Fuzzing is a way to take random bytes, shape them into the shape we want, and
chuck them at our interface in question to see how it responds. Some fuzzing
libraries take input that lead to a crash and continually mutate it to find
other cases where it might crash as well.

A related concept is property based testing where we define how random data
should be generated for types or from functions. We'll dig more into that
another day but for now it suffices to say that both approaches help to drive
out test cases you might not have imagined! When you find failures from fuzzing
or property based testing it can also pay to keep a regression suite of the
failures so it's clear which cases have failed in the past. You don't _have_ to
do this as some fuzzers and property based testing libraries will keep a
"corpus" of data that has failed that it can try again, but I find it helpful to
have the regressions as unit tests so there is a fast way to verify earlier
failures aren't still happening.

For now, let's see if we can use fuzzing to catch some panics in an external
library we might be using. [Here's a
case](https://github.com/rust-num/num/issues/268) taken from the `rust-fuzz`
organisations [trophy case](https://github.com/rust-fuzz/trophy-case). I'll use
`num` v0.1.31 which panics when parsing `BigInt`s. I'll add it to the
`Cargo.toml` of our project:


```
[dependencies]
num = "=0.1.31"
```

Then, I'll install `cargo fuzz`.

```
cargo install cargo-fuzz
```

then in our project we can initialize cargo fuzz.

```
cargo fuzz init
```

Which gives us a single, initial fuzz target which we can rename if we want.
I'll rename it to `parse.rs` so the name is `parse` when listed but you can call
it whatever makes sense. In order for this to work, we can change the `fuzz`
subdirectories `Cargo.toml`. You'll see a `[[bin]]` key in there that designates
what targets are available. Since we're moving `fuzz_target_1` to `parse`, we
have to do this in the toml file since the `cargo fuzz` subcommand doesn't have
this ability, but we _can_ use the `cargo fuzz add` subcommand to add extra
targets with custom names in the future.

The new key should look like this

```
[[bin]]
name = "parse"
path = "fuzz_targets/parse.rs"
```

with the rest of the `fuzz/Cargo.toml` left as-is.

Then we can write a simple case for our function. This can be a little tricky
because the fuzzer will hand us raw bytes and it's up to us to shape them into
the right format, whether that be a struct, i64, f32, or even a string, as we'll
see here:

```
#![no_main]
use fuzzing_test::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse(&s);
    }
});
```

and then we can run our fuzzing example:

```
cargo fuzz run parse
```

This finds the offending string roughly similar to the trophy case example quite quickly:


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

But that doesn't look like the offending case mentioned in the issue we linked
above from the trophy case, does it? Again, this is because it's the raw bytes.
A handy trick I use when running regressions is to utilize the stored failing
input. In this case it's stored at
`fuzz/artifacts/parse/crash-00a78c613a00b21ea723de12e16f32a0385d9bdc` per the
output above. We can write a regression that uses this directly:

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
lobbed a `dbg!` in there of the transformed raw bytes into the String. When the
test panics we see:


```
running 1 test
test tests::fuzz_regression_01 ... FAILED

failures:

---- tests::fuzz_regression_01 stdout ----
[src/lib.rs:17] &s = "0+1"
thread 'tests::fuzz_regression_01' panicked at 'called `Result::unwrap_err
()` on an `Ok` value: 1', /home/rjs/.cargo/registry/src/github.com-1ecc629
9db9ec823/num-0.1.31/src/bigint.rs:388:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtr
ace


failures:
    tests::fuzz_regression_01

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered
 out

error: test failed, to rerun pass '--lib'
```

And there is our offending input, `"0+1"`! Beautiful.  This gives us an exact
test case we can provide upstream crates with if the code isn't ours that is
panicking or we could write the code ourselves so we get the same functionality
without the panic. Switching to `num` v0.1.41 avoids the panic and we can run
`cargo fuzz` again. We can also change the fuzzing library used, such as libfuzz
or [afl](https://rust-fuzz.github.io/book/afl.html), which `cargo fuzz`
supports. You can also use `honggfuzz` via the `honggfuzz.rs` library over at
[honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).
