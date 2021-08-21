---
title: Write Honest Benchmarks
author: Ryan James Spencer
date: 2021-08-20T23:57:06.506319068+00:00
tags:
  - rust
  - performance
summary: >-
  Do benchmarks feel really _really_ hard to get right and that you are never sure you are measuring what you want to measure? It’s likely you feel this way because benchmarks are like mini experiments; you run the experiment to see if there are reproducible results given some fixed parameters. Yet despite the relative ease of running experiments and collating data, experiments can be a net negative as they are prone to lie if you don’t heed some straightforward mistakes.
---

Do benchmarks feel really _really_ hard to get right and that you are never sure you are measuring what you want to measure? It’s likely you feel this way because benchmarks are like mini experiments; you run the experiment to see if there are reproducible results given some fixed parameters. Yet despite the relative ease of running experiments and collating data, experiments can be a net negative as they are prone to lie if you don’t heed some straightforward mistakes.

What do we want out of benchmarks?

* Insight on the impact our changes have on runtime characteristics
* To prove to others that the system in question  possesses or lacks runtime characteristics

Thankfully, the tradition of experimentation and statistics is a long one, full of lots of lessons learned. Many of the tips I’m about to go into I’ve learned the hard way, but I can say that once you have these in hand, writing reliable, honest benchmarks should feel painless.

## tl;dr

Much of these things can be tackled by choice of benchmark harness. Other of these are important to keep in mind as you write benchmarks.

* Choose a benchmark harness that runs your code across many iterations, performs trial runs, and tells you a good range of statistical information to help you confirm if you can trust the result
* Randomize the order of benchmark runs, as well as varying the workloads you feed them, to root out pathological cases
* Treat your benchmarks as an extension of a profiler, instrumenting many facets of the system as a whole to gain insights on the relative sizes each part plays in the bigger picture
* Pin down as many details as possible and mention them in documentation paired with the benchmarks
* Run benchmarks on as quiet a machine as possible, unless the intent is to draw conclusions about how the system will run on a fully loaded host, in which case be clear about this distinction.
* Verify the code under measurement is what you expect it to be by measuring overheads or inspecting generated assembly. Although you can guard against. compiler optimisations by using `black_box`, use it sparingly as it is unlikely that unoptimised code is what you want to be measuring!

## Variance and you

The aim of the game in benchmarking is to derive a single figure of merit. This is usually an arithmetic mean. Why aggregate a mean when we can run the program once and get a number we can use immediately? Unfortunately, if we did that, we’d be potentially hiding away all sorts of useful information. Consider a system that half the time observed performs some task in 20 milliseconds and then the other half of the time observed performs the task for 20 seconds. If we were to accept the first number we saw, we’d draw a drastically different conclusion depending on which number we saw!

By taking the average, we understand a better sense of _center_, but we can’t just accept an average by itself, either! If we did, we’d ignore how widely spaced our observations are. The greater degree of variance across the numbers, the less we can trust this sense of center. Thus, when we run benchmarks we generally

* Run many iterations of the experiment
* Average the results, and
* Report the range of the results and their standard deviation

This gives you a decent amount of information to tell you if stray statistical outliers are tugging an average in a certain direction or if the results are all over the place. If timings are all over the place, the only reasonable conclusion we can draw from the result is that the system under test does not display dependable characteristics.


## Variety is the spice of life

Hardware is stateful; data stored in memory, caches, buffers, and so on, from prior runs can have an impact on making a benchmark look better or worse than it might have if it was run in a different order or if it didn’t have warmup runs. If you run warmups, ensure their results are included in the mix or they may show up as unexpected variance in the results.

It’s additionally important to treat your benchmarks as an extension of a profiler. While performance isn’t necessarily additive, it is imperative to observe the different parts of the system to understand the relative sizes of the role of each part in the sum. It is tempting to look at a single part of a program and label the whole thing with “bad” or “good”, but understanding if a single bottleneck is skewing results helps you and others better understand the strengths and weaknesses of the system as a whole.

Lastly, it’s important to consider varying benchmark inputs or “workloads”. The purpose of this is to understand corner cases. Regular strides or powers of two may seem fine, but if you don’t explore the crevices between these inputs you’ll likely leave pathological cases lurking behind what seem like perfectly fine benchmark results.

## Be precise and be public

A common mistake with benchmarks is assuming two environments are comparable. Hence a developer may run benchmarks on their laptop and try to infer the runtime characteristics of a production machine from the local results, or vice versa!

The answer to this is to be public with your results. Documenting everything will force you to get feedback early about their legitimacy. A great way to drive this publicity is by documenting everything you do. In part of documenting everything, I find it helps me actually pin down specific details of the benchmarking: what configurations am I opting into? What exact hardware am I running things on? If it is variable, it’s likely to change when others are to run my benchmarks.

Speaking of comparing machines, it’s also to consider a machine that is quiet versus a machine that is in full effect running other programs is not the same environment: always prefer to run your benchmarks on as quiet a machine as possible.

## Compilers are aggressive optimisers

With benchmarks, you usually will be running code that has had some or all optimisations turned on. However, compilers will do all sorts of things to make code fast, which can involve replacing or removing whole chunks of code, for example.

If you want to verify that the code you are benchmarking is exactly the code you expect it to be, you can always check the assembly in either godbolt or generated locally. The one benefit of checking the assembly locally is that you may have, for example, the `target-cpu` flag turned on with an architecture that isn’t supported by godbolt, thus you’re assembly output will best match what was chosen for your target environment.

If you want the compiler to skip optimisations in rust within a benchmark, you can always use the `black_box` hint, which will issue a volatile memory access crippling optimisation attempts in the resulting code block.

“But I _want_ the compiler to optimise this chunk of code!” you say? And you’re right! The compiler _should_ be allowed to optimise away at will! Using `black_box` is a blunt tool used in specific places rather than huge chunks.

A traditional trick is to ensure the code you are testing is in a dedicated function, and that the benchmark is calling that function, and that the compiler isn’t aggressively replacing _that_ function call. Then, you can write a benchmark to record the overhead of a function call separately, and use that as a baseline against the specific code you are expecting to measure.

You can do this for other overheads, too. If there is some overhead I want to compare that isn’t necessarily a function call, I will sometimes `black_box` the thing I want to record the measurement of, say a loop with a particular number of iterations, and put something like an `asm!("nop")` in there to ensure that no actual activity is taking place, but that the compiler won’t look at this code, rightfully determine it does nothing, and delete it.

## Conclusion

Hopefully each of these has shown you a way to improve your benchmarks or benchmarking harnesses. The aim of the game is to have dependable, reproducible ways that provide insight into our systems, forming a foundation for driving improvements over time rather than making lofty guesses about the overall performance of a program.
