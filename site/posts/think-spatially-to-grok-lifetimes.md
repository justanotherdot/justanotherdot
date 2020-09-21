---
title: Think Spatially to Grok Lifetimes
author: Ryan James Spencer
date: 2020-09-19T04:32:14.514103735+00:00
tags:
  - rust
summary: >-
  Lifetimes and, hence, borrowing can be confusing, but a lot of that confusion
  can be cleared up by thinking about where values live as spaces and
  remembering that references must be able to reach values that still exist.
---

programs represent a **space**.

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-01.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>

this space may contain other spaces, values, and **bindings**.

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-02.jpg"
    alt="A diagram depicting bindings."
    title="A diagram depicting bindings.">
  </img>
</figure>

a binding *binds* or *associates* a name to a value. we use values in functions, operations, in data structures, and so on, by their name or by their value directly. in order to use a value we must **move** it into place. when values move they are never in both places at the same time, hence *original* values in Rust can be moved exactly zero or one times. this requirement allows us to track where values are used in a program.

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-03.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>

we could duplicate a value, but such a **clone** would not increase the number of times we can use the value, it simply has created a new, distinct value that can move around independent of the other. Some families of values automatically clone when they move, but this isn’t the norm. if we truly want to **reuse** an original value, we can create a **reference** to it. values do not live outside of the space where they are defined, unless they are moved to another space explicitly for use.

When a value is left behind in a space and we can no longer reach it, we say that it has been **dropped**.   in Rust, we can only take references to values that have not been dropped. in other words, if the value resides in a space that *contains* or *is equal* to the space where a reference will live, then we can create the reference. a similar analogy might be talking to someone in a house; you can only talk to the person if they still exist and are living in the same space or some attached space where they can hear or see you. This is in contrast to languages where a references is entirely valid if it points to something that no longer exists.

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-04.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>

if the intent is to *change* a value through a reference, you can only have one reference at a time, but if the intent is purely to *view* a referent, the value being referenced, then you can have as many references as you like. we call these **mutable** and **immutable** references respectively. Containment isn’t always required for references, though; sometimes the use of a reference can be elastic, meaning that the compiler understands the use is only for a given period of time and no longer. This elasticity comes from a property of references you may hear called **non-lexical lifetimes**, AKA **NLL**, meaning the space of the reference isn't tied to the space where the binding happened.

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-11.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>

the most obvious definition of space is with blocks using curly brackets, but functions, data types, and loops all define space. functions hide away space like a fold on a piece of paper. data structures may define space like a shed containing the original value(s). loops define space by compressing several spaces into what looks like the space of one. closures are portable spaces that *capture* or *close over* values and references to values. As we saw before, each space tends to also provide ways for values to be bound to names.

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-05.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>

the space of the entire program is not the same space of the `main` function. we call the space of the entire program a specific label, `'static`. other threads may run to completion well *after* the main thread has finished, for example, meaning we can not spin up threads that take references to original values in another thread. again, the static label is special in that it is *not* a placeholder but rather the name of a specific space.

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-06.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>

otherwise, these labels are placeholders which are *generic* names of spaces that will get filled in based on the context of where they are mentioned. because spaces are like spans describing where a value lives, along with some sense of time, as the program executes, we call these labels **lifetimes**. we say that original values are **owned** as the name owns the value it was bound to. since values have owners, we say that a reuse of a value through a reference is a **borrow** since the value will be given back.

Rust will assume that all references in function arguments point to the same space. this is known as **elision** as the labels of the spaces do not need mentioning and are removed (elided). however, sometimes you want to make it clear that each reference points to different spaces. we can declare these differences using explicit lifetime labels on the type or function in question that we tend to call **annotations**.

**when you run into a borrowing or a lifetime issue it can help to think *spatially*.** consider how the spaces look unpacked across your code. this can be tricky, but you can practice by occasionally inlining code directly from a function or unrolling a loop to help visualize the spaces in question. as with this article, drawing out the spaces can help abstract away a lot of the other noise that code may supply.  Here are some examples of spaces inclined into larger spaces:

<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-07.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>
<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-08.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>
<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-09.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>
<figure>
  <img
    src="/assets/images/think-spatially-to-grok-lifetimes-10.jpg"
    alt="A diagram depicting a program."
    title="A diagram depicting a program.">
  </img>
</figure>

lastly, you should generally make lots of small programs to learn the ins and outs of fundamentals, and making minimally reproducible cases of borrows and moves is no different. Running `cargo new` is cheap, and so is going to the Rust playground when you want to scratch an itch about a question you have.

