---
title: Harvesting Pull Requests
author: Ryan James Spencer
date: 2022-02-24T04:21:52.914129798+00:00
tags:
  - delivery
summary: >-
  Commonly the ideal way to work with code changes is to do them immutably.
  Sometimes changing code in-place is easier, but it can easily lead to hard to
  reason about patches. Here are some ways to avoid the sharp corners of making
  changes on existing code.
---

Commonly the ideal way to work with code changes is to do them immutably. What i mean by this is that you

1. begin the work as an independent thing
2. provide a way to switch from the old thing to the new thing (feature flag, fork, and so on)
3. switch to the thing under particular conditions (sampling users, explicit flag curation, other rules)

the point of (2) is that it gives you the ability to make a decision reversible, so long as the code is backwards compatible. backwards compatibility gets a bad rap, but the reality is that by working in a backwards compatible way, you allow yourself to take that step back if you need it. if you’ve also separated deploys and releases in your infrastructure, deploying a release becomes yet another mechanism for you to go back in time. notice the difference here where we are saying we can return to a prior, good known state, rather than needing to deploy new, future ‘hot-fixed’ code. any system that can be arbitrarily restarted or can time-travel at will gives you a great deal of certainty around its stability running in production.

however, not all changes are easy to write in this way. while building the new thing and switching to it is ideal, it is not always feasible or practical. sometimes the new thing is going to have a lot of overlapping behavior that also needs changing. perhaps the way things were initially designed, either intentionally or unintentionally, may cause boundaries to be hazy and implementations to be tangled together. other times things are quite well defined, but the change is menial enough that changing something in place is far faster. being backwards compatible here is still feasible, but the emphasis is on reducing time to delivery to production by simplifying the change in question.

however, doing things in-place tends to come with consequences. there is a common tendency to continually grow out a pull request that seeks to patch trunk by changing behavior or properties of the system as-is; adding, modifying, removing all conjoin in such a way that the holistic pull request works precariously with all the changes involved, but experience shows us bigger pull requests (more lines of code changed) means more risk. whenever i find myself with this type of pull request on my hands, i try to attack it methodically by harvesting it down repeatedly into safer, easier to reason about patches.

firstly, identify all the changes that are absolutely safe to add. this means finding all the additions that can sit in the shadows. tests can be initially marked as skipped, modules can live without being used (potentially incurring warnings, but this is only a temporary measure). this change does not impact the stable state of the system in any way, and is safe to put in and think alone.

next, identify all the changes for removals. remove as much as possible that is authentically safe to remove such that the stable state of the current system remains fine. these removals then can declutter the mental space of suspicion on review of the upcoming pull request(s) that involve modifying the existing code.

any type of refactoring or moving things around should probably be done independently now before the final step. factoring code will mean the ultimate modifications we perform can likely be easier to test and measured, but it also, again, means we can reason about factoring and modifications to the existing behavior as separate issues. factoring code means changing the way it is organized and structured without impacting the current behavior of the system.

and now, the part that is left; with all the remaining parts above done, the final pull request(s) are the ones transitioning the system from the old state it was in into the new state. one can think of this as the emergent state of the system where all the states of our system are but immutable nodes in a big state machine. once more, if the change is non-breaking, then we are safe to rollback if we want to, hence it makes some sense for us to make this a solitary change to avoid having to roll back many patches at once, or have others changes intermixed with our deployment. that said, it may make sense to break the changes up into smaller changes that incrementally transition the system to the newer system, but beware; this approach is fragile. it is not one that is going to give you easy understanding of where things went wrong unless you have a system instrumented for observability and can easily correlate a specific deploy and the outcomes, or if you use a canary with minimal traffic to ascertain problems early. problems aren’t always provoked early on, either. it can take time for a bug to manifest, and it makes sense to consider the clump of changes as still a group that should be considered related, even if the intent is to have finer grain understanding of what went wrong and only roll back the problem behavior (while the rest may be fine). in my opinion, if the goal is to avoid rework, it is better to think of distinct pieces that makes sense as a unit, rather than a bunch of broken up pieces that one must stitch back together to understand their interconnection.

but isn’t breaking up what we did with those initial steps, adding, removing, and factoring before we did our final “risky” change? yes, we did break it up, but we were methodical in sieving out all of the authentically unrelated changes to the primary work. in the end we were left with a patch in our hands that we could demonstrate to others is doing the exact change from the old state of the system to the new state, whereas all of the unrelated changes may be related superficially, they also make review of the bigger pull request harder as they may (quite easily) mask bugs.

or, as the classic addage goes. 10 lines, 10 bugs. 1000 lines, lgtm.
