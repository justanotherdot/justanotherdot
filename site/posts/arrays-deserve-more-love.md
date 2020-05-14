---
title: Arrays Deserve More Love
author: Ryan James Spencer
date:
tags:
  -
summary: >-
---

There is often a lot of talk about reducing allocations. Being nervous of clone
might be rationalized, and it might not. You won't know until you start
profiling. Profiling can tell us where the bulk of time is spent and how
resources are being spent across the flow of time. As memory consumption grows,
it may shrinks as well, just like a garbage collector might.

Deciding how much memory to use and bounding it is something programmers dealt
with for a long time. Before operating systems, programs wrote directly to the
memory. When operating systems came into the picture, they posed as programming
interfaces to resources, allowing programs a single, "stable" target.

The beauty of growable data structures is that we only use what we intend to
use, but we pay for growth. This means calls to malloc, copying values over to
new backing stores, freeing data that is no longer needed on the old backing
store, etc.

Taking the array approach means the other direction is at play. We start with an
"arena" of memory.
