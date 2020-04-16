---
title: A Release Does Not Make a Deploy
author: Ryan James Spencer
date: 2019-11-24T09:21:53.272218419+00:00
tags: [infrastructure]
summary: >-
  Is the vision in your head of your pipelines that of lean, graceful atheletes?
  Do branch builds simply test your changes swiftly and anything that hits master
  builds artifacts finished with the flourish of a ephemeral "deployment"?
---

Is the vision in your head of your pipelines that of lean, graceful atheletes?
Do branch builds simply test your changes swiftly and anything that hits master
builds artifacts finished with the flourish of a ephemeral "deployment"?

Your pipelines are overweight slobs, unwilling to truly do real work.

Conventional wisdom dictates that deployments occur at the ends of pipelines by
running a simple task, say `kubectl apply` or similar, with the produced
artifact mentioned. This act is transient and for many pipelines means rolling
back is an act of rerunning the whole pipeline, an individual step in the
pipeline, or even reverse-engineering the action in the deployment step and
performing it manually, given the level of desperation.

**Build artifacts aren't deployments.** By turning deployments from "transient
action" into their own artifact you can scrobble across deployments with little
fuss. A deployment artifact can be anything that describes the act of deploying.
This might be a script, a set of versions packaged together, or even a
specification like a kubernetes manifest. **Once you have release artifacts _and_
deployment artifacts start the exercise regime for your pipelines by building
and publishing all the things**.

Won't all this extra work cost more money and time? The reality is that
amortizing the cost of storing your artifacts and building whenever you get a
chance helps provide options so you don't have to do extra work when it is the
most untimely to do so. What costs more? Having a terrible
mean-time-to-resolution (MTRR) and frequent outages or paying for more build
bots and storage space? If you haven't learned the cost of burning the trust of
your end users, then you have an important lesson to learn.

Scrobbling deployments not only helps reduce the blast radius of botched code
hitting the pool of production by increasing your MTRR but it also gives you the
opportunity for functionality such as preview deployments. Some approaches may
provide previews in different deployment environments entirely whereas others
allow service or resource "naming" (e.g. unique URLs or distinct IP addresses)
to route traffic accordingly. Some blend the two together. This last approach is
often how services such as [zeit](https://zeit.co/) and
[linc.sh](https://linc.sh/) do their previews for branch builds. It depends on
how much reproducibility you care about to get a sanity check before deploying
to production.

The one wrench in all of this is the matter of shared state; sometimes the
complication of going backwards or forwards from a certain deployment involves
running or reversing migrations, reinstating or removing coupled infrastructural
changes, or even having third party services paid and available. There are
islands of deployments that may become totally inaccessible due to the above and
the best advice I can provide in the briefest period of time is that all of
these can be (somewhat) circumvented by ensuring nothing exists that isn't code.
To address the noted issues (which is incomplete, mind you):

* Having a process that ensures all migratory actions on a database are verified
  to revert properly and that snapshots are regularly taken at frequent
  intervals
* All infrastructure is code so rolling back infrastructural changes isn't a
  matter of someone GUI-poking or frantically performing manual changes
* Providing configuration and testing that ensures a system behaves as it needs
  to behave without the reliance on third-party software and services

**[Fear not the Friday
deploy](https://charity.wtf/2019/10/28/deploys-its-not-actually-about-fridays/)
when you have options at hand; fear the duct-tape and popsicle-stick
infrastructure that makes Friday deploys a nightmare.**
