---
title: Performance Analysis is Cost Analysis
author: Ryan James Spencer
date: 2021-11-22T09:35:03.046987264+00:00
tags:
  - rust
  - performance
  - profiling
  - benchmarking
summary: >-
  It may seem obvious that performance analysis and cost analysis
  go hand in hand, but the truth is that for us to get the most of our programs
  and teams we need to focus on what all of our measurements are telling us.
---

Benchmarks and profiler in hand, you’re ready to start taking stabs to improve
your program’s performance. Looking around, there are some large numbers that
seems obvious to attack, but this is the start of a broken perception about
performance analysis. Bottlenecks are worth evaluating, but they aren’t the only
thing you should be evaluating. The title of this article may seem obvious at
first glance, but the truth is that **performance analysis is not about focusing
on one view on the measurements you’ve collected.**

**tl;dr** Next time you are profiling, benchmarking, or laying out plans for
what you’re about to build, think about *all* of the costs. By thinking
*holistically* you’ll better understanding the value of every expenditure,
meaning you can both build an intuition for the benefit of certain classes of
costs as well as tangibly improving overall program, and non-program,
characteristics. The value of some costs will be obvious, but if the value of a
cost isn’t apparent, it may still be valuable in an indirect manner; usually
this means an upfront cost that leads to improvements later on or a “tradeoff”
that improves a quality that may seem unrelated. For example, amortized growing
of dynamically resizable arrays is done on purpose to help improve overall
performance on the whole, despite the effort put in at each stage of allocation
and copying. Or, we may decide it’s not worth optimising anything in the program
at all because we need to work on a new feature that is much more valuable to
customers than a 5ns improvement to a rarely used endpoint. And when in doubt,
if your program is hurting others or damaging the world around it, consider if
the software should even exist rather than helping it accomplish its job faster
([hat tip to Itamar Turner-Trauring on this
point](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjbrLeX5rD0AhUnTmwGHb8uC_4QwqsBegQIChAB&url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3Ds_xWflagwbo&usg=AOvVaw32CWmP0sDX92TBm5MCbPLz)).

Costs are everywhere; they are in the pooled costs we see pile up as well as the
costs that spread about a program. These latter costs constitute the diffuse
profile. Turning your attention on evaluating all costs means changing your
attitude from being frugal in the local sense to being economic in the global
sense. In other words, you might save a lot on a big item purchase, but you
might equally save as much over the course of a year with smaller, consistent
savings over time. Having both as savings is the real aim of performance
analysis as cost analysis.

In this cost analysis view of the world, payments should come with returns.
Sometimes the return on investment is definite, but other times it is not, in
which case it is deemed either an acceptable or careless risk depending on the
variables at play. We can define the former as direct and the latter as indirect
returns.

If we write code that brings random allocations in the heap into a single
contiguous block of memory, we are paying for an indirect return on investment
such that every attempt to read the memory afterwards is now much faster than it
would have been.

Alternatively, a direct return for a cost paid out might be the actual core
operation we need done; consider the difference of running several instructions
to calculate a population count (number of ones) of an integer or calling a
processor specific `POPCNT` instruction.

Keep in mind that not all costs are about the characteristics of your program.
There are the costs of a team of engineer’s salaries or the readability of code.
What is wasteful for our system’s context can be considered an acceptable loss
if we consider what it pays for elsewhere.

Projecting what costs are going to be potentially encountered is just as
important as reviewing costs. This can take two forms: either considering the
budget(s) of what we’re about to spend, or projecting what we assume we might
spend. One gives us a threshold for which to judge future expenditures while the
other gives us a a baseline for where we might might know if we are drifting too
far where we thought we would wind up. The distinction is subtle but important.
With a budget you want to stay as close as possible to the line, while with a
projection you are not trying to aim for anything, really, but you know how far
off your estimate was when you wind up with something tangible.
