---
title: Lightweight is Beautiful
author: Ryan James Spencer
date: 2019-09-08T10:03:00Z
tags: [pattern, feedback loop, verification]
---

We are all guilty of having done the "edit a little bit, go to another terminal,
hit the up-arrow a number of times, fire off the found command" dance over and
over again at some point in our careers. It's such an easy automation to remove
these steps! IDEs give this to you because they know best about when a buffer or
a file has been saved or modified. Indeed, people go crazy for IDEs because they
provide information directly in the editor.

Even though things like VSCode and the Language Server Protocol have done a
tremendous amount of work in reducing complexity around both the setup and
maintenance of an IDE environment since days of yore, there are still times when
the array of plugins and external tooling 'go wrong'. Bugs or even the nefarious
'opinionated' feature can cripple a dev's workflow. Fixing these issues isn't
necessarily time poorly spent but it's hard to shrug off because the integration
is so tight-knit—now that you depend so heavily on the plugin, switching to
something different is slow. Here's an approach I think is a bit more
[antifragile](https://www.goodreads.com/book/show/13530973-antifragile), to use
a term coined by author Nassim Taleb. An antifragile approach is distinct from a
fragile approach because

* a fragile approach will break when encountering an unexpected event and
* a robust approach does not change when encountering an unexpected event but
* an antifragile approach gets better as it encounters unexpected events

I'm a bit spartan when it comes to coding. I do this largely because I've had a
lot of tooling mistreat me and this has taught me that the weight of a tool or
process is a matter of its cost. **Lightweight is beautiful**. By lightweight we
mean cheap to replace not 'small' and 'simple'. Sometimes you do need beastly
machines because you can't bore a hole into the earth to make a tunnel with a
few workers armed with spoons. **Lightweight functionality is preferable to
mindless adherence to a given tool or process.** In other words, it's
antifragile to be prone to lightweight .

So here is the setup; two terminals or windows or whatever you like to use. In
one is your source code and in the other is your tests, linting, typechecking,
you name it. Either they are side-by-side or perhaps there is a dead-simple way
for you to swap between them. You can have several of these going at once and in
fact I recommend it. If they are resilient to files changing from version
control that's even better. **It's important they stay _relevant_ and by that I
mean obvious and up-to-date.** When we talked about
[debugging](https://www.justanotherdot.com/posts/stdout_is_forever.html), this
is the very loop I was referring to. With this in place you can progressively
slap in debugging statements and changes while watching the results come seeping
out.

There are plenty of testing frameworks and tools that support automatically
running tests or commands on file save. `jest`, `PyTest`, `cargo watch`, `go
watcher`, `mix watch`, you name it. This sets up an automatic link between the
file(s) being edited and the suite of tests to run. Just alleviating the step
where you need to context switch is the small win here and is not the point.

With this approach, if anything like a plugin or even a specific command in the
pipeline you setup goes awry, you can cheaply swap it out for an alternative.
**This is the best kind of feedback loop as it favours tinkering and
experimentation.** Lately because I mostly write Rust at work, I tend to use
`cargo watch` but one incredibly handy, language agnostic tool is
[`entr`](http://eradman.com/entrproject/) which is useful when I foray into the
unknown or uncommon. Let's say I find that I need to run a particular pipeline,
I can do that by running,

`rg -l . | entr -cs 'cmd1; cmd2; cmd3'`

Now if `cmd2` is being a pain, I can take it out of the pipeline quickly or even
choose to replace it. Perhaps it's a new project and you are furiously adding
files, you could set up a governing loop that watches all files and tears down
the loop if that changes some known set.

`while true; do ls src/* | entr -d cmd; done`

Most people never even think of doing a `git bisect` because of the pain of
steering the interaction with the bisect and running the tests to confirm the
first failure in the regression suite. This isn't just the cost of swapping
between terminals. Sometimes it can be tests that are flaky and come up as false
positives or maybe a test suite is slow to run but there is no way to neatly run
a subsection without commenting out code. With this approach, however, we can
focus on the steering and watch what happens in the other window. If flaky or
slow tests show up, we can comment them out and move on (`git clean -fdxx` is
handy for these sort of tempermental changes if you tack on it on the back of
the pipeline you construct).

If a great style guide favours [deletability and ease of
modification](https://www.justanotherdot.com/posts/a_plea_for_style_guides.html),
this approach is stressing for **replaceability** for producing tinker-friendly,
antifragile feedback loops. If you lower friction you'll always beget action,
and [fast systems incur usage](http://jsomers.net/blog/speed-matters).
