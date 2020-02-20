---
title: Errors Across a Boundary
author: Ryan James Spencer
date: 2019-09-27T11:10:01.917084567+00:00
tags: [error handling, code]
summary: >-
  There is a line across our systems we shall call the boundary. On one end of the
  boundary are the consumers and on the other side are the providers. This
  boundary is what we are accustomed to calling an interface. Interfaces are the
  embodiment of the dance needed to cross the boundary. The interface may have
  adapters on either side whose purpose is to munge details of the internals into
  this known language of communication. This way internals can continue working
  without the fuss of the protocol driving their decisions.
---

There is a line across our systems we shall call the boundary. On one end of the
boundary are the consumers and on the other side are the providers. This
boundary is what we are accustomed to calling an interface. Interfaces are the
embodiment of the dance needed to cross the boundary. The interface may have
adapters on either side whose purpose is to munge details of the internals into
this known language of communication. This way internals can continue working
without the fuss of the protocol driving their decisions.

Things go wrong. But when they do developers tend to clump everything up as a
single form of error. Errors are about reporting mistakes or complications. A
better name compiler writers have been using for years is 'diagnostics'; they
should help diagnose a particular problem by being part of the symptoms an
ill-behaving service might demonstrate. **As such, when an infraction occurs you
want to know who is the offending party. _Are we holding it wrong or are you?_**

Borders on your errors make clarify what the fix is by knowing who should be
performing the correction. This might mean the speech of diagnostics changes. A
person hacking on some code is much more accustomed to cryptic messages from a
compiler than an average person using a web interface to access their bank who
doesn't understand how any of this is rigged up.

Clearly delineate your errors and you'll know better if something is a mistake
or a matter of environment, if it is something a maintainer needs to worry about
or a blunder from usage. There are many styles to error handling but this
approach does not impact which style you end up using. You can use this whether
it is error codes, exceptions, or values. This is a matter of organisation.

Systems architecture itself can largely be seen as a form of organisation.
Conceptual partitions, domain concerns, and our pursuit for pieces that compose
drive this organisation. Independent the layer of granularity the focus on
organisatoin is always the same. **When things go wrong, tell everyone which
side of the fence the mistake or complication came from.**
