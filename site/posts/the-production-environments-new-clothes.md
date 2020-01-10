---
title: The Production Environment's New Clothes
author: Ryan James Spencer
date:
tags:
  - software teams
  - feature flagging
  - feature flags
---

Staging environments are a distraction. Massive hours have been poured into
making them a coherent with production all to little effect. When staging
environments become unbearable developers start resorting to alternative
environments that either can be spun up at will or are equally long-lived as
staging environments are. These production clones feel safe to developers and
product managers because they make them feel safe as changes pushed to them
aren't shown to customers.

**Production doesn't mean seen.**

[I've written about this
before](https://www.justanotherdot.com/posts/move-fast-and-tuck-code-into-the-shadows.html)
but received some confused responses and I think I've realised why people feel
uncomfortable about this concept. Feature flagging is a simple enough idea.
There are third party services out there but you can easily write your own
feature flagging system to hide away details although there are some caveats
I'll discuss in feature flagging system construction. I have long been a
proponent of small pull requests; small changes give large boosts of energy,
helping progress and leading to the eventual discovery that you've built a
mountain when it has not felt like you've built a mountain Developers who feel
safe pushing changes start pushing a lot of changes, hence, I think having a
great feature flagging system is pivotal to making the small pull requests
approach feasible in a team.

It is an uncomfortable idea for most devs. There is a natural pain associated
with pushing bad code. Some don't care, while others are scarred and frantically
plan, specify, and test their code before it hits production. You don't need to
entirely stop doing this, but you do need to accept that production ought to be
your own environment.

A basic feature flagging system is a key-value store for named tags and
booleans. Non-existent tags are always false to avoid oddities. The steady state
of the system should be when all flags are off. Flags should persist across the
whole of the architecture to reduce mismatch and bloat. Flags should be
persisted to long term to be robust in the case of failures.

Fancier feature flagging systems support things like traffic routing and mutual
exclusion. For example, a user may be randomly assigned to a split in an [A/B
test](https://en.wikipedia.org/wiki/A/B_testing) and that particular flag they
were assigned may be incompatible with other flags. This isn't quite needed out
of the box unless your platform is already quite messy or your hitting load
concerns. A great many developer know feature flagging entirely for the purposes
of A/B testing but, as noted, feature flagging helps increase confidence in what
you merge into trunk.

Buggy code or migrations can poison a production database. If you are not
already taking regular snapshots of your database, fix that! Trying to prescribe
solutions for various use cases could easily fill other articles, but I will say
that despite it seeming scary that you are mixing feature-flagged code and
steady-state code that both touch the same shared state, with some forethought
it is far easier to curate one pool of data. If you can get back to a good known
state, you can work towards a granularity of restoration that suits your
products needs. A fantastic book on operations around databases that goes in
much greater depth is [Database Reliability
Engineering](https://www.goodreads.com/en/book/show/36523657-database-reliability-engineering).

Tying this all together, you should treat features as immutable migrations. An
immutable migration is one that doesn't happen in place, such that if I have
state A and want to be in state B, I first create state B and transition over. A
mutable migration is the one most people are familiar with, changing a piece of
pre-existing code, testing it locally and in a staging environment or similar,
perhaps even prod, and hoping for the best. With immutable migrations and
feature flags, you can push the code progressively to prod and test at-will. I'm
a big fan of pushing code to production that doesn't isn't being used _yet_.
Doing this a lot and using namespacing on a lexical level, e.g. with function or
module names. When I wrote about this before I mentioned the idea of breathing
room and immutable migrations give you jus that. In fact, they ought to be your
default form of migration unless you are certain doing an in-place change is
going to make things quicker and be relatively pain-free.

Less is more; have one environment you put all your effort and love into. Hide
things from your customers until you are confident of your release. Make
immutable migrations the default instead of risky (albeit faster) in-place
migrations. Put some thought in what you need to do to protect your shared
state. With this your deployments will get more fearless and frequent as well as
your changes smaller and easier to reason about.
