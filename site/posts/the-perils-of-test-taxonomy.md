---
title: The Perils of Test Taxonomy
author: Ryan James Spencer
date: 2019-12-08T04:53:57.916842789+00:00
tags: [the lowly assert, test taxonomy, test classification, tests, testing]
summary: >-
  You are wasting your time by classifying tests. Instead of discerning what
  defines a test we'll hone in on tests to avoid.
---

You are wasting your time by classifying tests. Instead of discerning what
defines a test we'll hone in on tests to avoid. If a test is:

* slow
* flaky
* or subject to churn as new features are added

then delete the offending test right now.

For testing to work your test suites can't be grounds for noise pollution. Nor
can they be a museum for specimens fit for dissection. Decide on what you want
to guarantee and work to achieve that guarantee _within contraint_. **Tests
themselves are un-tested chunks of code.** Tests that exhibit any of the
characteristics listed above lose local reasoning and are, therefore, hard for a
human to verify.

Slow and flaky tests mean you can't form a feedback loop with them. It means
people will stop running the test suites to drive development. I often will
chalk up work in CI for build bots to test and also test things locally at the
same time, racing the two to get feedback as soon as possible. Tags and simple
test names provide a handle to hone in on specific areas of functionality that
can be verified as new features are added. Fast tests also mean people will add
more tests and while a test _suite_ might continue to increase in time needed to
finish, it is arguably a point to break test suites up into new test suites and,
possibly, separate libraries and programs that have their own test suites.
Decomposition shows its beautiful face once again.

A non-deterministic (i.e. flaky) test may seem to _sometimes_ provide a
guarantee but the reality is much bleaker: a non-deterministic test tests
nothing. I am not talking about tests that fail because of the occasional
third-party service going down or network issue. I know you will be accordingly
[play-fighting with swords](https://xkcd.com/303/) if that happens. What I am
referring to is the situation where tests are _known_ to occasionally but the
reason is unclear. Is it configuration with a database? A third party library?
Some state setup or internals of the subject of the test? Flaky tests are white
noise. Devs start to ignore them and must waste time determining what is at
fault if they are to ascertain if the test failure is because of something they
should truly be concerned about or "just because".

It is also a waste of time when a new feature is birthed into the system only to
lead a dev on a surgery process of fixing an array of tests that now fail. This
is distinct from intentional changes: a test might need fixing because you are
intentionally migrating away from some older behaviour into a new one and doing
so in-place. But tests should have isolation: bringing in new functionality
shouldn't _necessarily_ mean overlap on older functionality and, therefore,
older tests.

It's helpful to delete tests and see if you would passionately defend against
their deletion in the process. If there is no passionate defense you will not
likely miss them when they are gone. A giant wall of tests is also a giant wall
of maintenance burden and there is only so much energy a group of persons can
apply to maintaining something they don't care about whatsoever.

Tests and types provide a degree of confidence, one that allows us to assuredly
tell others something is _more likely_ to be correct, such that is to say it is
aligned with some specification or set of requirements. **Lacing your codebase
with questions that can be quickly answered with a clear yes or no helps aid
confidence.** Debating if something is _truly_ a unit test or integration test
or whatever test is the equivalent of the art communities cliché  of _"but is it
art?"_; humorous but not useful. Along with foundations such as quality release
and deployment engineering, operations, visibility into running systems, and so
forth, pushing things out to production becomes trivial with time. I obviously
and hand-waving away from the concern of scale here. Scale drastically impacts
trust and confidence, but many organisations are still paving a path forward and
charting new territory in this space to still make shipping code something sane.
Whatever you take from the above, the most important aspect about any kind of
testing is to make sure you are asking yourself one primary question when
writing tests: [**What will you
assert?**](https://www.justanotherdot.com/posts/the-lowly-assert.html)
