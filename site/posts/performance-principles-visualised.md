---
title: Performance Principles, Visualised
author: Ryan James Spencer
date: 2021-10-24T02:35:01.709259730+00:00
tags:
  - rust
  - performance
summary: >-
  Not all of our decisions around performance are going to be driven after the thing we want to optimise has been built. In order to design programs with performance in mind at the start, we need a set of heuristics to guide us to make better choices.
---

Not all of our decisions around performance are going to be driven after the thing we want to optimise has been built. In order to design programs with performance in mind at the start, we need a set of heuristics to guide us to make better choices.

Often performance gurus have a set of principles they mention early on in their seminal texts. These are useful, but usually only given a short segment. I want to expand on these. Over time, through profiling and observing systems, you’ll grow an intuition for what will and won’t make an improvement. Sometimes your intuition will be wrong, but the probability of you being wrong versus right doesn’t mean that you shouldn’t be thinking abstractly about performance at a higher level.

Some of my favorite mathematical texts focus on visual or geometric interpretations of their subjects. I am a big fan of systems thinking, and I have, over time, grown to relate most of the principles I’m about to discuss with a specific, well-known degenerate system known as a “pipeline”.

For the uninitiated, a pipeline is a system representing very clear entrances and exits, with obvious stages that may, themselves, be pipelines if we are to zoom in on them. We compose stages together into a pipeline to simultaneously increase isolation between concerns and specific transformations while also increasing cohesion of the pipeline as a whole. Pipelines are used in a variety of places where both performance and rigor of correctness matter, and while they are not the only solid architectural pattern, they are definitely one of the foundational ones.

Pipelines are a great way to visualise these principles because they allow us to focus on how the principles work both on the local and global levels. The system as a whole is improved by applying a principle, but we can also see how it was the direct result of a local change. Conversely, sometimes we apply a principle generally across all stages, and a local stage or two in particular are the primary benefactors.

There are a handful of meta principles at play across several of these. Some have to do with laziness, others have to do with bounding resources. There is also the view that every stage of a pipeline represents a layer deeper into a system. As data progresses from stage to stage, we can also see that in a different light as data passing from one layer into the next. However, unlike most client-server architectures where data trampolines, the pipeline’s exit is at the final layer.

## The Principles

### Don’t do it

When we break down a large transformation into a pipeline, we are actively seeking how to decompose the transformation into steps or stages. Stages let us clearly see what can and can’t be removed. If a step isn’t necessary, or is duplicating work that is done earlier on, we can avoid doing it altogether.

Additionally, we can choose to not support specific functionality in order to simplify and reduce the work that the overall pipeline has to make. If I make an explicit choice to reduce expressiveness, I may be making things less ergonomic for users in one way, but also drastically improving ergonomics via another way, namely latency. Fast incurs usage.

### Reduce, Reuse, Recycle

This is partly noted in the last principle, but the idea is that every stage should be focused on producing an output, and should not be repeating work that a prior stage has performed. This isn’t quite the same as having multiple passes on some data, because maybe each stage needs to look at the same data over again, but this is more that if we have created a specific output, only to later recreate the same thing, we are creating waste that should be pruned.

Recycling earlier work forms the backbone of larger techniques such as immutability, incremental computation, dynamic programming, buffers, caches, and so on.

### Skip the travel time

In the same way that keeping tools or information close by is easier than having to drive out to some distant location every time you need them, the same is true for relevant details to the pipeline. This practically means making things cache friendly, or reducing pointer chasing on the way that data is structured in a program.

### Work on reasonable chunks

Working on small morsels of data is inefficient in the same way that working on the entirety of the data is inefficient; the more that we can work on at a given time is good, but only up to the point that we can manage. After a particular point, working in large multiples may not be friendly to cache line sizes, or it can cause thrashing on the operating systems virtual, paged memory.

Bulk interfaces over unit-level interfaces, loop unrolling, as well as streaming, and by consequence stream fusion, are all great examples of working on ranges or runs of data. We may internally bound to a particular buffer size, but simultaneously elevate our work on several things at the same time. This is distinctly different from scaling techniques such as parallelisation we’ll talk about later as this isn’t saying the units in the multiples will be worked on exactly the same time, but instead is saying that a bigger payload upfront or at each stage can help speed things up tremendously.

### Focus on throughput

