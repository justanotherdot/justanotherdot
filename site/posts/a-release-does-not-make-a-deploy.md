---
title: A Release Does Not Make a Deploy
author: Ryan James Spencer
date: 2019-11-24T09:21:53.272218419+00:00
tags: [ci, continuous integration, releases, deploys, artifacts]
---

Is the vision in your head of your pipelines that of lean, graceful atheletes?
Branch builds simply test your changes swiftly and anything that hits master
builds artifacts finishes by casting off a transient "deployment".

Your pipelines are overweight slobs, unwilling to truly do real work.

Conventional wisdom dictates that deployments occur at the ends of pipelines by
running a simple task, say `kubectl apply` or similar, with the produced
artifact mentioned. This act is transient and for many pipelines means rolling
back is an act of rerunning the whole pipeline, an individual step in the
pipeline, or even reverse-engineering the action in the deployment step and
performing it manually, given level of desperation.

**Build artifacts aren't deployments.** By turning deployments from "transient
action" into their own artifact you can scrobble across deployments with little
fuss. A deployment artifact can be anything that describes the act of deploying.
This might be a script, a set of versoins packaged together, or even a
specification like a kubernetes manifest. Once you have release artifacts and
deployment artifacts start the exercise regime for your pipelines by building
and publishing all the things.

Won't all this extra work cost more money and time? The reality is that
amortizing the cost of storing your artifacts and building whenever you get a
chance helps provide options so you don't have to do extra work when it is the
most untimely to do so. What costs more? Having a terrible
mean-time-to-resolution (MTRR) and frequent outages or paying for more build
bots and storage space?

Scrobbling deployments not only helps reduce the blast radius of botched code
hitting the pool of production by increasing your MTRR but it also gives you
flexibility for functionality such as previews. Some approaches may provide
previews in different deployment environments entirely, whereas other technology
allows simple service and URL naming to route traffic accordingly, such as with
single page applications. This last approach is often how services such as
[`zeit`](https://zeit.co/) and [`linc.sh`](https://linc.sh/) do their previews
on branches.

The one wrench in all of this is the matter of shared state; sometimes the
complication of going backwards or forwards from a certain deployment involves
running or reversing migrations, reinstating or removing coupled infrastructural
changes, or even having third party services paid and available. There are
islands of deployments that may become totally inaccessible due to the above or
more happenstances and the best advice I can provide in the briefest period of time
is that all of these can be (somewhat) circumvented by ensuring nothing exists
that isn't code. To address the noted issues (which is incomplete, mind you):

* Having a process that ensures all migratory actions on a database are verified
  to revert properly and that snapshots are regularly taken at frequent
  intervals
* All infrastructure is code so rolling back infrastructural changes isn't a
  matter of someone GUI-poking or frantically performing manual changes
* Providing configuration and testing that ensures a system behaves as it needs
  to be behave without the reliance on third-party software and services

[Fear not the Friday
deploy](https://charity.wtf/2019/10/28/deploys-its-not-actually-about-fridays/);
fear the duct-tape and popsicle-stick infrastructure that makes Friday deploys a
nightmare. Start first by turning your pipelines into clean, efficient assembly
lines fit for robotic factories, not back-room drug packaging facilities fit for
movies and slums.
