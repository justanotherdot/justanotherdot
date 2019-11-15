---
title: Teleporting At The Speed Of Thought
author: Ryan James Spencer
date: 2019-11-15T09:25:52.666511749+00:00
tags:
  - editor
  - code
  - principle
  - vim
  - nvim
  - vscode
  - emacs
---

Adept text editor users fly around and manipulate text as if by _teleportation_.
For me, this is a principle I hold dear when considering my editing experience.
**Teleportation is chiefly driven by thought and is effortless by
construction**. This is actually not something inherent to teleportaiton.
Instead of hopping in a car and driving around the neighbourhood to find where
you want to go, you tend to make a decision about your destination ahead of
time. However, driving expects a certain amount of effort to reach a destination
whereas teleportation requires little effort if at all. Teleportation doesn't
just mean _jumping_ someplace but also transporting text somewhere, whether it
be someplace else in a buffer or into textual purgatory.

Some call this "code golf" but using that term implies falling into the trap of
constantly optimizing when the aim is to [carve up the text in front of
you](https://www.justanotherdot.com/posts/how-fast-can-you-take-your-time-kid.html).
As such, optimizing for teleportation comes from finding ways to facilitate your
thinking, rather than endless reduction for the sake of reduction.

As teleportation is driven by thought occasionally some
["precognition"](https://twitter.com/gregmcintyre/status/1194811646234873856) is
required. I [recently gave an
example](https://twitter.com/_justanotherdot/status/1194732136948875264) where I
and former colleagues would abuse vim's "paragraphs" to jump up and down between
chunks of text by leaving gaps of newlines between them and hitting `Shift-{` or
`Shift-}` respectively. I also use syntactical constructs to [form
barriers](https://www.justanotherdot.com/posts/dumping-grounds-for-good-and-bad.html)
where chunks of text might go to die if they aren't ultimately used or I might
further abuse whitespace to do [temporary debug
statements](https://www.justanotherdot.com/posts/stdout-is-forever.html). As I
write this article my editor is cutting newlines at eighty characters to make
sculpting up sentences and paragraphs easier.

This brings us to an important point; teleportation is editor agnostic. All
editor users alike, by experience and refinement, have been taught and taught
their editors how to zip around as if they are lightning incarnate.

As such, we have, as a larger community, cultivated a melting pot of ideas that
continually enhance teleportation as a practice. Sublime, VSCode, and others
have popularized the idea of the fuzzy-find palette for discovering files, text
matches in a buffer, git commits for a project, and so on. Things like `fzf` and
plugins for it now make this accessible to editors that don't have built-in
support. I particularly love fuzzy-find because it favours an aspect of
teleportation I call _course correction_, so long as the "palette" in question
provides a collection of results. From the results we can change our mind about
the direction we want to head. We can even simply go to some other option
without having to delete and type different results so long as the option is
present (you can do this in `fzf` with `Ctrl-P` and `Ctrl-N`).

Next time you trudge your way across your editor by keyboard or mouse, think
about how you could be teleporting, instead. Spend all those lost minutes on
stuff you want to spend them on. This principle is flexible enough to support
all sorts of optimisations and hopefully I've piqued your interest to explore
building your own.
