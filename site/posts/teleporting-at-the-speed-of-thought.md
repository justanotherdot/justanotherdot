---
title: Teleporting At The Speed Of Thought
author: Ryan James Spencer
date:
tags: [editor, code, vim, nvim, principle, vscode]
---

Adept text editor users fly around and manipulate text as if by
**teleportation**. For me, this is a principle I hold dear when considering my
editing experience. Teleportation is chiefly driven by thought and is effortless
by construction. This is actually not something inherent to teleportaiton.
Instead of hopping in a car and driving around the neighbourhood to find where
you want to go, you tend to make a decision about your destination ahead of
time. Driving expects a certain amount of effort to reach a destination whereas
teleportation requires little effort if at all.

Some call this "code golf" but using that term implies falling into the trap of
constantly optimizing when the aim is to [carve up the text in front of
you](https://www.justanotherdot.com/posts/how-fast-can-you-take-your-time-kid.html).
As such, optimization comes from finding ways to facilitate your thinking,
rather than endless reduction for the sake of reduction.

As teleportation is driven by thought, occasionally there is a
["precognition"](https://twitter.com/gregmcintyre/status/1194811646234873856)
that can come with how to even drive the engine for that teleportation. I
recently gave the example for vim where I and former colleagues would tend to
abuse vim's sense of "paragraphs" to shift up and down easily between chunks of
text by simply leaving newline gaps between them and hitting `Shift-{` and
`Shift-}`. Although not _entirely_ related, I also use syntactical constructs to
[form things like
barriers](https://www.justanotherdot.com/posts/dumping-grounds-for-good-and-bad.html)
are where certain chunks of text might go to eventually die if they are
ultimately not used, or I might abuse whitespace to do [temporary debug
statements](https://www.justanotherdot.com/posts/stdout-is-forever.html).
Teleportation doesn't just mean _jumping_ someplace but also transporting text
somewhere, whether it be someplace else in a buffer or an textual purgatory.

This brings us to an important point; teleportation is editor agnostic. Although
in my experience the people who use vim and emacs have, from years of
experience, been taught by the editor, and also have taught the editor in
return, how to zip around as if they were lightning incarnate, I no doubt
support that years of experience with or careful application of Atom, Sublime,
VSCode, and so on would yield the same result.

There being a melting pot of editors means that we have managed to cultivate
ideas that continually enhance teleportation. Sublime, VSCode, and others
popularized the idea of the fuzzy-find palette for discovering files, text
results in a buffer, git commits, and so on. Things like `fzf` and plugins for
it now make this accessible to editors that don't have built-in support for it.
I particularly love fuzzy-find because it favours an aspect of teleportation I
call **course correction**, so long as the "palette" in question provides a
collection of results. From the results we can change our mind about the
direction we want to head. We can even simply go to some other option without
having to delete and type different results so long as the option  is shown in
the results.

Next time you trudge your way across your editor by keyboard or mouse, think
about how you could be teleporting, instead. Spend all those lost minutes on
things you really want to be spend them on. This principle is flexible enough to
support all sorts of optimisations and hopefully I've piqued your interest in
some initial ways you can try or build from.
