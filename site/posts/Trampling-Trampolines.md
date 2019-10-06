---
title: Trampling Trampolines
author: Ryan James Spencer
date: 2018-05-24T09:40:17Z
tags: [javascript, python, code]
---

## Continuation Passing Style

or CPS for short, is a way to ‘continue’ a function call by calling into another function. The simplest example would be:


    function cps(x, return) {
      return(x);
    }

The important thing is the callback is passed and it is called at the ‘completion’ of the function, passing things along.

CPS is not a problem in languages where Last Call Optimisation is commonplace. What these languages do (normally of eager evaluation) is collapse the stack frame and call the function call so long as there is nothing else beyond the function. [Here’s a fun rant by Joe Armstrong regarding the implementation and implication of Tail/Last Call Optimisation](http://erlang.org/pipermail/erlang-questions/2016-October/090663.html). Of note is how he is careful not to say this is the same as Tail Call Optimisation, or TCO, as that typically implies a recursive call.

## DIY

Languages that don’t collapse the stack or represent functions in a form that is conducive to fusion are subject to ‘blowing the stack’ (exceeding it’s maximum size) when writing a recursive function. The function may be correct, but for particular values it may grow too large and too many stack frames will be pushed onto the program’s stack.

There are some [interesting ways to mimic TCO](http://chrispenner.ca/posts/python-tail-recursion) in languages that don’t have native support for it. JavaScript does, technically, but implementation seems spotty across engines.  A clever way to mimic TCO in JavaScript is to rewrite your function to return a continuation, of sorts, and then wrapping it in another, generic, function (the trampoline) that knows about this arrangement in order to collapse the stack itself, for example:


    let ackermannGo = (n, m) => {
      if (m === 0) {
        return n+1;
      } else if (m > 0 && n === 0) {
        return ackermannGo(m-1, 1);
      } else if (m > 0 && n > 0) {
        return ackermannGo(m-1, ackermannGo(m, n-1));
      } else {
        throw new Error(`ERROR: unhandled case m: ${m} and n: ${n}`);
      }
    };

I’ve chosen the [Ackermann function](https://en.wikipedia.org/wiki/Ackermann_function) here as it’s a golden standard for testing recursive functionality in programming languages since its value grows rapidly even for small digits. I’m also using the convention of naming things as `go` to specify recursive helper functions with arguments we don’t care about. A common counterpart to `go` you might see (particularly popular in LISPs) is `do` .

If you play around with this function you should quickly find it exceeding the allotted call stack size. Let’s fix this with a trampoline:


    let trampoline = fn => (...args) => {
      let rv = fn(...args);
      while (typeof rv === 'function') {
        rv = rv();
      }
      return rv;
    };

    ackermannGo = (n, m) => {
      if (m === 0) {
        return n+1;
      } else if (m > 0 && n === 0) {
        return () => ackermannGo(m-1, 1);
      } else if (m > 0 && n > 0) {
        return () => ackermannGo(m-1, ackermannGo(m, n-1));
      } else {
        throw new Error(`urk: unhandled case m: ${m} and n: ${n}`);
      }
    };

Note the zero-arity functions we are returning in order to signal that the return value can be applied (specifically lines 13 and 15). What kind of functions will `trampoline` fail on given it’s current implementation? When you’ve given it some thought, consider this case:


    let higherOrderFunc = (n, acc, offset) => {
      if (n < 1) {
        return offset => acc+offset;
      }
      return () => higherOrderFunc(n-1, acc+n, offset);
    }

    let hof = trampoline(higherOrderFunc);

Although this will terminate, it won’t give us the correct result. We want a closure in the end, but here we will wind up with `NaN` since that is the result of adding any number in JavaScript to `undefined`, and since we are calling the result value, `rv`, with no arguments, we are technically passing `undefined` to this final value.

A *structural* type system is one which cares only about the form of given values, whereas a *nominal* type system cares about the *names* that values have in the system. In a language like Haskell, we’d use what’s known as a “data constructor” to declare a type. Consider:


    data Cont a = Stop a | Cont (() -> Cont a)

This says “for any given value of `a`, I can either be a `Stop` value wrapping some given value of `a`, or I can be a `Cont` wrapping a function which takes `unit` (the `()`, or, in other words, no arguments that matters) to another `Cont` value”

We can, again, mimic something similar using a `tag` field on an object, a la:


    higherOrderFunc = (n, acc, offset) => {
      if (n < 1) {
        return {
          tag: 'stop',
          val: offset => acc+offset
        };
      }
      return {
        tag: 'cont',
        val: () => higherOrderFunc(n-1, acc+n, offset)
      };
    };

Which we can leverage in a new definition of `trampoline`:


    trampoline = fn => (...args) => {
      let rv = fn(...args);
      while (rv.tag === 'cont') {
        rv = rv.val();
      }
      return rv.val; // Of tag 'stop'.
    };

We only have two cases for our ‘type’, so when we no longer have a `cont` tag, we must have a `stop`, and with this we get the correct result: a function!


## Conclusion

In mathematics it is just as important to reason about the types of objects we are dealing with as it is to reason about their values. The same is no different in programming. Even though you may say “I hack in a dynamically typed language, I don’t need to think about types”, the inverse is actually the truth! Hacking in a dynamically typed environment means juggling these notions around in your head rather than allowing the type checker make sense of the form of things for you.

*food for thought:* What we’ve done by tagging these values is upgrade a `union` type into a `discriminated union`. A `discriminated unions` is also sometimes known as a `sum` type. The power of passing around sum types is that we can reason about the cases in our code: recursion itself is similar to mathematical induction, and both are forms of breaking down data whereas their duals, co-recursion and co-induction, build up data. This is an important notion because it means that when we write things recursively in this form with inductive-like types (e.g. sums) we can ‘pattern match’ on their values and know something about each case. In the above examples we knew that `cont` always contained a function that took no arguments and returned another `cont` or `stop` tagged object. When we finally got a `stop` we knew we had our final result we could return, and since there were only two values we could be sure that those were the only two cases worth exploring (show exhaustivity checking as well as case analysis). As is common with FP, some languages give you the power of actually statically checking this; ensuring that you’ve considered all cases, whereas in others you’ll be left to discipline or libraries to replicate this, just like trampolines.

