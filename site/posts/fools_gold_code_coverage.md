---
title: "Fool's Gold: Code Coverage"
author: Ryan James Spencer
date: 2019-10-01T11:05:06.811504920+00:00
tags: [fools gold, tooling]
---

If you are unfamiliar with code coverage, the idea is simple: you write
accompanying tests to code and a code coverage tool produces reports of lines
covered by tests and percentage of code covered to all lines of code. The hope
is that a higher coverage with tests means you'll have a correct system. Some
places even initiate quotas on required coverage per lines of new code being
introduced. "If it doesn't have tests it doesn't exist" is the argument for this
requirement; code without tests is potentially problematic code, but tests are
also untested chunks of code in our codebase. Consider this bit of React code,
(assuming jest as the test framework):

```
test('Breadcrumb renders', () => {
  expect(() => {
    <Breadcrumb/>
  }).not.toThrow();
});
```

What is this testing exactly? Literally any other test, even one without the
`toThrow` expectation, would mark this as a failure on an exception being
thrown. This will light up code coverage though. People learn to cheat the
system, or please the metric of code coverage percentage going up, and focus
less on the guarantees the tests are providing them.

Does this help us deliver better products to end users? No.

Code coverage percentage is a useless metric. There is no way to know what
percentage of code written is code your end users actually care about. You may
write a continent of code, but only 0.01% of that code may actually be hit by
users. If you are using code coverage to tell you that you have greater than 0%
code coverage than you have a code organization issue or tests are not being
verified on creation. The easiest way to kick the tires is by making a test fail
to ensure it's not always passing. Tests take more effort to create because
developer's need to ensure the tests are valid and actually checking what we
want to check.

Here's a different approach: instrument your application to track invocations of
code paths. You can do with this tracing, structured logging, profiling replayed
traffic, etc. The technique doesn't matter, what does is the focus on what code
production users indirectly touch.

There are some decent descriptions on how to think about this [from a human
perspective](https://kentcdodds.com/blog/how-to-know-what-to-test), and any
approach that asks developer's to take a pause to reason about their code is
remarkable in my book, but actually knowing what your users are using is the
best way to determine what is code is valuable and what is dead.

**Code coverage is fool's gold.**
