---
title: An Architecture Agnostic Intro To Assembly
author: Ryan James Spencer
date: 2020-08-14T06:20:36.957169552+00:00
tags:
  - rust
  - performance
  - assembly
summary: >-
  Deciphering assembly can be a harrowing activity, but it doesn't have to be.
  I look into various classes of instructions across platforms to lay out a top
  level mental model of assembly instruction sets to help guide future research
  and exploration when staring at the assembly of your programs.
---

Assembly is the last "human readable" frontier before reaching the representation of a program that a machine consumes. Times have certainly changed; the PDP-11 had only *16 instructions* in total, whereas the [Intel x86_64 and x86 manual is well over 2000 pages](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf). You may not ever need to write assembly yourself, but if you ever want to understand what comes out of a compiler, you'll need to know how assembly works. Due to the staggering complexity of modern processors, learning assembly for different architectures could take forever to grok in their entirety. Alas, there's no need to know every nuance of each platform's assembly language; there's enough generalities between Instruction Set Architectures, or ISAs, to lead you to more research on a case-by-case basis.

No matter the ISA, the instructions they offer tend to fall into three primary groups.

Some terminology first:

- A **register** is a named place in memory that is fast to access as it is normally right next to the CPU. That is to say data doesn't have to travel across a bus to read or write from the location in memory.
- A **word** is the *most common* width of a CPU. If a CPU works usually 32-bits, then a word is 32-bits. This isn't to say some CPUs that are, say, 64-bits wide can't work in 128 bits or 16-bits.
    - Note: contrast this by the odd Intel convention that a word is sometimes 16-bits. This means there are double-words (32-bits) and quad-words (64-bits) which you'll see in some instruction manuals. This comes from the quirk that Intel stuck to the 8086 convention since they introduced 32-bit mode with the i386. As such, you can actually run an x86 or x86_64 machine in 16-bit mode, also known as "real" mode.
- A **byte** is *almost always* 8 bits. There are some earlier machines that did not have this but you don't have to think about them anymore for the most part unless you are being excruciatingly specific to porting to all sorts of machines.This is now an IEC standard: [IEC 80000-13:2008](https://en.wikipedia.org/wiki/ISO/IEC_80000)
- A **nibble** is half a byte.

