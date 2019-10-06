---
title: A Plea For Style Guides
author: Ryan James Spencer
date: 2019-08-28T10:16:00Z
tags: [code style]
---

You commonly hear two particular attributes that drive style guides, and,
subsequently, automatic formatting tools: 'consistency' and 'readability'. The
argument goes that a developer reads a codebase far more than any other
interaction.

Now, you could always take a codebase that has a style and use [machine learning
to generate a formatter](https://github.com/antlr/codebuff) to keep things
'consistent' and 'readable'. This would get around the subjective definition of
readability because it's what the team picked through usage. Some feel that a
community driven style guide is ideal because then the codebase's 'readability'
is 'consistent' with the larger ecosystem, so formatting tools should simply be
blindly adopted.

Unfortunately, they are focusing on the wrong thing.

I have read a lot of code. I care about it as a practice and I like teaching
others how to do it, but I don't think it's the right metric for a style guide.
Bar things like the [obfuscated C contest](https://www.ioccc.org/), minified
markup and javascript, and many other mind-melting formats , most code I see is
actually quite 'readable'. Consistency is no better because you can have a style
that is consistently spaghetti.

In truth, **developers change code far more often than they read and write new
code and they sure as hell should be _deleting_ code with a frantically high
frequency, as well, if they aren't already.**

About two years back someone mentioned [the elm style
guide](https://elm-lang.org/docs/style-guide) to me. The focus on
ease-of-modification _for a human_ was eye-opening. With this mindset, alignment
was pointless. What good would it do a developer to re-align things after making
a change than to simply let them make the change by itself, communicate it
simply to their peers, and get it into master ASAP?

Then, later, another practice I adopted was adding newlines to assignments/let
bindings, taken from a team of brilliant software engineers. Every time I wrote
an `=` I would hit enter, allowing the name of the thing and its guts to be
distinct. The contents of the variable could be expanded, shrunk, removed
entirely, turned into an error, whatever. The name could be changed to better
suit constantly shifting needs and not highlight the guts in code review. It was
a handy pair.

I found sometimes having stuff as modifiable or deletable meant you would get
the other for free, similar to what people claim about consistency and
readability. In the end, the specific practices aren't important here. What is
important is that gaining momentum and keeping up against inertia is pivotal
when keeping a project relevant and rot-free. And, if you do the same thing
everywhere, you'll inevitably get a 'consistent' codebase, anyways!

Next time you write a style guide, try to think about the sea of changes that
will need to take place and the stuff that will get old and need to die before
you consider your codebase as another magnum opus.
