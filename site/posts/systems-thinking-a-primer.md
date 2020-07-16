---
title: "System's Thinking: A Primer"
author: Ryan James Spencer
date: 2020-07-16T06:18:52.277083099+00:00
tags:
  - systems
  - rust
summary: >-
  Are you confused about what to learn when it comes to the fundamentals of programming? Do you constantly get told that algorithms and data structures are the gold standard for what will help propel your understanding? How do you get better at designing solutions rather than operating preexisting ones?
---

Are you confused about what to learn when it comes to the fundamentals of programming? Do you constantly get told that algorithms and data structures are the gold standard for what will help propel your understanding? How do you get better at designing solutions rather than operating preexisting ones?

Most of what people try to shill as fundamentals does not end up empowering people with a framework around problem solving. People assume that instead of having the ability to solve problems themselves, they instead need to learn a grab-bag of tool-oriented prior knowledge. This taints one's arc of learning with the idea that it is all about gobbling up technical arcana, all the while reliant on others to find just which mystery to focus on next.

Mathematics and systems thinking has helps me build mental models of almost everything around me, aiding me in how I navigate the world and make decisions. You are likely carrying around *lots* of mental models with you already. Strong mental models that refine over time give you the ability to understand different programming languages without having to read entire books, immerse yourself in new code you've never seen before without having to ask a number of it's inventors who may have left little to no documentation, or, perhaps, how to reason about a market you are creating as part of a business. **Models describe the essence of these things; them being "mental" is merely a matter of keeping them in your head.**

Modeling is about describing systems. While a collection holds all the items contained, a system augments this with descriptions of how the items interrelate. **At it's essence, a system is a collection of shapes, usually boxes or circles, representing distinct entities, and arrows describing the relationships between these entities.** If you're mathematically inclined a system is a graph with nodes (the shapes) and edges (the arrows).

<figure>
  <img
    src="/assets/images/systems-thinking-a-primer-pdp11.jpg"
    alt="A functional block diagram of the PDP-11 computer"
    title="A functional block diagram of the PDP-11 computer">
  </img>
</figure>

The shapes in the graph expressed should **not** be thought of as black boxes. Instead, **think of a systems diagram as merely a view of the system at a particular scale.** It is not that we should never see the details of the entities in the system, as the popular notions around encapsulation may lead us to think, but that we are simply ignoring them when discussing the system at this level of detail.

<figure>
  <img
    src="/assets/images/systems-thinking-a-primer-555-timer.jpg"
    alt="A schematic for a 555 timer integrated circuit"
    title="A schematic for a 555 timer integrated circuit">
  </img>
</figure>

Systems that describe the flux of some unit, continuous or discrete, often refer to the unit flowing across the connections as a **stock**. **Taps** adjust the rate of flow of the stocks along the arrows. It's is a good practice to not overload the usage of what an arrow means by having it describe verbs *or* the inlet and outlet of stocks, but not both.

Systems inevitably end up describing repetitive actions, otherwise known as **loops**. A loop may consist of a way to communicate necessary changes to some other entity in the loop which we call **feedback**. A loop with finely-tuned feedback mechanism will seek some state of equilibrium while a loop whose feedback is disregarded or misaligned will self-reinforce in some general trend. If a stock is involved, you can see how producers and consumers are affecting the quantity of the stock in this system with a behavior-over-time graph.

<figure>
  <img
    src="/assets/images/systems-thinking-a-primer-feedback-loops.jpg"
    alt="A systems diagram showing the basics of feedback loops. Author: Kjell Magne Fauske licensed under CC BY 2.5"
    title="A systems diagram showing the basics of feedback loops. Author: Kjell Magne Fauske licensed under CC BY 2.5">
  </img>
</figure>

When we talk about misalignment of feedback in loops, we tend to mean the amount of delay of some message or action. **All loops have some delay in their communications**, but the tuning of that delay is important. Given the context of the system or loop, a short delay may be just as bad as a long delay. A system describing the supply and demand of a widget may want a certain sized delay so that demands for manufacture do not lead to oversupply.

An example of a system seeking a state of equilibrium may be a thermostat connected to a heater. If the heater reaches a target point on the thermostat, the heater stops until the thermostat drops below the target point. If the thermostat didn't exist, there would be no feedback and the heater would continue to increase over time. If there was a substantial delay in the feedback of the thermostat to the unit that controls whether or not the heater is on, the heat would also continue to grow up until a point. If you include a cooling system, you may get finer control of both directions of temperature. It's worth mentioning that most systems of growth have a saturation point, but that point may be functionally quite high in the context of use!

<figure>
  <img
    src="/assets/images/systems-thinking-a-primer-aircon.jpg"
    alt="A systems diagram showing the parts of an air conditioner. Author: Controlsystemintro licensed under CC BY-SA 4.0"
    title="A systems diagram showing the parts of an air conditioner. Author: Controlsystemintro licensed under CC BY-SA 4.0">
  </img>
</figure>

State machines also make up the basis of a system; the boxes are states, the arrows transitions between states, and the use of a state machine includes tracking the current state of where an agent in the state machine might be. This type of thinking is highly underappreciated when designing transactional systems such as network requests, payment flows, and so on. State machines help clarify if the flow of logic has any odd behavior, such as winding up in invalid states or transitions that lead to inescapable states.

<figure>
  <img
    src="/assets/images/systems-thinking-a-primer-state-machine.jpg"
    alt="A state machine describing the TCP/IP control flow. Author: Ivan Griffin licensed under CC BY 2.5"
    title="A state machine describing the TCP/IP control flow. Author: Ivan Griffin licensed under CC BY 2.5">
  </img>
</figure>

The best way to get better at systems thinking is to start drawing out systems diagrams. Drawing by hand in a journal is my preferred route, but there are also many services that make doing these easily. For example, here's an incredibly rough sketch of a diagram I copied from Richard Hamming's classic "The Art of Doing Science and Research" showing the essence of a CPU.

<figure>
  <img
    src="/assets/images/systems-thinking-a-primer-cpu.jpg"
    alt="A hand-drawn diagram of the essential parts of a CPU, based on
    Richard Hammings work in The Art of Doing Science and Engineering"
    title="A hand-drawn diagram of the essential parts of a CPU, based on
    Richard Hammings work in The Art of Doing Science and Engineering">
  </img>
</figure>

It also helps to see other people building up systems, as I've included some examples here. Functional block diagrams are my favorite but if you see any type of graph you could, theoretically, coerce that into the idea of a system and think about its causal relationships, how time plays a part of the system and the states that it may be in, how the author of the diagram has chosen to render distinct parts of the system, or maybe combine both the flow of units *as well as* the actions between nodes and see how confusing that gets. There are lots of places like the [Internet Archive](https://archive.org/) and [Bitsavers](http://bitsavers.org/) that have scanned copies of old computer reference manuals with plenty of circuit and functional block diagrams that are great places to start, and an article discussing architectural changes is worth it's weight in gold, [like this one by Charity Majors](https://www.honeycomb.io/blog/secondary-storage-to-just-storage/). *Don't worry about system diagrams being perfect; in the same way that writing can help you refine your thoughts, systems diagrams can help refine your thinking around a system.*

