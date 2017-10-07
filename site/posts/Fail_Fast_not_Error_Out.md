---
title: Fail Fast not Error Out
author: Ryan James Spencer
date: Sat  7 Oct 12:50:02 2017
tags: [software]
---

**tl;dr** Static analysis is a form of 'failing fast' that does not consist of
leaving error based exit strategies (which should be reserved for situations
where the program simply cannot transition to a new state) in code that will
eventually be shipped to production.

The notion of 'failing fast' in programming details finding faults at the
earliest possible time; when the application developer is fitting out the code!
This seems to be sensible, but is often strangely antithetical to the notion of
'the only true test of code is production data'; how can we fail fast and catch
a ton of bugs when the truly icky bugs we want to smash are after we've done
some kind of deployment? Clearly the distinction here is to find bugs, in any
context, as soon as possible, production or otherwise, but that does mean the
concept can be carried over to production, where failing fast could mean major
problems (payments not being processed, account information being leaked, etc).

Ops people have devised all sorts of methods to roll out code in deployment to
handle situations like this; blue-green deployments, canary deployments, et.
al. all focus on testing code on a much smaller subset (on some segment of
traffic) accepting _some_ failure as an acceptable loss to know if the code is
ok enough to push to 100% of the traffic. Percentage deployments put a lot of
focus on monitoring and logging. Essentially, people have to watch the metrics
after the roll out to make sure everything is ok.

A computation does not need to crash the program in order to fail fast:

* Errors are for irrecoverable states of program transition; the program
depends on writing to disk for some critical task, and the disk has been ripped
out of the server rack and can no longer be accessed via the kernel drivers.
The kernel tells us something very bad is up, and we die. This is fine, because
there's no sensible state to transition to in this scenario.

* Exceptions are for situations where something bad happened, but it's not bad
enough to cause us to fail completely, i.e. we can do something to transition
to another sensible step. The general frame of mind is that exceptions can be
problematic when they are not caught, but can be a pain to constantly look out
for (this is the source of the 'checked exceptions' controversy in the Java
community). The primary problem with exceptions is that if an exception is not
'checked' or 'caught', then it will bubble up to the main function (entry
point) of the program and cause it to error out as above. Exceptions are said
to be sensible if they preserve **progress** and **preservation**, meaning that
they are able to move forward and they don't manipulate the types of
expressions where they are thrown. In most languages, however, we can't be sure
if something is going to throw an exception, so many programmers are told to be
defensive and paranoid; hardly the kinds of things you'd want out of people who
need to also be innovative.

In most pure functional programming languages, we know less about lurking
exceptions, and this is of particular importance. When we have a type system,
which is effectively a lightweight proof system that gives us static guarantees
and checks at compile time (a form of 'fail fast' but without the problem of
leaving 'ticking time bombs' in our code base that may still present themselves
in production), then it makes no sense to fail fast in an error-prone way.
Abstractions such as monads and friends allow us to do this elegantly and
tersely.

It is far more ideal to let pure computations transition gracefully to new
states, failures to be found at _compile time_, and production code to be
robust and resiliant. If we extend this notion of static analysis to property
based testing, formal correctness practices, and even linters, among other
things, there are several smarter alternatives to failing quickly and
validating the correctness of our programs.
