---
title: On breathing room
author: Ryan James Spencer
date: 2022-03-06T00:03:22.391830659+00:00
tags:
  - software development
summary: >-
  Change is inevitable, but need not be unsafe.
---

You need to make changes to produce new states of the system you expose. Code
change velocity is a metric people sometimes use to determine health of the
platform. While a high change rate may seem good, it can also be a sign of a lot
of churn. Change is inevitable, but need not be unsafe. To avoid releasing code
you regret, try these approaches to making changes in the future.

At a macro level, any emergent change to the system consists of three fundamental phases:

1. thinking, specification, communication, problem decomposition and solution synthesis
2. execution, cycle of design and implementation, discussion around issues where the rubber meets the road, preferably working in a way that avoids cleanup in the next step (“baggage”)
3. cleanup, tidying, ensuring work doesn't need revisions or bug fixes in the future,  *preferably only*ceremony around things being done such as communication and demoing

The goal of these three phases is to produce work that is itself [a single stepping stone](https://medium.com/@jamesacowling/stepping-stones-not-milestones-e6be0073563f), allowing steady progress. Phases (1) and (3) can be thought of as "bookends", allowing you to gain traction and wind down ensuring work is both thorough and complete by the time you leave it.

Making changes should support *breathing room* where you feel comfortable making the switch from the old state of the system to the new one.

## Immutable Changes: The Default You Need

Not all changes are backwards compatible, but most can be. When you work to push backwards compatibility as a principle of development, you gain the ability to easily move forward with new work without consistently carrying knowledge of the old system along with you. how is that possible when you are adhering to an older form of the system? In an immutable view of change, the new changes don’t have to be immediately, or ever, touching the old system. compatibility is contained naturally, without even holding in the thoughts of “how do i make this work with what is there?”. sometimes the thought must be there, especially when you are making changes mutably (more on this later), in general, with immutable changes we can focus instead on making the boundary compatible, rather than the union of all changes.

A rough sketch of what this looks like is:

1. Make a new target in isolation, lots of testing and verification
2. Provide means to make changes reversible, such as feature flagging, rapid deployments by decoupling releases as artifacts and deployment as the action.
3. Make the switch and observe

Mental model: you are going from previous changes to new changes and allow *decisions to be reversible*. Some changes are not going to be reversible and that's ok! But with a framework in place to allow reversal, knowing when something can or cannot be reversed gets easier.

This system promotes flexibility. Want to try out three different rendering engines for performance? Treat the immutable approach as what others might call "growth" or experiment based. Remember: the stable state of the system is when all the flags are off, or, put another way, a flag should never be required on to make the system stable. That way if you lose the flag service, you still have the stable state of the system, hence releasing something eventually means pruning the need for the flag.

You also don't need to lean heavily on flags. The new change could be pushed up “into the shadows” without any rigging, and the independent work of connecting everything can be done as it’s own switched behavior. Isolating surface area in this way is a great alternative to what else is possible, which is that you may want to run one or two adjacent implementations of large or whole parts of the system in parallel. [These could just as easily be forks of the codebase, where one split implementation is gong to win out.](http://www.sevangelatos.com/john-carmack-on-parallel-implementations/) The problem with these massive changes is that people tend to prefer small, sectional changes to a codebase, thus the forks would continually need to absorb new changes as they come available, and deal with any shortcomings of the newly introduced code. Hence it is a good idea to focus forks on short-lived experiments, and why I feel immutable changes should be a default, but they may not be preferable given the sheer complexity of making the switch, therefore we sometimes do want to keep mutable changes in place where it is less error prone, either due to drastically reduced code or complexity involved with switching; anytime you see deeply embedded flags or _lots_ of usage for the same flag, you know that the changeover is going to involve a wide surface area that, although may be great to get the benefits of reversibility, may also mean it is harder, and longer, to get into place in general. As a guide, anytime things grow to a particular size, either in time or space, there is a tendency for that growth to continue rather than have a definite stop.

## Mutable Changes: Occasionally Avoiding Increased Complexity

Occasionally changes are so massive that we don't want to become a blocker for other's making changes on the same system. When that happens it may be preferable to work on a mutable change, but this doesn't mean introducing a massive PR. For every new line introduced in a PR is another line of code to have to worry about causing more trouble in production. Instead, with mutable changes, there is nothing wrong with raising a massive PR, so long as we harvest it for smaller, atomic changes we can introduce incrementally. These incremental changes, along with a good release system, allow us to mitigate failures and contain them to specific places. Five individual changes affecting different areas is more likely to have one change reversed as opposed to a change that affects all five changes simultaneously.

This process is noticeably more involved than the above, but you will see it is mostly a multi-pass process. In general:

1. Raise the massive PR that has all the changes holistically displayed
2. Identify all the add-only changes in the PR: these can be added in the shadows without having anything rigged up, similar to our immutable approach above.
3. Make precise changes to prior interfaces and running logic in pieces. Accompany these with 'migration' tests that may or may not be permanent but nonetheless support hardening the platform by baking expected logic into place. When you introduce the change, the tests ought to verify that the old behavior / properties are upheld.
4. Start working on glue logic rigging up your new code to work. Sometimes steps (2), (3), and (4) have to be paired to ensure the system stays stable, and that's fine. We are aiming to keep changes minimal and atomic.
5. At this point, your new code is in, the prior parts of the system have shifted over to their new shape(s), and old code can be deleted, but you needn't rush to do this. In the same way adding codes into the shadows lets you decide when you want to make it public, unused code is simply harmless and living in the dank recesses of the shadows now, one day waiting to be swept up. If you are worried others may end up accidentally using it, add warnings, errors, or commentary to the code making it effectively useless, or just delete it on the spot. If you decide to delete on the spot, you may have to do a bit more work to bring it back into existence, rather than simply flipping out the glue change back to the way the old world was.

I detailed this process a bit more with the article [Harvesting Pull Requests](https://www.justanotherdot.com/posts/harvesting-pull-requests.html). I likely ought to do the same thing with a deeper dive on the immutable process above, as there are nuances and characteristics to it that intertwine with the problem at hand and what one has available to them to make shipping things safer.

## How do you decide which to pick?

Whether to enact change immutably or mutably is a matter of deciding what will help you avoid sharp corners. We want breathing room when we make changes. Breathing room is where you don’t release a change with bated breath, crossing your fingers for the next hours, days, weeks, hoping that it won’t simply blow up and be a fiasco for everyone. Breathing room means that *when* it blows up it will be easy to fix without having to rush at the ready, dropping everything, to perform a hotfix on the system. Hotfixes suck. They may sometimes be required to resolve an incident, but they will not help your mean time to resolution. If you can go back to a good known state in less than a minute with a good deployment system in place, and the changes you’ve imposed are backwards compatible, you will allow you and others on call or responsible to feel a bit less pressure, which in turn gives you more freedom to keep progressing towards your the goals that you *want* to be dealing with rather than the goals you *have* to be dealing with.
