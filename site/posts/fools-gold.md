---
title: Fool's Gold, An Introduction
author: Ryan James Spencer
date: 2019-09-22T05:37:09Z
tags: [software architecture, code, fools gold]
summary: >-
  Developer's are a big target for what I call "fool's gold". It's the hope that a
  piece of tech can solve all of our problems that keeps us going with the bait of
  new tech. Solutions tempt despite us knowing better. An experienced software
  developer realizes that everything has strengths and weaknesses which we call
  "tradeoffs", but plenty of developer's don't realise this yet or are in denial.
  This article is an introduction to the concept that plenty software and services
  are sold as panacea but anything sold as panacea should be considered with
  caution.
---

Developer's are a big target for what I call "fool's gold". It's the hope that a
piece of tech can solve all of our problems that keeps us going with the bait of
new tech. Solutions tempt despite us knowing better. An experienced software
developer realizes that _everything_ has strengths and weaknesses which we call
"tradeoffs", but plenty of developer's don't realise this yet or are in denial.
This article is an introduction to the concept that plenty software and services
are sold as panacea but anything sold as panacea should be considered with
caution.

Let us discuss two ends of an argument first. There is the camp of [choosing
boring tech](http://boringtechnology.club/) and [running less
software](https://www.intercom.com/blog/run-less-software/). This camp says that
cognitive load and operational costs are distractions for teams whose primary
focus ought to be the product they are building that makes them profit. By
running less software you are curbing the desire to have a bijective mapping
between problems to solutions where the relations are each their own distinct
solutions. Think about it this way, if you have N many devs and M many distinct
solutions for your problems, you have three particular cases to consider

1. `N > M`: devs can't be experts except for some subset of the total pool of
   technology. Ditto (and more importantly) maintenance and operations. More
   than one dev has to be allocated per tech to make this work (pigeon hole
   principle).
2. `N = M`: every dev can own a particular piece of tech and grow with. Devs may
   get bored and want to congregate on other pieces of tech. If this happens you
   wind up with `N > M`, effectively.
3. `N < M`: Devs can congregate around pieces of tech without much fuss. They
   have freedom to pick what they like most (within a certain degree depending
   on the delta `M-N`). Maintenance and operations is bearable as the whole team
   can participate and not have to spin plates.

Then there is the camp of constant agitation. A company goes under if it's not
constantly pushing to optimise for end users and reducing costs. [Some even
argue you can only pick one of these two optimisations](
https://www.goodreads.com/book/show/28592994-simplify). Enterprises claim to
avoid this complication because their golden goose is sitting pretty, but the
reality is that any revenue generating organisation has to constantly push
themselves into the future to compete. Technology is an enabler, it allows teams
to move faster by automating away toil, easing collaboration friction, and a
productive team means they can, hopefully, deliver user experiences that delight
or at a cost that is beats the competition.

All of this shuffling around comes at the cost of churn and bloat. Companies try
to sooth this issue by either by hiring more devs and/or performing lots of
migrations. [Software rots](https://en.wikipedia.org/wiki/Software_rot), which
can mean different things to different people, but I see it as the eventual
ineffectiveness of a piece of software as improvements are found in competing
solutions. That is to say, even though your software may not actually be getting
slower, it will definitely feel slower in the context of all neighboring
solutions getting faster. This is but one example yet other things like security
exploits, support for a particular version of a language or library, and so on
all work to disempower your application or system.

The reality is that we cannot simply pick one camp to be part of as professional
developers. We are paid to help companies continue to live and be better than
they were before we joined, all within the confines of the ethics and legalities
we are bound to. **We cannot sit still but we can't move too much!** Some have
suggested things like a [novelty
budget](https://www.shimweasel.com/2018/08/25/novelty-budgets) to support
keeping the platform largely stable while pursuing new ways of handling
constantly arising issues.

This article isn't meant to attack particular companies or pieces of tech or
practices despite those practices that carelessly hold on to the past or
endlessly throw it out for the new. The core intent is to encourage critical
thinking and consideration of tradeoffs when problem solving. Weigh your
options! You cannot make decisions purely by reading and watching, nor can you
make a choice by trying everything as there simply isn't the time. Certain devs
learn to do cheap experiments at work or at home but this can have the pitfall
of comparing a toy project a success that may not handle the scale of an
industrial grade application. This sort of scrutiny is part of the weighing
process.

If anyone tells you they're going to make you rich, they're getting rich off of
you. **Take marketing with a grain of salt and throw out the fool's gold!**
