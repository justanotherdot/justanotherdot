---
title: Running Build Bots On Premise
author: Ryan James Spencer
date: 2019-12-22T04:13:05.717405584+00:00
tags:
  - continuous integration
  - ci
  - build bots
  - computers
  - compute
  - local compute
  - cloud compute
  - hardware
summary: >-
  Late November I did a video series discussing continuous integration and
  automation strategies for projects. I used GitHub Actions as they aided me in
  demonstrating configuration of pipelines without setting up supporting
  infrastructure. If you are a developer making things having a fast response time
  for feedback is crucial and continuous integration helps drastically.
hero_font_color: white
---

Late November I did a [video
series](https://www.youtube.com/watch?v=DL_hODqnUy0&list=PLG8S6YrJRoYI3CIUqvGX4NBSaMWZxe9in)
discussing continuous integration and automation strategies for projects. I used
[GitHub Actions](https://github.com/features/actions) as they aided me in
demonstrating configuration of pipelines without setting up supporting
infrastructure. If you are a developer making things having a fast response time
for feedback is crucial and continuous integration helps drastically.

When you use a SaaS offering for CI you are stuck using whatever tiers and
upgrades are on offer. For the last six months, however, I've not used a SaaS
offering for my infrastructure and instead have chosen to run computers in my
home. I use buildkite to pick my own infrastructure. I did the numbers for
renting my ideal EC2 instances on AWS and figured I could pay the same amount of
money to buy a machine or two to do my bidding that would still be relevant four
to five years later. Buildkite has an offering for an elastic stack build agent
that can scale to zero when idle but I the stack configuration was too bloated
to my liking. Nonetheless, having the ability to opt into whatever
infrastructure you please has some cool consequences and I doubt this will be my
last post on the subject!

Regardless of running compute locally or in the cloud, one can choose if they
want to pay the overhead of virtualization or let things build on bare metal,
which makes sense for slower machines. In the case of local compute this is a
"true" bare metal unless you are paying a cloud provider the money to not rent
in a co-tenant server. It does help to _build_ things in a virtualized
environment with the excuse that it is a "clean room" but if you want to do a
compile check or run some tests you probably don't care about dirt.

Initially I bought three raspberry pis; two B+'s and one Zero. The intent was to
run docker on them but I had forgot the host needs to be the same architecture
as the image you intend to build on and I often use x86_64 images. There was
nothing stopping me from converting these little boxes to running the languages
directly for tests and basic checks. I have yet to see any architecture specific
failures with the languages I tend to build for. These agents don't produce
artifacts as I don't need ARM releases.

Later I took a box I use for streaming video, a [2011 Mac
mini](https://everymac.com/systems/apple/mac_mini/specs/mac-mini-core-i7-2.7-mid-2011-specs.html)
I upgraded to 16GB of memory, and equipped it with a few buildkite agents and
docker. I have since reduced the box to running a sole agent. This build bot has
been the most frustrating because the box isn't only doing CI related things.
It's easy for docker to randomly die or get OOMed as the browsers can take up a
fair portion of CPU and memory when streaming video, often ~20-30% CPU load, and
it makes no sense for me to constrain the docker daemon unnecessarily.
Eventually I split configurations into two types: those that use docker or those
that don't.

I also own a rather beefy Intel ATX tower that sometimes participates as a build
bot. I recently dissected an older tower to contribute whatever parts I could
collect to build another ATX box for full-time builds to ease pressure off the
mac mini on release builds. No matter what the machine is, I try to use it do
any kind of automation, CI related or not. I was curious if anyone else was
crazed enough like me to do this. I poised a question on twitter a bit
indirectly:

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">Those paid
professionally to code, how often do you dump money into buying
computers?</p>&mdash; Ryan James Spencer (@_justanotherdot) <a
href="https://twitter.com/_justanotherdot/status/1208218000626634753?ref_src=twsrc%5Etfw">December
21, 2019</a></blockquote> <script async
src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

My intent was to find out how often people buy various machines and actually put
them to use for purposes beyond what they use directly. I got several awesome
responses, primarily that often people buy new machines every 4-7yrs and, no,
this isn't really a common thing, at least for the people following me and keen
enough to have respond.

That said, a few cool machines were mentioned:

* [NUC](https://www.intel.com.au/content/www/au/en/products/boards-kits/nuc.html)
  is a small form-factor device sold by Intel. There are equivalent ones
  such as from [System76](https://system76.com/desktops/meerkat) and if you care
  about open firmware it's well worth supporting them!
* [Mintbox3](https://fit-iot.com/web/product/mintbox3-pro/) -pro and -basic
  both seem really cool and are
  competitively priced in comparison to building your own machine. They are also
  fanless if noise is a concern!
* You wouldn't run this as a node in the cluster itself, but I had never heard
  of the [GDP portable
  ultrabooks](https://www.amazon.com.au/GPD-Portable-Ultrabook-Notebook-m3-8100Y/dp/B07W8MW2ZR)
  that seem like they'd be useful for quickly SSH'ing into a box without having
  to carry around a full-sized laptop.

Inevitably there are other computers I'd love to own "just cus":

* [Rockpro64](https://www.pine64.org/rockpro64/) which Daniel Lemire
  occasionally throw into his benchmarks
* [Hi-Five unleashed](https://www.sifive.com/boards/hifive-unleashed)
* [System76 Thelio Massive](https://system76.com/desktops/thelio-massive-b1/configure)

You don't need a super computer to build and test software. Unless you are
looking for a laptop or need really bespoke hardware you can _generally_ build a
desktop machine that'll run laps around most of its earlier variants. Building
your own also gives you a degree of customisation although, to be fair, it is
partly on par with cloud compute options: if you want to upgrade your file
system storage to a higher IOPS device, for example, you can abstractly do that
with a cloud provider, although you get a much finer degree of resolution with
your own computers.

Now that renting compute from cloud providers is commonplace, I suspect most
people would favor the ease of cattle-based systems administration and simply
slay any misbehaving servers. I find doing local systems administrations to be a
bit educational and cathartic in the sense of being a master of what you have
and working within limitations.

In terms of availability it's ok! There is always the risk that my children flip
a power switch or something goes wrong with my internet connection or power.
This sort of thing is already solved if you have, say, an AWS EC2 spot instance
in an autoscaling group where the terminating box will swiftly be replaced.
Because of this risk I occasionally supplement with cloud compute temporarily.

There are a few lingering things such as:

  1. External access into the specific network that has the bots. Something like
     [smith.st](https://smith.st/) and [wireguard](https://www.wireguard.com/)
     could supplant this if configured correctly. I remember seeing Brad
     Fitzpatrick [asking about ways to do a TLS based
     termination](https://twitter.com/bradfitz/status/1206058552357355520?s=20)
     into his home network awhile back.

  2. Doing smarter things with scaling agents in and out as ewll as scaling to
     zero, regardless of location. I had one crazy thought which was to have
     agents scale out if local bots have pending work but haven't picked
     anything up in X amount of time, say an hour tops. The newly spun up agent
     could sit around for an hour attempting to pick up work and then kill
     itself when idle, too.

If you or someone you know runs local compute at home I'd love to get in touch
and see what usages are in practice out there. I'm always curious to see how
people are using on premise computing rather than switching entirely over to
cloud compute.
