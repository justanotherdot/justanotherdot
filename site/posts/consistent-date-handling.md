---
title: Consistent Date Handling
author: Ryan James Spencer
date: 2020-01-18T09:27:50.965014871+00:00
tags:
  - date handling
  - time
  - software
---

Date handling is the kind of funny where you sob from of the ways it can
horribly cut you when you least expect it. Developers either pretend that _all_
date handling concerns can be shoved onto third-party libraries or that they
don't exist at all. Here's a short, incomplete primer.

There are two common formats for storage; as strings or as integers. Although
integers have a history of heavy optimization on modern CPUs and compilers,
strings can have reasonable performance with the right memory structure. This
integer format is typically known as [Unix Epoch
Time](https://en.wikipedia.org/wiki/Unix_time) and the start of the world for
this format is January 1st, 1970; the birth of Unix. A 32-bit integer,
expressing seconds since January 1st, 1970, ends at 19 January 2038. 64-bit Unix
Epoch's will end [292 billion years from now, at 15:30:08 UTC on Sunday, 4
December 292,277,026,59](https://en.wikipedia.org/wiki/Year_2038_problem#Possible_solutions).
This is far after the estimated death of the universe and if your system is
still running after that I think that deserves a nice pat on the back.

Where you are is your _time zone_ based on wobbly, vertical slices of the world.
UTC is the "base" time zone and is such because it's on the prime meridian (zero
degrees longitude). Think "base time zone" where the offset is `00:00`. Let's
pretend we are sitting in a lawn chair in this time zone, which is the same as
GMT or "Greenwich Mean Time", so, not a sunny day.

As you recline in the lawn chair time passes by but the earth's rotations and
the solar orbits are complicated things. Time doesn't _just_ pass bit-by-bit. It
does in a mathematical sense, sure, but time is a construct with ideas such as
days, weeks, and years. To fit time into these relatively standard quantities,
such as the number of days per given month or total days in year, we must make
small adjustments to time, such as leap years, leap seconds, and daylight
savings. Each of these 'correct' some kind of drift. However, the Unix Epoch
format doesn't encode leap seconds, which is one type of correction.

Enter [ISO8601](https://en.wikipedia.org/wiki/ISO_8601). Among several nice
properties but for starters, humans can read it! The timestamp,

```
"2005-01-01T00:00:00"
```

is equivalent to the Unix Epoch,

```
1104537600
```

It's far easier to quickly determine if an ISO8601 is suffering corruption than
eyeballing integers. It's also much easier to tell what ballpark of dates an
ISO8601 is in. If you have a bunch of ISO8601 timestamps, you can sort them with
default strings comparison (lexicographic) and they will naturally be in
ascending order. I love this feature about them because it means I don't need to
rely on a library to order a bunch of well-formed ISO8601s. Opposed to our
fixed-precision Unix Epoch integers, ISO8601 allows for variable granularity.
You _can_ get finer granularity for time on Unix systems but I won't go too far
into that here. You can run `man 2 gettimeofday` and `man 2 clock_gettime` for a
slightly deeper understanding of some options on Linux.

Back to our lawn chairs someplace in Sunny England, time zones are expressed
officially as strings describing two parts separated by a forward slash, e.g.
`Australia/Sydney` or `America/Los_Angeles`. If you have any formatting you need
to do for a client reading data, you need to encode time zones. However, it is
OK to not deal with time zones if you are dealing with an "absolute time" for a
given data set that is fixed to a place. You then have a direct link between a
set of timestamps and time zone.

ISO8601 has an optional time zone specifier. RFC3339 enforces that the timezone
be specified but for the case of UTC you don't need to specify the specific
offset as it is implicit. Time zones tend to be exposed to many odd political
changes. Offsets assume a timezone will _always_ be a particular amount, but
this isn't quite true. As recent as 2011, Samoa changed their time zone for
trade reasons. Originally, `Pacific/Apia` had an offset of UTC-11 (note the
minus) but it changed for trade reasons and went to UTC+13 (note the plus).
That's a big jump! Thus, the timestamp:

```
2013-01-02T12:00:00Z-11:00
```

is invalid for Somoa. However, if you didn't specify the offset as part of the
stored data, you could get away with looking up the time zone for `Pacific/Apia`
indexed by some granularity, say, year. This way you can record both offsets
before and after 2011. We could encode the timestamp simply as

```
2013-01-01T01:00:00Z // UTC
```

and the lookup for the year 2013 would reveal that `Pacific/Apia` is `+13:00`
meaning we can now shift this date over properly for Samoans. In fact, you don't
even need to store the specific index as most time zone databases that third
party libraries provide already extensively document this information.

Its important to keep your timestamps in a canonical timezone. In the case of
Unix Epoch, by definition the seconds from midnight January 1st 1970 needs to be
in UTC, similar to the default timezone for ISO8601. Picking a canonical time
zone, specifically UTC, will save you a lot of time from painful sleuthing as
two dates without timezone information _could_ be from different timezones.

Chat applications have to pay particular care to this. If someone is sending
messages from San Francisco but another person is replying from Beijing, the
time difference is an important part of the UI. How many hours are we off? When
did they really read my message? Did they send it before or after me? Causality
is its own can of worms for distributed systems, hence this is a bit of a
hand-wavey argument that is ignoring some really critical (and nasty) aspects of
time, but having most things stored as a single time zone and stashing client
time zone preferences or figuring out what their device location is and using
the time zone from that can save a _lot_ of grief.

As a recap here's the basics:

1. **Pick one format and stick to it**
2. **Always store your timestamps in a single, canonical time zone**
3. **Store time zone preferences for clients as a string rather than the
   offset**

And that's a short primer. Time can get a lot more nuanced, such as focusing on
monotonically increasing time for servers and thinking through concerns like
storage space, but most applications won't need to care too much about these
things. A little upfront focus on consistency will save you a lot of shed tears.
