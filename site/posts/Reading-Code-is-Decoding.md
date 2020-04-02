---
title: Reading Code is Decoding
author: Ryan James Spencer
date: 2018-01-27T03:43:09Z
tags: [software, reading]
summary: >-
  Early this January I finished reading Coders at Work and in each interview
  there is a recurring question of “how do you read code?” Here’s a rough summary
  of some styles mentioned I found particularly useful.
---

Roger Antonsen says in his Ted Talk [*Mathematics is
the Secret to Understanding the
World*](https://www.ted.com/talks/roger_antonsen_math_is_the_hidden_secret_to_understanding_the_world),
mathematics, or rather the act of understanding, is largely about:


- Discovering patterns
- Devising language(s) to express said patterns
- Making assumptions
- Playing around with all of the above

Early this January I finished reading *Coders at Work* and in each interview
there is a recurring question of “how do you read code?” Here’s a rough summary
of some styles mentioned I found particularly useful:


- Get the code building early and often and make various changes to study
  connections
- Read it like literature whether printed out or jumping around
- Rewrite the code into a version optimised for legibility
- Puzzle through it the same way one would tackle a mathematical problem

It turns out I had previously read [a post from Peter
Seibel](http://www.gigamonkeys.com/code-reading/), the book’s author, who had
tried on several occasions to start code reading groups at his places of work,
in which he states:


> It was sometime after that presentation that I finally realized the obvious:
> code is not literature. We don’t read code, we decode it. We examine it. A
> piece of code is not literature; it is a specimen.

He goes on to quote a passage (my favourite in the book) of his interview with
Knuth (emphasis added by me):


> Knuth: But it’s really worth it for what it builds in your brain. So how do I
> do it? There was a machine called the Bunker Ramo 300 and somebody told me
> that the Fortran compiler for this machine was really amazingly fast, but
> nobody had any idea why it worked. I got a copy of the source-code listing
> for it. I didn’t have a manual for the machine, so I wasn’t even sure what
> the machine language was.
>
> But I took it as an interesting challenge. I could figure out `BEGIN` and
> then I would start to decode. The operation codes had some two-letter
> mnemonics and so I could start to figure out “This probably was a load
> instruction, this probably was a branch.” And I knew it was a Fortran
> compiler, so at some point it looked at column seven of a card, and that was
> where it would tell if it was a comment or not.
>
> After three hours I had figured out a little bit about the machine. Then I
> found these big, branching tables. So it was a puzzle and I kept just making
> little charts like I’m working at a security agency trying to decode a secret
> code. But I knew it worked and I knew it was a Fortran compiler—it wasn’t
> encrypted in the sense that it was intentionally obscure; it was only in code
> because I hadn’t gotten the manual for the machine.
>
> Eventually I was able to figure out why this compiler was so fast.
> Unfortunately it wasn’t because the algorithms were brilliant; it was just
> because they had used unstructured programming and hand optimized the code to
> the hilt.
>
> It was just basically the way you solve some kind of an unknown puzzle—make
> tables and charts and get a little more information here and make a
> hypothesis. In general when I’m reading a technical paper, it’s the same
> challenge. I’m trying to get into the author’s mind, trying to figure out
> what the concept is. **The more you learn to read other people’s stuff, the
> more able you are to invent your own in the future, it seems to me.**

Alas, if we’re to treat literacy in a human language as the combined skills of
writing *and* reading, why do we place so much emphasis on the former when it
comes to teaching how to code? I now actively seek out code to read for the
same reason Knuth mentions early in his interview; dispelling magic is an
invaluable skill we crucially need to keep improving. Treating things as a
black box may sometimes help reasoning but it doesn’t mean we should keep the
covers on until the end of the universe.

Take my [favourite mathematical
post](https://j2kun.svbtle.com/mathematicians-are-chronically-lost-and-confused)
by Jeremy Kun in which he discusses, with a wonderful supporting analogy from
Andrew Wiles about stumbling around a dark house looking for light switches,
that feeling lost is far more common and acceptable than the enlightened state
we assume intelligent role models seem to possess. These role models have
simply learned to live with and accept the discomfort of being lost because
that’s what it means to be in a process of learning and growing!

Simon Peyton Jones is well known for stating how important it is to simply
*do*, no matter how humble the project in question may be. This is fantastic
advice for coding literacy; Writing this blog post involved an initial sit down
of a roughly one-thousand word brain dump followed thereafter by approximately
two days of refinements, with lots of rereading, simply honing in on the main
theme. It’s important to get fingers moving and code executing, but it’s just
as important to advocate to new starters that reading is something they should
be pouring time and attention into.