Some terminology such as "sentence" and "paragraph" are no longer in common use and so I won't bother with them here. Also, if you have no prior understanding of how a CPU works, I strongly recommend you go read my article on [what's in a CPU](https://www.justanotherdot.com/posts/what-makes-up-a-cpu.html), first. Although I will be explaining this with x86 and x86-64 examples, I want to make it clear that the classes of instructions are the more important part here rather than the specific names of instructions or how how they precisely work under the hood.

No matter what, instructions are always short mnemonic words. They take zero or more arguments or "operands". Some assembly languages support putting direct values or constants into memory by specifying the "section" and writing out specific values. The "text" value is historically the section in a binary where code lives, whereas other sections designate types of memory to be included in the final binary. Different syntaxes, such as "intel" and "att" (short for AT&T), designate effectively the same content but represented with different symbols and words. When in doubt or encountering an instruction for the first time, check the manual or search around for further details. There are normally sites that include good per-instruction explanations as well as other assembly courses for specific architectures. The point of this article being architecture agnostic is that many assembly languages have plenty of things in common and it's best to grok those top-level ideas first before diving into internals. Hopefully these segments help guide you in your deeper explorations.

Pretty much the only supported "data structure" in assembly is that of a stack. This model forms the basis of functions at the assembly level. I mention them here because they're a bit of a combination of several of the classes we're about to talk about. When you want to call a function, you push a number of things onto the stack, such as the return address where you were at before you called the function, local data for the function, and so on. The format for how things should be put on the stack for a function call is called the _calling convention_. Functions are a unit of computation that drastically improve code legibility and reuse and, well, I really shouldn't need to explain why functions are awesome.

### Reading and Writing Data To Memory

There are usually instructions to read and write to memory. Data has to move around in a system for it to be changed into anything meaningful. Reading is sometimes known as a load and writing is sometimes known as a store. Depending on the architecture this may include ways to indirectly access memory: you specify an address given some syntax and the value that is at that address is used. This is the assembly version of a pointer.

Memory tends to fall into registers and memory at particular addresses, which could be main memory or could be hardware. Hence writing and reading some address in memory might be communicating to a device, or it may simply be storing or querying a value in memory. When we communicate to memory in this way we call it memory mapped IO.

Some architectures require all memory to be put into registers before before manipulating it or writing it back out. This style is known as "load-store" and is what RISC-V and ARM requires, for example. Intel and AMD, however, allow you to manipulate memory directly, which is known as a "register-memory" architecture. We'll cover what I mean by "manipulate" here in the next section.

Common instruction names for this are called `mov` for "move", `lea` for "load effective address", instructions that manipulate a "stack" such as `push` and `pop`, and others. As you read through assembly or instruction manuals, these will probably be the first or most common instructions you find.

### Transforming Data In Memory

There is usually some form of an ALU or Arithmetic Logic Unit. These were the "manipulations" I mentioned in the last section. As the unit name implies, there are both instructions that perform arithmetic as well as logic. Boolean operations, addition and subtraction, and bit manipulations such as shifting left or right, all fall into this camp. A lot of these instructions are the same across platforms as they form fundamentals, but some architectures provide fancier manipulations such as `popcnt` which will count all the ones in a binary operand. Particular platforms such as the now ancient VAX had instructions to do polynomial evaluation with the name `POLY`. If data is being updated, it falls into this category.

On most platforms when you perform a transformation on a value or set of values, it will generally update one of the registers provided. As noted in the last section, if you are on a "register-memory" architecture, some transformation operations can change data directly in memory, without having to be loaded to registers for manipulation.

### Branching and Conditions

We can move data around, transform it, but we can't be Turing complete without some kind of way to express conditions and branching. Branching is sometimes known as "jumping" as the code literally loads the value given in the jump or branch argument into the program counter and then executes the instruction found at that address.

Usually with jumps a comparison instruction is run, setting some state on the processor known as a "flag" or set of "flags", and then the following instruction will depend on which flags got set during that instruction. Some platforms, such as ARM, allow for conditional transformations, such as "add these two values together if the comparison was equal".

### Instructions That Work On Multiple Data

Now that we've seen most instructions, there are sets of instructions that can work on multiple sets of data simultaneously. This is hardware-level parallelism. There are different names for this. On x86 it's called "SIMD". Doing things in batches will always be faster than doing it one-word at a time, but it can be clunky to manipulate code in this manner as you generally must load up specific registers to perform these operations. These special registers are called vectors as they contain multiples of the same unit, hence these instructions are often called *vectorized* instructions.

Compilers are usually smart cookies. They can figure out how to generate vectorized code themselves when there are chunks of data that can be transferred. This is often called "autovectorized" and if you can learn the look of vectorized instructions for the architecture you are working on, you can easily tell if the compiler is generating the optimized code or not. In fact, this is the primary goal that most people exploring assembly use this knowledge for. If you know what the assembly is doing, then you can tell if the compiler is making good or bad choices and whether or not you have to step in to help nudge it in the right direction or be explicit.

### Quirky Instructions

I don't group these with transformational instructions as they may do special hardware specific things and possibly not change data at all. Some are about coordinating synchronization when updating memory. Examples of this on CISC, or Complicated Instruction Set Computer, architectures that can do things such as atomic instructions (e.g. `cas` for Compare And Swap), memory fences such as `mfence`, and so on. The most common "quirky" instruction that is on all platforms is the noop instruction, which stands for "no operation". This is useful for progressing the program counter without changing anything. A noop can be useful for waiting for a lock to be returned, for example, in what is commonly known as a "spinlock" on multi-core architectures.


## Where to next?

You know the basic classes of instructions, now it's time to start dumping out assembly and sifting through it! It will seem like nonsense at first and you will need to explore specific terminology and conventions, but following small examples first can give you a sense of direction. If you need a guide on how to dump assembly for your Rust programs or Rust examples, [you can read my article on the subject](https://www.justanotherdot.com/posts/magnifying-glasses-for-rust-assembly.html). Like any kind of programming it takes time and practice, but with experience you'll slowly get comfortable looking at and deciphering assembly. Picking apart assembly is yet another part of your performance toolkit. For me, when I encounter something I don't understand, I try to put it into a classification above to better know what to expect the instruction to do. As I mentioned, I would recommend generating small examples and watching the output. When you feel comfortable that you understand what is going on, try another small example. When you've got a good understanding of small examples ,try something bigger. Usually when I look at assembly for a program, I am looking one function at a time. It can still be daunting if lots of other functions have been inlined into the code, but you can always break pieces of logic up, granted the generated assembly code doesn't change, if it helps you better understand.
