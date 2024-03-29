---
title: What Makes Up a CPU?
author: Ryan James Spencer
date: 2020-07-07T23:23:00.767692476+00:00
tags:
  - rust
summary: >-
  If you're feeling lost about hardware or low-level related discussions in
  software, then it pays to learn the foundations of how a CPU works. Modern
  CPUs are a beast with a maddening amount of complexity, but you don't need to
  understand every iota of detail in order to build up a working mental model of
  their operation.
---

I consider Ben Eater's series on building an 8-bit CPU from scratch to be a
remarkable learning journey that anyone keen enough to better understand the
hardware they build software for should watch. I liked Ben's course so much that
I thought I'd write a little mini summary of it. This isn't trying to be
satisfactory replacement to Ben's video course and I *strongly* urge you to [go
watch the whole thing from start to
finish](https://www.youtube.com/watch?v=HyznrdDSSGM&list=PLowKtXNTBypGqImE405J2565dvjafglHU).
For example, I won't cover the details of circuit designs such as s-r latches, d
latches, d flip-flops, and j-k flip-flops the way Ben does, but I will try to
cover an overall view of the CPU as best as I remember and will also mention
some meta things I liked that Ben did regarding the design and assembly of the
CPU as a whole.

## The Core of a CPU

Modern digital electronics are like irrigation networks for electricity. CPUs
are no different. Data is stored either in memory that requires power (volatile,
as it will disappear when the power is lost) or not (non-volatile, such as flash
memory). Electricity flows to different parts of the system which I'll call
*units* via the **bus**, which simply holds some fixed size of data at a time.
We might generally call this a *word* and in this case a word is eight bits, as
this is an eight-bit CPU. Units have to be gated to talk or read from the bus to
avoid cross-talk. Data can be temporarily stored in **registers**, of which
Ben's CPU has six, not including the register he uses to display numbers from
binary, which he calls "out", and the memory register on the 64 bytes of Random
Access Memory, or **RAM**, he builds up to store data and code for programs.

All the registers on the CPU are eight-bits wide. One of these registers is the
**instruction register** where instructions from RAM will be fetched and stored
so they can be decoded into the actual bits that will control the various units
by the **control logic** unit. These bits enable or disable certain units, as
well as configure them to do different things. The act of translating the
instructions into these individual bits is called **decoding**. Decoding
instructions is merely the act of supplying the instruction as a key to a lookup
table, which we will talk about more later, to get the resulting set of bits.
This CPU is based on **microcode** which means that the instruction plus a
counter built into the control logic unit make up the key that is used for
translation. Not all CPUs need microcode. For example, a CPU might translate
directly to one set of bits rather than several broken up into steps; this is
what RISC, or Reduce Instruction Set Computer, architectures are supposed to be
like. The reality is that most modern RISC processors most likely include a bit
of microcode.

The other two registers are for the Arithmetic Logic Unit or **ALU**, which only
supports addition and subtraction in Ben's case. The registers are simply named
"A" and "B". Values stored in "A" and "B" are added or subtracted together and
stored back into the "A" register. Ben later augments the ALU with flags to
detect overflow and if an operation resulted in a zero so he can install
conditional instructions, which is stored in a "flags" register. As part of the
change, he also includes the flags as part of the key in the instruction
decoding phase.

From a pedantic point of view, the RAM and the "out" register plus its
associated display aren't actually part of the core of a working CPU, but they
help making writing programs easier by giving us a place to put data and code as
well as visualizing the results of our calculations. The output display is
actually rigged up using four seven-segment displays. With the instruction
decoding phase and the display, Ben uses Electrically Erasable Programmable
Read-only Memory, or **EEPROMs**, as the look up tables we mentioned before; on
one end he can feed some bit pattern and out the other end receive a result. A
good mental model is that with an EEPROM we 'select' some value in the memory
given some provided key, even though this typically called an "address". For the
"out" display he can provide the binary value and receive a bit pattern that is
just right for showing on one of the four seven segment displays. The display
actually display one-by-one but display so rapidly ("refresh") that the shift
isn't visible to the eyes.

Behind all of this is a pulse that goes "high", i.e., it emits a five volt
signal, at regular intervals, called the **clock**. Clocks drive a CPU by
breaking up actions into discrete steps. When the clock pulse is high, we might
call that **tik** and when the clock pulse is low we might call that **tok**.
Going tik, then tok, is called a **cycle**. Imagine some crank that shows single
images for a movie on a screen for every cycle. If you move the crank fast
enough and the images look to be close enough together in time, you wind up with
what looks like a fluid image, but you can also single-step the crank or move it
more slowly and notice all individual images making up the movie.

A **program counter** stores the next instruction that will execute on the
CPU.You can either increment the counter by one or change it to some absolute
value, which is how jump instructions, and therefore conditionals, work. All
instructions have two initial steps which involve pulling a value from the
program counter into the memory register on the RAM and then pulling the
addressed value into the instruction register. This way the program can startup
on any instruction but wind up back to where the program counter is pointing.
The memory register is needed to ensure that whatever address is chosen to
output or read into the RAM won't change simply because the rest of the system
has moved on, e.g., the bus value has changed.

Of notable mention is the reset switch he builds into the system, as well as an
ability to stop the clock to emulate a HALT instruction. These make starting
from the beginning of a program and stopping the program at a particular point
easier than simply having the program "spin" at the end.

## The Design and Assembly

There's a lot of meta things Ben does I think are applicable to a wide range of
projects.

If it's not obvious, I like that Ben breaks the CPU into separate parts or
units. This way Ben can focus on one thing at a time, building up earlier
tools and units for re-use later, freeing up his ability to think on different
problems without having to hold the whole of the CPUs design in his head at
once.

He tends to build out units using bare circuits and simple transistors, first,
moving onto integrated circuits, or ICs, later when things become tedious. Ben
actively takes the time to build out initial circuits to demonstrate some
essential electrical patterns. The way Ben leads up to d and j-k flip-flops
based on his prior building and use of d and s-r latches informs a *lot* of the
circuitry across the whole of the computer. Knowing how data gets "trapped",
forming the basis of memory, and how this plays into how memory is "gated" on
and off the bus, can help strip away whatever magic you might have left about
what is going on under the hood. He does have some supplemental videos on how
semiconductors, transistors, and diodes work that can help fill in fundamentals
beyond what the primary playlist covers.

On a number of units Ben goes out of his way to make driving the unit by hand,
such as allowing one to single-step the clock or program the RAM with
dip-switches. Doing this allows him to rapidly smoke test or make quick tweaks
on a given unit.

I like that he builds the EEPROM from scratch because it helps keep the attitude
that "there is no magic".

At several points he uses a multimeter and oscilloscope to visualize what is
going on with the circuits. This really drives in the metaphor of the electrical
current analogy and that the transistors and complicated logic are the gates,
feeding data into and out of particular pools.

In the core explanation, I noted that the RAM and LED display weren't crucial
parts of a CPU, but it is good that Ben included them because they helped
facilitate the process of building up the CPU, testing it, and making it useful
beyond simply being a doormat that blinks.

Last, but not least, it's a small thing but I like that Ben actively goes out of
his way to build a number of programs for the CPU to run. This verifies the CPU
is, indeed, working as intended, and helps highlight a bit of a discussion
around "what is Turing completeness?", or, more simply, how can a computer be
deemed of enough general-use such that it can be used to compute anything we
would generally hope for it to compute?

## Conclusions

Ben's courses are great. He covers a number of other topics, such as building up
a basic computer using a 6502 chip, talking about networking internals, checking
for reliability on a transmitted message, and he also has a pair of videos
building up a video card from scratch.

The brilliance of all of this is that there isn't any magic behind modern
computers. There is a lot of complexity through many, **many** layers of
abstractions, responsibilities, code, data, hardware, and so on, but the overall
view of things need not be that complex. Ben's CPU isn't the fastest or
feature-complete, but it does give a mental model for a basis of a CPU.

Modern CPUs are a *beast,* but you don't need to know everything to feel like
you know enough of a subject to be dangerous. Many modern-day, bootcamp-trained
software developers have learned specific patterns that might aide them with
particular stacks of technology, but I have helped mentor a number of these
people who wish they could better understand just what is going on under the
hood. Holding the idea that there isn't any magic and you can continually
unravel the layers and components to gain a deeper understanding *is* possible
and it is thoroughly rewarding. You don't need to know everything at once,
either! Understanding how a CPU works and some fundamentals of electronics both
at a rudimentary level *will* help guide you with other understanding other
things, and with time you can refine that understanding.
