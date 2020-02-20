---
title: "The Lowly Assert: Idempotence"
author: Ryan James Spencer
date: 2019-10-30T10:28:07.931497878+00:00
tags: [the lowly assert, assertion, mathematics]
summary: >-
  Charging someone twice is bad for business; it burns trust with customers and
  it involves a lot of unnecessary churn. Payment providers go to great efforts
  to support idempotent endpoints. When you do something more than a given
  number of times, and every time after that, things don't change. In the case
  of a payment it would be once and only once, no matter how many times the
  request was submitted after that.
hero_font_color: dark
---

Charging someone twice is bad for business; it burns trust with customers and it
involves a lot of unnecessary churn. Payment providers go to [great
efforts](https://stripe.com/au/blog/idempotency) to support _idempotent_
endpoints. When you do something more than a given number of times, and every
time after that, things don't change. In the case of a payment it would be once
and only once, no matter how many times the request was submitted after that.

An
[involutive](https://www.justanotherdot.com/posts/the-lowly-assert-involution.html)
function is idempotent modulo a _certain_ number of applications. Involutive:
Driving a car around a square block means after four turns you're back on the
same corner you began on. Idempotent: A volume knob that reaches maximum volume
but still keeps turning. The assertion of idempotence looks suspiciously like
involution, but the concepts aren't quite the same:

```
-- Involutive

f(x)       != x
f(f(x))    == x
f(f(f(x))) != x

-- Idempotent

g(x)       == x
g(g(x))    == x
g(g(g(x))) == x
```

If the function `f` was a one-hundred and eighty degree turn around a point then
the next part of the series would be another equality and would alternate back
and forth for every other function application. In the case of `g`, we do
something once, twice, or n-many times and nothing seems to change. Per the
volume example, there might be _some_ changes initially, but `g` becomes
idempotent at or after a particular value.

[Idempotence](https://en.wikipedia.org/wiki/Idempotence) can relate to values,
but it can also relate to side effects, such as the payment example we've given
above. A "thunk" is a function that performs a calculation once and then stores
("memoizes") that result to return on all future calls: in this case a thunk is
idempotent in its computation: it's lazy _and_ cached.

Things don't always _need_ to be idempotent but can be chosen to be idempotent
for stylistic reasons. One API may force users to use explicit `insert` and
`update` calls, managing the housekeeping of keys itself, whereas a different,
but equally effective, API could allow a single endpoint that "saves" the
provided data, inserting at first, overwriting when the data is different, and
idempotent when the data is the same, forcing the tracking of keys on the
client. Both of these are valid options but have different trade offs for
particular applications!

When you think of idempotence, think about the mental model of things "clamping"
into place for a particular subset, or all, of our domain (inputs). And while
you're at it, make sure no one ever gets charged again for smashing the refresh
button for a slowly loading payment submission page!
