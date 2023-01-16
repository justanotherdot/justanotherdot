---
title: Time management is time sensitivity
author: Ryan James Spencer
date: 2023-01-16T07:45:50.189517857+00:00
tags:
  - software engineering
  - estimation
summary: >-
---

Time management advice tends to devolve into the treating time as an expendable
resource (it is) that must be cost-analysed (it should), but when people look up
"time management tips" I want to propose that they aren't looking for the how or
what of time expenditure, but rather are looking to become better _aware_ of
time. I base this argument on the fact that those working in jobs that
are conducive to "flow" state can lead to time loss. Sure, losing four hours to
solving a problem is good, right? But what if those four hours could have been
compressed to one, or if the pursuit lost value after a particular threshold?

[Philip Johnston](https://embeddedartistry.com/blog/2020/03/02/why-we-estimate/)
talks about the driver for estimation; we estimate because it _aligns_ team
members by exposing information in our heads in a public forum, allowing us to
discuss risks, possibilities, and so on. But if we are terribly insensitive to
time, our estimations won't be of much help. Of course, we can always _improve_
our estimations through practice, requiring us to record and reflect on our
estimates, determining the factors that went out and how things shifted from our
original guess. 

In the last couple of years I've come in close contact to two ways of developing
better time sensitivity. Albeit far from perfect, it has helped me refine
estimates, improve my ability to focus on things that matter, and avoid a
certain level of strategic myopia that crops us. 

The first way is known as time boxing. You determine a task, set a period of
time you suspect it will take, or that you are willing to allow, and commit to
abandoning the task within that period. Doing this *properly* means learning to
include the ramp down in the time frame, otherwise one is left with work strewn
everywhere. This is helpful in revisiting purpose or change. The intent isn't to
merely abandon tasks where the outcome may no longer be relevant or as high a
priority as initially determined, but moreso to get a higher standing and
evaluate lessons learned. At a microscopic scale, programming can be broken into
30-60secs feedback loops which, after the box edge is hit, involves reverting
back to a prior state, only to restart again, often with new knowledge that can
fuel simpler approaches. It is said that Joe Armstrong of Erlang fame told many
of this approach at conventions, whereby he would recommend throwing out your
work at the end of the day that wasn't finished. I'm not sure if he intended
only the parts that were incomplete, or if he meant the whole design, and if
someone could clarify that to me I would be grateful. The best I can find is
[various
anectdotes](https://twitter.com/sadisticsystems/status/1119614274538823687?s=20)
about Joe from [around the
internet.](https://github.com/lukego/blog/issues/32#issue-435504246). To splice
in a quote from sadisticsystems (spacejam):

> #rememberingjoe Once I had an opportunity to ask Joe a few questions in-person
> about workflow and managing complexity. He said he would throw code away that
> he couldn't complete in a day of work. This felt wrong to me then, but over
> time I've grown to appreciate it

> It's a razor that forces rewrites of code likely to have been warped to adapt
> to incorrect guesses when starting the task. It also forces a limit on the
> complexity of the implementation. You also notice and avoid many bugs that
> crept in during the failed first pass

> Since then I've met people who have taken similar ideas to the extreme.
> @yoshuawuyts said that when writing http://choo.io he would sometimes do a
> full-rewrite in one sitting. The result was craftsmanship and not just a code
> dump

This is excellent advice, as it means you are consistently exploring alternative
paths, which sharpens your divergent thinking and simplification skills. Can you
summarise your problem, as well as curate the research that went into it? What
variations were explored, either before you set to writing code, but as well as
when you started smashing the keys?

The second way is simply being highly conscious of time through simple analogue
means. A practical means of accomplishing this is wearing a non-smart watch. I
have a collection of casio watches I've collected over the years that I've taken
to wearing again, as the context switch and distraction-inducing nature of
phones means I'm more focused on the act of handling my phone than I am of
becoming conscious of how much time has elapsed since I last made note. As I
mentioned before, when lost in a flow state, we can lose awareness of external
stimuli. Although this can be fantastic for compressing work into smaller
spaces, it can also mean flow states where you are ignorant to the fact that
four hours has been lost solving the wrong problem. Part of the process of
becoming "more senior" in software engineering is bigger picture thinking; big
picture thinking involves understanding long-term and knock-on consequences but
it is also about understanding the context and specifics of a problem at hand.
Like the time boxing approach, this is intended to make you aware of how long
things actually take, at a finer resolution than you may be used to, again
prompting reviews of your work. Sure, doing that refactor in an hour made sense
when you thought you could do it, but it's now a little over two hours; do you
keep pushing or do you stop and take stock of where you are at? I'd say the
latter is to be preferred, as the "why" to our outcomes should always trump the
cardinality of our outcomes.

Lastly, there is an experiment I am doing with my calendar, which I suspect
people who have physical planners already benefit from, which is to spend a lot
of time on the yearly and monthly views rather than the weekly or daily. This
shoves my attention into a larger scale, meaning I am more conscious to think
about tasks extending out into weeks or months and the consequences of that
extension, rather than to focus on merely getting through what's immediately in
front of me for this week. This isn't solely identifying problematic upcoming
events, but again is about becoming sensitive to how time drags on, and
revisiting the question of whether or not the value of me or others dedicating X
amount of time on an initiative, or some part of those initiative, makes sense,
or if the flashlight of focus needs shifting.

Ultimately, despite what I've said in the past about [software
estimates](https://www.justanotherdot.com/posts/fools-gold-time-estimates.html),
I think the approximate value of estimation is valuable, but it requires
thinking about estimates on a real time scale, and not a BS scale such as
t-shirt sizes or other intangible metrics that are vague to others, and, worse
yet, yourself! Since writing that article, I've come to learn more estimation
techniques,  but estimation functions within the notion that what we are working
on can matter less or become completely pointless as time advances. The pitting
of efficiency against effectiveness is a classic dichotomy to this point; we can
be highly efficient, focusing moreso on cardinality of outcomes, rather than
effective, where the the outcomes have impact. In the efficient sense, we are
managing time by cutting the cost of evaluation out. But we aren't machines
where we spend extra cycles doing extra work and choosing after the fact. The
output we have available to us is limited, and easily put of course by even the
slightest of factors, hence we need to be wary of how we spend time, and we
don't get that awareness unless we have practice honing our sensitivity to time.
Surely we can be effective, and after that efficient, but focusing squarely on
efficiency misses the point of time management.
