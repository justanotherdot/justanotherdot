---
title: 'Grokking Code: Tips for Reading Source Code'
author: Ryan James Spencer
date: Sat 25 Nov 2017 14:14:23 AEDT
tags: [software, programming, tips]
---

I distinctly remember asking, to the response of a chortle, if a TA would fill
me in on their secrets to grokking codebases; In other words, how does one
'read' code? Weren't there any generic tips someone could impart?

There are both obvious and subtle problems with these questions: what if you
don't understand the languages syntax and semantics whatsoever? If you do, why
is the code so dense and confusing that you can't quite immediately 'see' what's
going on?

Obviously, a lot of people focus on the other end of this spectrum trying to get
people to write 'readable' code and this generally falls into the "I know how to
write dumb, simple code, that just does what it's supposed to do."

At one of my interviews the interviewer remarked of my code response "Yup,
that's what we asked for." and I think that says a lot; your code _should_ be
expressing the simplest thing it can. In other words, _cleverness is a sin_.

But here are some general tips to get things off the ground regarding reading
code that I have found help me when I am lost in a sea of clever or
hacked-together code:

* **You can read declaratively** Writing code can be 'imperative' or
'declarative', but I think this is also possible with reading code, as well.
Instead of seeing a chunk of code and thinking about what's going on at a
hardware or internal level, you can read things as "And then this fetches
another item from the cache iff that item is present."

* **It's not about syntax** Perhaps this is because I'm biased having dabbled in
a lot of different languages, but with practice you eventually can see code as
sections of thought. That is if the author managed to group things together
intelligently enough.

* **Fearlessness is key** Similar to the paradox of choice, there is also the
paradox of bike shed'ing where one spends far too much time trying to understand
a problem via external means and never gets into the heart of it. It's important
to simply dive in and _do_ for things that are difficult as that's the best way
to keep motivation up and drive forward. As time goes on _you will learn_ and
you simply must 'keep the boulder moving' as it were.

* **Be wary of comments (both in code and from colleagues)** People internalise
things differently from one another; I may personally see a chunk of code as
being the core business logic whereas someone else may feel the larger part of
everything is a grab-bag collection of various details spread across the system.
Both of us may be holding onto the same truth, but our view of it may not be the
same and in this regard you should be wary of comments from colleagues and that
which you find in code. In fact, this is generally why I favor code comments to
be short and sweet unless their intention is for automatic document generation
in which the goal is to make clear the larger pieces of the system and to
specify any assumptions being made or caveats to be fallen into.

* **Don't fear the paper and pen** Sometimes code isn't dumb and simple and it's
at that stage that one needs to either diagram the codification with pen and
paper or to (quite possibly) write smaller subprograms on their own to grok the
internals of the system. This is an especially useful tactic I use for
understanding large-scale system internals (e.g. OS', DBs, etc.) as well as
programming languages and the like. If a system is sufficiently complex, it can
be decomposed (whether it already is or not is another matter), and if a system
can be decomposed, that decomposition can either be A) decomposed further or B)
be turned into an experiment. This is actually a trick I picked up from The
Linux Programming Interface where author Michael Kerrisk recommends writing lots
of small programs when one is learning linux internals in order to better
understand the specific working parts. Eventually, these decomposed elements
will all fit together to form the larger picture (e.g. if you know about stacks
and you know about pointers, it may be trivial for you to understand how an
optimized exception system in a programming language might work).

* **Composition and decomposition really are the king and queen of all problem
solving** Learning a codebase is still a _problem to be tackled_, and the best
way to tackle problems is to decompose them into smaller parts that fit into
ones head and handle them from there. With solutions to these disparate parts in
hand, one can stitch them together to form a cohesive solution, and reading code
is no different. Breaking apart a codebase into smaller pieces is absolutely
crucial; it means that we can get past our initial worry of 'how am I going to
understand all of this?!' into 'Oh, it's just this, this, and _this_?'. There is
a reason why people argue that composition should be favored and it's because
every engineer worth her salt knows how to compose and decompose appropriatley.

* **Make it a habit** There are varying degrees of doc quality out there and by
no means am I recommending that documentation be discarded in favor of always
reading the source code but there is a reason why people say _the source is the
truth_. For the last year I've worked in some capacity with Angular and the
Angular2+ docs are horrifically missing lots of information. On several
occasions when things were unclear I went directly to the source code. This
technique can be used in tandem with the 5-whys (or what I like to call
'pretending you're a 3 year old'):  here's an example:

 Q. Why is sentry telling me that there are objects that are `null` for which it
 can't find methods and properties when we use `--strict` on our typescript
 compiler options?

 A. It always consistently looks like it's coming from Observables that were
 piped into `AsyncPipe`.

 Q. Why is `AsyncPipe` emitting `null` values?

 A. Because the observable piping a value in must not have any initial value
 (i.e. there is no `startWith`). It looks like `AsyncPipe` defaults to `null` as
 it's first value it will emit when nothing is present and a value is requested
 which seems a bit odd, but must imply (perhaps) that this design decision was
 put into place before TypeScript had the `--strict` flag in place.


