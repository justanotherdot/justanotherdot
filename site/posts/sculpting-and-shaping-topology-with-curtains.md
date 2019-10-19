---
title: Sculpting and Shaping Topology With Curtains
author: Ryan James Spencer
date: 2019-10-19T11:29:55.346897423+00:00
tags: [principle, pattern]
---

Once upon a time I studied photography. It was there that the importance of
separation between tones in an image, of shade or color, was hammered into me.
If the separation in my image wasn't right I'd have to redo all my work in order
to get a grade. [Separation is how we define our mental map of the
world.](https://en.wikipedia.org/wiki/Gestalt_psychology) I'll call anything
that is distinct from other things an 'entity'.

Entities have edges. They may have flows of communication out and into these
edges. Edges may be incidental, e.g. defined by others or from natural
consequences, and they may be intentional, i.e. the result of deliberate
planning and execution. Entities have non-zero surface area, otherwise they
wouldn't exist. Notice how I use the idea of surface area rather than visiblity:
glass has a surface area despite not being able to see it directly (bar
reflections).

Incidental edges can become intentional edges if we have control or influence
over the topology of the ecosystem in question. Part of planning how edges are
defined, transforming incidental to intentional or from nothing, is
organization; the big strokes. I will call this process 'clarification'.

Clarifying (edges) means simpler mental maps. Simpler mental maps means easier
to reason about systems and programs. Simpler systems and programs means
increased velocity for progress and experimentation.

Examples of clarifications:

1. Serialization to the wire (network) goes in its own module. The rule is that
   it is only used in the interface layer of your service.

2. Serialization to disk goes in _its_ own module and is used only for writing
   out to disk, not for the wire!

3. The interface layer is distinct from the core of the application. A CLI tool
   has a `main` module that is its entrypoint, but the core of the program lives
   in a `core` module outside of that space.

4. Per (3) above, we may worry that interface layers may be too intertwined with
   core internals, so we have a munging layer (some call this an adapter) that
   transforms data to the shape you so desire. This is bidirectional; it's
   equally fair to have the adapting layer work on outbound and inbound
   interfaces.

It is fine to accept some mess early on in a project. In fact, some projects are
like [simulated annealing](https://en.wikipedia.org/wiki/Simulated_annealing):
early on we take many risks but with time we slow down, the risks decrease, the
stability intensifies. We are (hopefully) more likely to be in an ideal place.

There is the general thought that programming in a professional context, or even
in a personal context if one is so inclined, _has_ to be related to this
balancing act of progress (by accepting breakage) to stability (by leaving
things alone). [I've talked a bit before about how vital constant
experimentation
is](https://www.justanotherdot.com/posts/may-you-be-the-author-of-two-to-the-n-programs.html),
but this balancing act is not the only way to go about things.

It is entirely possible to make progress but retain stability.

Although it may seem strange for me to use the term _artificial_ when all the
boundaries discussed here so far seem planned by ourselves or by others, I use
the term to denote delineations we establish to avoid working in _slices_. A
sliced approach to development means we attempt to get all working
functionality, from front to back, one slice at a time. In the following
diagram, the red boxes are slices of features whereas non-sliced functionality
is stable:

![a diagram depicting 'sliced'
development](assets/images/sliced-development-example.png "An example of
'sliced' development")

Alternatively, one can setup an artificial curtain to retain stability in all
edges exterior to it. Setting up a curtain can be done with [feature-flags,
parallel
implementations](https://www.justanotherdot.com/posts/move-fast-and-tuck-code-into-the-shadows.html)
or even creating new surfaces where interaction will be performed and doing a
migration to retain stability. Per the example above it might look like this:

![a diagram depicting 'curtained'
development](assets/images/curtained-development-example.png "An example of
'curtained' development")

There is always an implicit countdown when you keep things stable but don't make
progress. "Where is the business value they are adding?" squawks the manager. If
you are making a lot of progress but breaking things the countdown timer is time
to completion but in the face of churn, top-down pressure, peer-pressure, and so
on.

If you code in this manner it buys you a lot of breathing room. **Breathing room
gives you space to think. Space to think means you can buy yourself breathing
room. This helps build healthy systems with reduced complexity, without having
to sacrifice progress.**
