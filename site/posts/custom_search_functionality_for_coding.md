---
title: Custom Search Functionality for Coding
author: Ryan James Spencer
date: 2019-08-19T06:10:36Z
tags: [code, software]
---

This may not be revelatory to some, but it's a cool trick I use daily and I
thought I'd write about since it's managed to surprise enough colleagues and
friends when I've used it. Credit where credit is due, I was taught this trick
two years ago by Charles O'Farrell.

Firefox and Chrome both support this functionality but are setup differently.
Let's say you have a github codebase with a particular org (which are also,
confusingly, demarcated as users in github search). You want to find a repo
quickly; you can quickly go to your search bar and hit `repos a_project` (or in
the case of Chrome, `repos<tab> a_project`), hammer the enter key and you wind
up at `https://github.com/search?q=user%3Aorg+a_project`. How?

In both Chrome and Firefox, you can add a custom search engine by right-clicking
on the search 'bar' (form) you'd like to add, except in Firefox the mechanism
works via bookmarks and Chrome has the functionality as it's own thing (seems
like a first class citizen). There _are_ custom search engines for Firefox you
can add (I've noticed I can add them for things like crates.io, docs.rs, amazon,
et. al.) out of the box given a specific version (I'm not sure which) of Firefox
but you'll still need to bookmark approach for most cases. Once you have the
search URL you care about just replace the term you searched for with `%s` and
all is well, e.g. `https://github.com/search?q=user%3Aorg+%s`.

Some other examples of uses are:

* `code term` - similar to the `repos` keyword above but searches across all
    repositories of an org for `term`
* `(docs.rs|crates.rs|younameit) <module>` - looks for module documentation,
    listing in some large store of knowledge
* `rstd term` - search for `term` in the rust std lib (handy when paired with
    something like rust-tags so you can jump to definition inside of std in your
    editor)

The above are rust centric because it's what I've been in the headspace of but
you could easily set this up for things like hoogle, amazon, shortening things
like youtube to `y`, `hex.pm`, and so on. Personally, it's been empowering to
gain a handle on parameterizing search functionality with your address bar.