Choosing what to do is a cost. Every decision requires having to expend the actual energy contemplating what is the right thing to do. In programs this means computing conditionals and performing checks, as well as jumping to the correct, new location of code. Most modern processors perform branch prediction, in which the processor tries to make an educated guess at which branch will be taken, preemptively executing the guessed code before the decision is verified. If it turns out the decision is wrong, we have to throw out a lot of work in order to perform the alternative. Hence, whenever we can eliminate the need to decide, we are likely to reduce an unnecessary cost.

Sometimes performing both outcomes may be far cheaper than deciding which to do, and it also allows future invocations of the code to potentially reuse the results. We can also achieve this by writing direct bitwise operations that compute results without needing to introducing any conditional tests or branch logic. It’s important to remember that the moment you start writing bitwise code, you should verify the compiler isn’t already making a similar optimisation or better.

### Bring the cheap work upfront

Amortization is the act of performing work at infrequent intervals such that the cost of the infrequent activity is outweighed by the alternative cost of doing it frequently. Dynamic arrays such as vector do this by growing two times in size every time they reach capacity. Ideally we don’t ever need to make this decision and we know the exact size upfront, which is even better. Furthermore, we may want to push certain activities in the front the same way that IO bound tasks are favored in modern operating systems to improve user interaction latency by deferring CPU bound tasks. Paying for a scheduler by using async/await in your programs is another way that we can move particular classes of activities upfront given their priority, either through push or pull semantics or via characteristics of the task being invoked.

In the more general sense of a pipeline, every stage that we get through is more costly to bail at. If we can figure out that things are wrong early on, we can avoid doing lots of unnecessary work.

## Parallelisation is a scaling technique

Pipelines have clear entrances and exits for a reason; they showcase how data transformation is a sequential operation that is composed of several stages which, themselves, can be pipelines and so on ad infinitum.

However, most people get told to write fast programs by parallelising them out of the box. Parallelisation should be seen as a scaling technique, it allows us to oversee how fast we can make the straight line first and where dependencies lay, and then we can trivially parallelise the isolate parts. It should always be safe to work on multiple chunks by scaling a stage horizontally, and in turn it should be easy to work on multiple raw inputs by horizontally scaling the pipelines themselves. As noted, we can also parallelise the throughput oriented cases, which may run in tandem, even though only one will be chosen or used subsequently later on the pipeline. And lastly, we can parallelise the stages themselves, in what is known as “pipeline parallelism”, where each stage does what work it can in isolation until it needs to block on the stage before it for more input to process.

At the instruction level, this is exactly what SIMD et. al. do; Single Instruction Multiple Data allows strips of data to be processed in true parallel at the hardware level as there are no shared dependencies. All we are saying here in this section is that we can take this same thinking to a systems level to identify where we may or may not need to take advantage of immutability for isolation-safe sharing of data, or conversely pay the tax of lock contention for consistency of reads and writes on solitary allocations of shared memory.

## Instrumentation

The other advantage of designing things in terms of a pipeline, or any system for that matter, is how it allows us to clearly identify where we want to put instrumentation to help guide us after the implementation of the system is complete. This means we can think about how we are going to profile and benchmark our code from the start, and be able to observe, i.e. ask questions of, the system as it continues to run.

Instrumentation is often an afterthought. If we include it as part of implementation stage, then it may be unclear if what we are instrumenting is worthwhile, although we may be unduly smug in the sense that we are collecting some kind of metrics. If we, instead, focus on instrumentation as a thought bubble with respect to the design of the system, we can better see where, and therefore what, instrumentation should be plugged in.

## Conclusion

Learning about patterns of systems has been wildly more effective a way to view performance tuning heuristics and principles, as well as understanding “algorithms and data structures” through the lens that they are patterns themselves, is wildly more effective than learning specific implementations as it allows you to do what we just did where we can see how specific principles apply to the design in question.

Pipelines are such a common trope, whether it’s the stages of a compiler, a graphics rendering engine, or a query interface for a database, that knowing _why_ they are amenable for solid performance design, and not just optimisation after-the-fact, is worth revisiting over and over again.  In my mind, these foundations form the basis of lots of other performance discussions and changes. Knowing why you want to do something allows you to work with intent, and working with intent in relation to performance means you aren’t taking wild stabs in the dark at what may or may not work. It isn’t to say all these principles can blindly lead to results, but having a mental model for a system of how your code is designed will allow you to think about performance for the model rather than getting bogged down initially with the specifics of code.
