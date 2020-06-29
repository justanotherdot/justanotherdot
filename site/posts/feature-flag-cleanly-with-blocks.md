---
title: Feature Flag Cleanly With Blocks
author: Ryan James Spencer
date:
tags:
  - rust
summary: >-
  If you're tired of polluting your code with one-off functions and identifiers,
  then you can turn to blocks in Rust to simplify the mayhem. I explore how to
  feature flag code using blocks as well as how to use the same pattern to make
  your code easier to read and maintain.
---

I recently had to feature flag some code in JavaScript and felt myself reeling
for blocks in Rust. In the JavaScript code, I didn't want to break out the logic
into it's own function or module as I didn't want to dedicate to an interface up
front as the code was a proof of concept. In languages that aren't expression
and block oriented, you have one of two choices:

1. Use an immediately invoked function
2. Have 'unset' variables on the outside of some scope and a scope that
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
assignments that break up code like a newspaper but there is some contestation
around shadowing. I am pro shadowing in Rust as I feel it bars a class of bugs,
but you can understandably avoid shadowing if you so care, [using `clippy`s lint
on the
matter](https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/shadow.rs),
or you can ensure the temporary shadowed variable(s) is only present for the
inner, temporary, scope, if it bothers you.

The best part is that when the "one-off" value becomes less "one-off", you can
easily take the block and dump the contents straight into a function and it will
work as-is, with or without the superfluous curly braces. Splitting up the
decisions around what is done to build up the value versus the surface area of
the value is a great way to ensure the interface is what you really want it to
be, rather than constantly shifting it in tandem while you work out the details
of the internals.
