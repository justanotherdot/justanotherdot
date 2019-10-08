---
title: The Lowly Assert
author: Ryan James Spencer
date: 2019-10-07T11:43:52.107748360+00:00
tags: [testing, the lowly assert, principle]
---

There is one thing that ties all forms of testing together; **assertions**. The
lowly assert humbly serves whether it's as types, panics, automated tests, or
any other glorious form. Regardless of how it manifests itself, it allows us to
declare things about our systems or program and automatically check them.

But when people test they don't tend to think about what they are asserting.
I've met a great number of people who are taught testing as a mechanical
practice, one that is simply followed because of the social expectation that a
tested system is a 'correct' system. But what is correctness?

Correctness is not merely the absence of bugs. **Correctness is the assurance
that a system is doing as is intended.** This can be business logic or even
sterile concerns like if a function returns the right value given the right
inputs (forms of unit tests). It can be about output or generated content
looking the way it's supposed to look (snapshot tests). It can be about multiple
systems behaving when coupled (integration tests) or about whole flows of usage
(end-to-end tests or possibly contract tests). The things we are testing _for_
and the ways to test for them is vast.

It helps to think about blocks of computation as black boxes: inputs go in and
outputs come out. Assertions that need to be upheld,

* while things are happening inside of the box are called **invariants**
* before the box starts work are called **preconditions**
* after the box has finished work are called **postconditions**

There are also a number of general properties the box can uphold:
[involutivity](https://en.wikipedia.org/wiki/Involution_(mathematics)),
[idempotence](https://en.wikipedia.org/wiki/Idempotence),
[totality](https://en.wikipedia.org/wiki/Partial_function#Total_function), etc.
The specifics of each of these isn't important but the idea is that there are
reusable patterns for guarantees we can wish from our systems and programs.

This article is the start of many to describe how the varying forms of
assertions lines up with their respective forms of testing. There are even
meta-principles at play about asserting facts about systems that we should make
elicit in the hopes they better our testing in general. These explorations
aren't going to be exhaustive but I am hoping they help expand your mind in the
things you can ask your code enforce.

A quick journey and recap, if you will.

When you write a program, you might use a typed programming language. In this
case you can use types to encode facts about your problem domain and structure
of data. [With types we can help make illegal states
unrepresentable](https://blog.janestreet.com/effective-ml-revisited/).

Later, you are writing a program and you want to know it acts the way you are
expecting it to act. Compilation non-withstanding you start to run the program
and check the results manually. [But this sort of tedium is easily
automated](https://landing.google.com/sre/sre-book/chapters/automation-at-google/).
**Toil should infuriate you!** With this sentiment in mind you start writing a
program to run your program in different circumstances, hence automated testing
is born. Now that you have this tool in place, you can run tests on small things
all the way up to big things. When the assertions in question fail, the tests
fail.

When a system misbehaves, you might want to know immediately while you are
coding and what faster way to know than to have your program halt when an
assertion is not met. Perhaps a failure is even one which requires a process to
abort while running in production (a fatal error). The difference between these
two is the subject of recoverable versus unrecoverable errors, which I won't
indulge in here, but it suffices to say catching mistakes and misunderstandings
sooner is always better than later by [attaching these sorts of assertions to
forms of
panics](https://www.cs.tufts.edu/~nr/cs257/archive/jon-bentley/correct-programs.pdf)

Now your test suite tests both small and large. As these tests get more
complicated, assertions can be about _models_ of these systems; as state
machines or even where the inputs are generated randomly. [Property based
testing](https://www.youtube.com/watch?v=hXnS_Xjwk2Y) starts joining your
repertoire for this reason. For verifying raw memory access you consider
[fuzzing](https://en.wikipedia.org/wiki/Fuzzing). Perhaps the end-to-end tests
are brittle and always breaking which might lead you into [contract
testing](https://docs.pact.io/) two systems to ensure that the pre- and
post-conditions (read: the contract) are being met. Maybe there are extremely
complicated concerns such as concurrency and you write a
[specification](https://en.wikipedia.org/wiki/Formal_specification) in something
like TLA+ which can verify the model it describes as part of the tooling.
Specify the system or program abstractly and test that, instead.

Like anything, there are diminishing returns. Finding assertions everywhere
doesn't mean proving your TODO-list single page application with a theorem
prover or dependant types is worth the time, although if those processes were
more lightweight it _would_ probably be worth it! **Think of assertions as bets
that pay off when code is introduced that violates them.**

Systems come in all sizes but despite their mixed formats they are all guided by
principles. **Instead of thinking the path to correctness is forged by
mindlessly coding and churning out fixes, try to think about the properties you
want upheld, instead, and work to encode those in every possible assertion you
can leverage within reason.**
