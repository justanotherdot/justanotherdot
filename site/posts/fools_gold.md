---
title: Fool's Gold, An Introduction
author: Ryan James Spencer
date: 2019-09-19T10:15:00Z
tags: [software architecture, code, fools gold]
---

Developer's are a big target for what I call "fool's gold". It's the hope that a
piece of tech can solve all of our problems that keeps us going with the bait of
new tech. Solutions tempt despite us knowing better. An experienced software
developer realizes that _everything_ has strengths and weaknesses which we call
"tradeoffs", but plenty of developer's don't realise this yet or are in denial.
This article is an introduction to the concept that plenty software and services
are sold as panacea but anything sold as panacea should be considered with
extreme caution.

Let us discuss two ends of an argument first.

There is the camp of [choosing boring tech](http://boringtechnology.club/) and
[running less software](https://www.intercom.com/blog/run-less-software/). This
camp says that cognitive load and operational costs are distractions for teams
whose primary focus ought to be the product they are building that makes them
profit. By running less software you are curbing the desire to have a bijective
mapping between problems to solutions where the relations are each their own
distinct solutions. Think about it this way, if you have N many devs and M many
distinct solutions for your problems, you have three particular cases to
consider

1. `N > M`: devs can't be experts except for some subset of the total pool of
   technology. Ditto (and more importantly) maintenance and operations. More
   than one dev has to be allocated per tech to make this work (pigeon
   hole principle).
2. `N = M`: every dev can own a particular piece of tech and grow with. Devs may
   get bored and want to congregate on other pieces of tech.
3. `N < M`: Devs can congregate around pieces of tech. They have freedom to pick
   what they like most (within a certain degree depending on the delta `M-N`.
   Maintenance and operations is bearable as the whole team can participate and
   not have to spin plates.

Then there is the camp of constant agitation. A company goes under if it's not
constantly pushing to optimise for end users and reducing costs. Enterprises
claim to avoid this complication because their golden goose is sitting pretty,
but the reality is that any revenue generating organisation has to constantly
push themselves into the future to compete. Technology is an enabler, it allows
teams to move faster, collaborate more quickly, automations to alleviate toil,
user experiences to be more delightful, and so on. The camp of constant
agitation is seeks perpetually test out new programming languages, cloud
provider packaged functionality, libraries, and so forth in the hope that it
helps said company gain a competitive advantage.

All of this shuffling around _does_ provide advantages to companies but at the
cost of churn and bloat. This is generally combated either by hiring more devs
and/or performing lots of migrations. [Software
rots](https://en.wikipedia.org/wiki/Software_rot), which can different things to
different people, but I see it as the eventual ineffectiveness of a piece of
software as improvements are found in competing solutions. That is to say, even
though your software may not actually be getting slower, it will definitely feel
slower in the context of all the other software getting faster. This is but one
example but other things like security exploits, support for a particular
version of a language or library, and so on.

The reality is that we cannot simply pick one camp to be part of as professional
developers. We are paid to help organisations continue to live and be better
than they were before we joined, all within the confines of ethics and the
legalities we are bound to. We cannot sit still but we can't move too much! Some
have suggested things like a [novelty
budget](https://www.shimweasel.com/2018/08/25/novelty-budgets) to support
keeping the platform largely stable

The theme of this, and future, articles is not to attack particular companies or
pieces of tech or practices. The intent is to encourage critical thinking and
consideration of tradeoffs. Weigh your options! You cannot make decisions
entirely in a vacuum, e.g. purely by reading, nor can you make a choice by
trying everything out as there simply isn't the time. It's totally fine to try
things out on your spare time or work on the skill of doing cheap experiments.
Sometimes experiments don't tell you much because they relate to a toy versus a
large-scale application, but this is part of the weighing process. If anyone
tells you they're going to make you rich, they're getting rich off of you. Take
marketing with a grain of salt and throw out the fool's gold!
