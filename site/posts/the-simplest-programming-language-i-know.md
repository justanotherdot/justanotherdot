---
title: The Simplest Programming Language I Know
author: Ryan James Spencer
date: 2019-11-20T10:35:16.488307008+00:00
tags: [lambda calculus, computation]
summary: >-
  I'm going to teach you the simplest programming language I know.
---

I'm going to teach you the simplest programming language I know.

Everything starts with functions:

```
(\x -> x)
```

`x` is the input argument. There may be more than one by adding comma separated
values, e.g. `x, y, z`, and so on. The body comes after the arrow (`->`).

`x` is a variable. We can call functions like this:

```
(\x -> x) 3
```

This says "substitute the value of `3` everywhere you see `x` in the body of the
function", like this:

```
1. (\x -> x) 3
2. (\x = 3 -> x)
3. (3)
4. 3
```

Here's another example with more than one argument:

```
1. (\x, y -> x + y) 3 4
2. (\x = 3, y = 4 -> x + y)
3. (3 + 4)
4. 7
```

Calling a function is called "function application" and when all arguments are
substituted with actual value we are left with the result. When we assign values
to variable names we call it "binding":

```
1. f = (\x -> x)
2. f 3
3. 3
```

Sometimes when we talk about function application. It can be bit-by-bit:

```
1. (\x, y -> x + y) 3 4
2. (\x = 3, y -> x + y) 4
3. (\y -> 3 + y) 4
4. (\y = 4 -> 3 + y) 4
5. (3 + 4)
6. 7
```

Functions are values. We call the above "partial application" because we get
functions back when we apply one argument at a time. This format for function
application is called "currying" where a function takes one argument at a time.

This programming language has many types but has no way to check this before
running the program. Hence we can wind up with weird expressions such as adding
the number `3` and `true` together. This programming language is called the
["lambda calculus"](https://en.wikipedia.org/wiki/Lambda_calculus) and when you
add a basic form of types you get the ["simply typed lambda
calculus"](https://en.wikipedia.org/wiki/Simply_typed_lambda_calculus).

And now you know one basis of functional programming and a model of computation.
