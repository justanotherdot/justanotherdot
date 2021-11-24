---
title: Pull Requests Also Go Stale
author: Ryan James Spencer
date: 2021-11-24T10:27:04.506901833+00:00
tags:
  - working
  - teams
summary: >-
    Finishing work shouldn’t be accomplished for the sake of finishing work. If we accept that pull requests aren’t precious, we accept that we make mistakes, and when we accept mistakes we give ourselves feedback on a variety of contexts.
---

Pull requests aren’t precious things. They propose changes, and there is often an unfair assumption in a company setting that they will always make their way to trunk. Pull requests, just like issues, go stale. Not just in respect to the being up to date with the latest changes that may have surfaced since the pull request has been raised, but also in terms of general age.

Why do pull requests fester? We’ve all done it; if I fix this one more bug, add this one more test, address this one more wave of changes, and whatever else might be necessary, I’ll have a change in place that I can finally merge. The sweet satisfaction of merging your changes on top of trunk and bliss herever after.

In reality, there should be a weekly non-zero rate of closing pull requests. Code will be complex or not related to the model used in helping conceptualize the change. It may have tradeoffs and bugs that are tricky to resolve. It may be an experiment or a proof of concept unfit for continued production use. All of these cases, and more, produce learnings. The sludgefest starts as soon as someone wants to get to done because it will end the discomfort of the work continuing, but struggling isn’t the answer; starting fresh is.

Here’s a helpful tip I’ve learned in the last year. It is a variation of Kent Beck’s “test AND commit OR revert”. In Kent’s system, every time you make some changes, you run the test suite and see if it goes green, and if it does, you immediately, automatically, commit the changes. This forms a growing wave of passing changes, moving the baseline every time something proves itself it works. We can de-automate this process and take a slightly more general approach. Every time we have a broken test suite or even broken compilation, say, we don’t thrash any longer than 30-60s. Sometimes we need to simply cut wood and carry water to get to the end of it, which may be longer than a minute, but if we feel like we are not continually making progress, we should reset our expectations back to the clean slate, except the clean slate is the _last_ clean slate we were at. Making incremental progress in this way is a core tenant to working without churn. You don’t need to blow away everything you’ve done and start clean, although that’s not necessarily an impossibility given the circumstance, but you can establish a line where you’ve completed work that can be truly called done and move forward in a monotonic manner.

Teams can work together to establish this practice. When the author of a change is in the middle of a sludgefest, they often don’t see it themselves. It takes practice to realise that work is sitting about, no longer fresh or relevant or applicable. In this situation, it should be ok for others on the team to be able to make a comment in a blame-free manner that the pull request should probably be closed. This isn’t about the author being bad at code or engineering or whatever; it is simply a friendly reminder to get out of the tunnel and back into the sunlight. This can be a comment, or even provided as a proper review.

A specific approach we felt might work isn’t actually feasible or has unexpected tradeoffs. An implementation isn’t going as intended, and the outcome is contentious or inconsistent with the rest of the team’s understanding of the problem and its paired solution. A seemingly innocuous refactor winds up being extremely difficult to apply and is leading to more code with high cognitive load and bugs. **Finishing work shouldn’t be accomplished for the sake of finishing work. If we accept that pull requests aren’t precious, we accept that we make mistakes, and when we accept mistakes we give ourselves feedback on a variety of contexts. **
