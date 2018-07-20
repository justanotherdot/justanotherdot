---
title: What makes a good pull request?
author: Ryan James Spencer
date: Fri 20 Jul 20:40:08 AEST 2018
tags: [software, git]
---

> Given enough eyeballs, all bugs are shallow. -Linus' Law

Pull Requests (or PRs) are a tango between two parties; the code author and the
code reviewer which I will simply refer to as the 'author' and 'reviewer' in the
remainder of this article. In a pull request, the author has provided code to
solve a particular problem and the reviewer is there to provide a feedback
mechanism to the author.

## Code review is not a gate keeping task

I once worked for an organisation whose code review process centered around a
total lack of faith in its developers ability to deliver quality product. This
meant the process was utterly broken as it meant tech leads were the gate
keepers of their respective stacks and that the average developer would have to
resort to underhanded tactics in order to get their changes in, regardless of
quality or prospective bugs. Unfortunately, this also meant that the gate
keepers felt more justified for keeping things safe; a vicious cycle indeed!

As a software engineer, every line of code you ship is code you, or someone
else, will need to maintain, and as such, you should be fighting to deliver the
best quality you can offer, regardless of deadline. **Code reviewers are there
to help people bring their code into the light of day where asking questions is
the chief tool a reviewer employs**. This can include, but is not limited to,
probing to see if:

* A thought is fully fleshed out
* This implementation is the right one to make or is solving the right problem
* There are changes that are principally unsound or dangerous
* Any other general performance or semantic concerns might be present

I'm purposefully leaving out stylistic choices here as the discussion often
leads to the argument around adoption of some automated code-formatting tool.

## Raise early and raise often

In "Debugging Team's", the authors kick off the start of the book with a simple
analogy between two competing inventors. The one inventor does not want to share
his ideas for fear of them being poached by others, while the rival inventor
gleefully goes to local places where experts might hang out to get more
information about how to build her inventions.

Software development is no different in that knowing things earlier is always
better than knowing things later. Raising PRs even before they are 'complete'
(and appropriately marking them as WIPs or 'works in progress') allows people to
possibly, time permitting, look into your changes and see if there are any major
red flags.

That said, if you are a reviewer and are asked (or not!) to look at a set of
changes that is marked as a WIP, try to hold off on a more in-depth review until
the author changes this status. And to PR raisers, don't keep things in WIP
stage for too long, which brings me to my next point.

## PRs are for small chunks of code to merge often

A PR should represent about a days worth of work. This is beneficial to both
parties in that it facilitates a 'merge often' approach for devs (and devs get
the little adrenaline kick from CI and approvals going green and clicking
`merge`) and reviewers can much more easily review a smaller hunk of changes. A
reviewer reviewing five PRs in the course of a week has to spend less time
grokking those individual changes than to review five days worth of work in a
single PR.

Massive projects poised around scaled tooling and reviews such as the Linux
kernel would flat out reject a patch with, say, 3k additions and 1k removals. If
a project of that size and calibre, and that many international hands involved,
is marking code review of those dimensions as 'unmanageable', what hope does a
startup have at making fast, rapid changes in the same light?

Small PRs are also _focused_ on a clear intent. Asking the author to fix
neighboring code 'just because' or refactoring/formatting several adjacent files
that are not directly tied to the immediate effort of the PR wastes the both
parties time. Opening a PR to refactor changes and another PR to add new
functionality is a much better way to get appropriate attention from reviewers.
As the joke goes:

> 10 lines = 10 possible bugs
> 100 lines = lgtm

It's important to remember that a PR is not to encompass a single ticket or
issue. Ticket's can have several PRs attached to them and all it takes is
lobbing `[FOO-123]` on top of one's PR title for Jira or marking `#<issue
number` in your description in GitHub. I like to call this act 'linking' and
it's useful for stakeholders to track down all the changes that have fed into a
particular ticket.

## Context matters

Reviewers need to discuss with the author about the purpose of a set of changes
and how close or far off they are from that goal, but if the reviewer is unclear
about this goal, it's difficult for them to strike up an insightful discussion
with the author.

In the context of OSS, raising a patch directly to a project such as the Linux
kernel is poor practice. If you want to make a change in any capacity it's best
first to contact the people who own the code on public channels. This provides
auditing and clear context for others. The same is true for many public
platforms for hosting code: if you want to make a change it's poor form to
simply lodge a PR proposing your new feature or 'fix'.

That said, raising PRs should feel natural; PRs are cheap and can be closed and
their branches pruned as need be, but regardless of the cost of raising a PR,
it's critical to include appropriate information. Some important things to
mention may be:

* What does this set of changes solve?
* Is there a specific task (issue/ticket) that this relates to?
* Is this blocked or blocking any other PRs/issues/tickets?
* Is there any additional information that will help the reviewer know about my
  manual testing of this ticket (screenshots, output from tooling, et. al.)?
* Have you updated tests and documentation accordingly? Have you added tests
  that the reviewer can skip to first to immediately see how you're proposed
  changes are supposed to work and in what cases?
* Is there current behaviour to contrast the new behaviour to?
* Are there breaking changes present?

The traditional approach for this was to include commentary in your actual
commit messages and headers. I don't think times have really changed in this
regard and the more context you sprinkle about the better, so long as you are
clear about your intent and you don't waste the readers time.

## Check out changes locally when it makes sense

A really healthy habit for reasonably sized changes is to always check out a PR
and see if it works for you. CI should always be there as an arbiter of
regressions. Other projects have really complicated processes for testing full
'e2e/raw hardware' tests like this like Intel's [zero-day testing
bot](https://01.org/lkp/documentation/0-day-test-service) which actually boots
up machines to test out differing version of the kernel. While CI and automated
testing and building are powerful, it's important to sometimes get a human eye
on things to verify things such as faulty tests. We all make mistakes, and
reviewers are there to give sanity checks to our changes.

Use common sense. If a change is pretty sensible (e.g. a single line change to
upate a variable name), you probably don't need to spend the time pulling the
change down, compiling, running the tests, etc. The developers mantra is "don't
be the machine" and nothing is different here! If CI has done more work than you
or you're repeating stuff CI is doing, you better have a good reason to mimic
the machine.

## Clean up your mess

Your changes have been merged and you can go on with your life, but before you
reach for beverage of choice, you should prune your dead branches. I actually
have a git bash script for this that I place in my `PATH` so I can call it as
`git wash`. The script is [here](
https://gist.github.com/justanotherdot/3e3a16df805d09a37e1c26bbedd23fcc). When
run without arguments, this will delete the current branch you are on locally
and remotely so long as the branch specified to `git wash` is not master and the
remote branch is not protected. If you need other git functionality like this,
any script in your path with the name `git-<thing>` can be run as `git <thing>`.
