---
title: Feature Flag Cleanly With Blocks
author: Ryan James Spencer
date: 2020-06-29T09:45:44.592349850+00:00
tags:
  - rust
summary: >-
  If you're tired of polluting your code with one-off functions and identifiers,
  then you can turn to blocks in Rust to simplify the mayhem. While you can use
  blocks to feature flag code, the same pattern applies to general refactoring,
  making your code easier to read and maintain.
---

I recently had to feature flag some code in JavaScript and felt myself wishing I
had Rust's expression-based blocks. In the JavaScript code, I didn't want to
break out the logic into its own function or module as the code was a proof of
concept and deciding on an interface early on would distract me. In languages
that aren't expression and block oriented, you have one of two choices:

1. Use an immediately invoked function
2. Have 'unset' or 'default' variables on the outside of some scope and a scope that
   potentially assigns to the variables

Here's an example of the two cases in JS:

```
// with an IIFE.
const featureFlaggedItem = (() => {
  if (!featureFlag) {
    return {
      // defaults without the flag.
      // the steady state of the system.
    };
  }

  // feature flagged code.
})();
```

```
// or with an assignment, relying on side effect.
let featureFlaggedItem = {
  // defaults without the flag.
  // the steady state of the system.
};
if (featureFlag) {
  featureFlaggedItem = {
    // featureFlagContent
  };
}
```

The problem with the side effect approach is that we lose the nice benefit of
having `featureFlaggedItem` as `const` as well as keeping things nice and tidy
for easier deletion later on. I personally refuse to use the second approach.
Rust let's you easily write the above code as following:

```
let feature_flagged_item = {
  if (!featureFlag) {
    return {
      // defaults without the flag.
      // the steady state of the system.
    };
  }

  // feature flagged code.
};

// or ...

let feature_flagged_item = if (featureFlag) {
  // feature flagged code.
else {
  return {
    // defaults without the flag.
    // the steady state of the system.
  };
};
```

But this is wildly useful for a lot of things beyond one-off changes. Maybe you
need a one-off value for a function argument but you don't want to immediately
invoke a closure or define a function to call. With blocks you can tuck all
sorts of code into places with or without assignment. If you have a lot of
"identifier" pollution going on in a given scope, say with a lot of temporary
variables, you can tuck them under the rug with blocks. I tend to have a lot of
assignments that break up code like a newspaper article but there is some
dispute around shadowing. I am pro shadowing in Rust as I feel it bars a class
of bugs, but you can understandably avoid shadowing if you so care; [using
`clippy`s lint on the
matter](https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/shadow.rs),
or ensuring the temporary shadowed variable(s) are only present for the inner,
temporary, scope.

The best part is that when the "one-off" value becomes less "one-off", you can
easily take the block and dump the contents straight into a function and it will
work as-is, with or without the superfluous curly braces! Splitting up the
decisions around what is done to build up the value versus the surface area of
the value (its interface) is a great way to guarantee the interface is what you
really want it to be rather than what it had to be in order to figure out its
implementation. In general it's best to split up work in such a way that you can
focus on each piece in isolation without the other pieces distracting you.
