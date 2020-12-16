---
title: Shape Systems That Are Easy To Take Apart
author: Ryan James Spencer
date: 2020-12-16T20:19:20.180587367+00:00
tags:
  - systems programming
  - systems
summary: >-
    Systems aren't systems unless the shapes in a system are connected together.
    Often when the shapes of a system are designed, the focus is solely on the
    shapes volume, but neglecting surface area can lead to systems that are
    painful to take apart and clean.
---

In imaging, there's a notion called "separation of tone" or sometimes curtly
referred to as "separation". Images that clearly distinguish objects from one
another possess this quality. Achieving clear tonal separation enables the
author of an image to put more effort into composition of the contained objects.
Separation and composition are also ways to think about system design.

Systems are, usually, different to images in on chief regard; a collection of
shapes remains a collection of shapes until the shapes start to interrelate, at
which stage the collection becomes a system. Unfortunately, when software
engineers go off to implement the various shapes of a system, there is an absurd
focus on the *volume* of the shapes and little effort put into the *surface
area*. The volume may be the code, whereas the surface area is the respective
interface(s) to the code in question.

Focusing on surface area gives you the ability to,

* couple shapes together without deforming them
* reason about a system and its properties
* version shapes and the system that contains them

## Coupling Is A Fundamental Act Of Programming

Despite the 'best practice' advice to keep elements of a system loosely coupled,
the reality is that most of programming is identifying/building components and
connecting them together. You've likely used the term "glue" or "plumbing" for
this reason. When surface areas are large and poorly defined, shapes subject to
coupling turn into giant globs that may be intractable to pull apart. Squishing
together pieces of clay may or may not retain the original lines of separation,
for example. In stark contrast, clearly defined shapes only touch at designated
places, and possess the important ability to *decouple* from one another.

By being able to couple and decouple shapes, we can perform experiments or
optimize segments of a system, change around whole topologies for the purposes
of resilience or to expose the system as a shape with its own surface area, and
so forth. An often overlooked capacity of having clear interfaces is the ability
to constantly ship functionality to production, but in the shadows, only to be
plumbed together when the time calls for it. If the interface is clear, we can
also choose to route traffic or subject the shape to feature flagging logic to
ensure particular backing logic is slowly integrated as an engineer gains
confidence in the implementation.

## Reasoning Tames Complexity

Systems ought to embody properties rather than being piecemeal creations. This
is because we can always judge the state of the system against the properties
and whether or not something is upholding the current properties (a target) or
if it *was* upholding a particular property. Healthy teams assume that prior
actions were done *on purpose*. This mindset avoids the narrative that "everyone
else is constantly making mistakes" as well as continually driving an attitude
that properties guide actions.

Properties can go by many names. The classic nomenclature for interfaces are
usually described by the terminology of preconditions, postconditions, and
invariants. Pre- and postconditions tend to be taken from the notion of
contractual obligations, and invariants are about properties upheld at all
points of execution. All of these are basically **guarantees and expectations**,
which are fundamentally part of the design of a shapes surface area. [Even
unintended but semi- or fully-observable properties of an interface can become
part of the expectations](https://www.hyrumslaw.com/).

## Versioning Supports Stability

Any mutable entity in constant flux is difficult to couple against as the
constant change places work on the dependent to adapt. To combat this, software
engineering has a long history of versioning. Looking at a system as an
immutable stream of points with keys (versions) means dependents can choose
which versions to couple against. Semantic versioning, changelogs, etc. intend
to express the degree of potential effort needed to couple against a particular
version. Clearly defined shapes in a system are subject to this form of snapshot
control.

We tend to think solely of going forward to avoid backwards incompatible
changes, but versioning that tries hard to be both backwards and forwards
compatible supports a form of time traveling for dependents, albeit potentially
only in ranges if there are breaking changes outside of the range. By pinning
compatible pairs of surface areas we reduce the amount of effort spent
constantly upgrading and may reallocate that same effort onto other more
important tasks.

## But That's Not My Remit

There may be someone with a title that specifically entails designing systems,
but in reality they are not the ones with the final say. Each commit of code
into production reflects a final decision about the form of the system and it's
constituent shapes as a whole. **Everything is a system**, from frontends,
backends, tooling, distributed systems, libraries, et. al. Systems are amoebic
in how the authors of a system go about making changes to it, but they need not
be out of control in terms of their growth. **The interface to backing logic is
as much part of the implementation as the logic itself**.
