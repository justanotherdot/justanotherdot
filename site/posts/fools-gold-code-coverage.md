---
title: "Fool's Gold: Code Coverage"
author: Ryan James Spencer
date: 2019-10-01T20:41:51.704360598+00:00
tags: [fools gold, tooling]
summary: >-
  If you are unfamiliar with code coverage, the idea is simple: you write
  accompanying tests to code and a code coverage tool produces reports of lines
  covered by tests and the percentage of that coverage to all lines of code. The
  hope is that a higher coverage with tests means you'll have a 'correct' system.
  I have even heard of some establishments initiating quotas on required coverage
  per lines of new code being introduced. "If it doesn't have tests it doesn't
  exist" is the usual argument for this requirement; code without tests is
  potentially problematic code, but tests are also untested chunks of code in
  our codebase. For example, consider this bit of React code:
---

If you are unfamiliar with code coverage, the idea is simple: you write
accompanying tests to code and a code coverage tool produces reports of lines
covered by tests and the percentage of that coverage to all lines of code. The
hope is that a higher coverage with tests means you'll have a 'correct' system.
I have even heard of some establishments initiating quotas on required coverage
per lines of new code being introduced. "If it doesn't have tests it doesn't
exist" is the usual argument for this requirement; code without tests is
potentially problematic code, but tests are also _untested chunks of code_ in
our codebase. For example, consider this bit of React code:

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
system, or please the percentage going up, and focus less on the guarantees that
tests are providing them. **This does not help us deliver better products to end
users.**

[Code coverage percentage is a useless
metric](https://twitter.com/KentBeck/status/812703192437981184). There is no way
to know what percentage of code written is code your end users actually care
about when that percentage is derived from synthetic traffic. You may write a
continent of code, but only a thousandth of that code may actually be hit by
users. If you are using code coverage to tell you that you have greater than
zero percent code coverage than you have a code organization issue or there is
the possibility that many or all of your tests are false positives.

Here's a different approach: instrument your application to track invocations of
code paths. You can do with this tracing, structured logging, profiling replayed
traffic, etc. The technique employed doesn't matter but what does is determining
what is valuable and what is dead. Regardless of collecting metrics, you will
always need to consider this [from a human
perspective](https://kentcdodds.com/blog/how-to-know-what-to-test).

Detractors may argue that code that doesn't immediately show usage should not be
hastily deleted. They are partly right. If things are early on at your company,
doing eyeball statistics may be fair but eyeball statistics is not real
statistics. Practicing some basic statistical understanding is always in order
for any kind of analysis. It may take time to reach a statistically significant
result and whether one is reached should, more often than not, drive your
decisions. As noted we need to exercise judgement despite what the numbers may
tell us. Perhaps the piece of code in question is a critical piece of error
handling that is rarely executed, for example, or maybe the code is serving a
particularly infrequent, but high-paying, user base.

I'd like to stress that I am **not** saying that testing is a pointless errand.
What I am saying is that **code coverage is fool's gold**. Tests take more
effort to create because developers must check that a test is actually testing
what you think it's testing. [Testing is a part of one's confidence
level](https://stackoverflow.com/questions/153234/how-deep-are-your-unit-tests/153565#153565)
in what they ship, and when we assert important properties of a system are
upheld you boost that confidence level. **Don't buy into the idea that coverage
is going to lead to a correct system by default.** Be vigilant with you tests
and instrument your application.
