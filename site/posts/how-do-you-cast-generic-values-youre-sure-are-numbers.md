---
title: How do you cast generic values you're sure are numbers?
author: Ryan James Spencer
date: 2020-10-20T05:29:46.336528293+00:00
tags:
  - rust
  - traits
  - generics
summary: >-
  Sometimes you may have collections whose generics aren't the same as the
  contents, by way of associated types; if you know the values are primitives,
  how do you go about casting the collections contents?
---

Imagine you have a generic collection that holds values, but the struct
mentioned in the generic is not the actual values of the holder _(astute
observers will realize this is a bit like `ArrowPrimitiveType` in `arrow`)._:

```
trait NumberLike {
  type Native;
}

impl NumberLike for Int64 {
  type Native = i64;
}

impl NumberLike for Float64 {
  type Native = f64;
}

#[derive(Debug, Clone, PartialEq)]
struct NumberLikeArray<A: NumberLike>(Vec<A::Native>);
```

Notice how the generic on our struct is different from the value stored, as specified by the trait.
You might think you can use this associated type to do primitive casts, like this:

```
fn cast_vec<A, B>(xs: NumberLikeArray<A>) -> NumberLikeArray<B>
where
  A: NumberLike,
  B: NumberLike,
{
  NumberLikeArray(xs.0.into_iter().map(|x| x as B::Native).collect())
}
```

But this won't work, the compiler tells us:

```
error[E0605]: non-primitive cast: `<A as NumberLike>::Native` as `<B as NumberLike>::Native`
  --> src/lib.rs:24:44
   |
24 |   NumberLikeArray(xs.0.into_iter().map(|x| x as B::Native).collect())
   |                                            ^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
```

This error helpfully informs us that `as` only works on primitive types or
specific trait objects on the right hand side of the keyword. With generics we
reduce the amount of duplicated code we need to write, but we also lose less
information about the values themselves. To gain back some information about
the types, we can put bounds on the generic values which will tell us about
whether or not there are associated types available to us
or functions we can run on said types.

The associated type for `NumberLike` could be anything, hence Rust won't let us
compile this code because it can't be 100% sure that `x` is a primitive value
or a specific trait object, nor can it confirm that `B::Native` is a primitive
type. Rust isn't going to try to figure out that information from all the trait
bounds it knows about during compilation, either. What we need is something
that can allow us to convert values but expressed as a trait; what about
`From`?

```
fn cast<A, B>(xs: NumberLikeArray<A>) -> NumberLikeArray<B>
where
  A: NumberLike,
  B: NumberLike,
  B::Native: From<A::Native>,
{
  NumberLikeArray(xs.0.into_iter().map(|x| x.into()).collect())
}
```

but we're going to hit another wall with this the moment we try to use it;

```
fn main() {
    let xs: NumberLikeArray<Int64> = NumberLikeArray(vec![12, 13, 14]);
    let ys: NumberLikeArray<Float64> = cast(xs);
    assert_eq!(ys.0, vec![12.0, 13.0, 14.0]);
}
```

there is no implementation for `From<i64> for f64` or the other way around!

```
error[E0277]: the trait bound `f64: std::convert::From<i64>` is not satisfied
  --> src/main.rs:30:40
   |
19 | fn cast<A, B>(xs: NumberLikeArray<A>) -> NumberLikeArray<B>
   |    ---- required by a bound in this
...
23 |   B::Native: From<A::Native>,
   |              --------------- required by this bound in `cast`
...
30 |     let ys: NumberLikeArray<Float64> = cast(xs);
   |                                        ^^^^ the trait `std::convert::From<i64>` is not implemented for `f64`
   |
   = help: the following implementations were found:
             <f64 as std::convert::From<f32>>
             <f64 as std::convert::From<i16>>
             <f64 as std::convert::From<i32>>
             <f64 as std::convert::From<i8>>
           and 3 others
```

This makes sense as `From` is really intended for cheap, infallible conversions
between types, and `as` is quite a blunt tool. `as` can perform truncations and
other changes to the data depending on the arguments. Converting from an `i64`
to an `f64` may seem odd, but we may want the behavior that `as` supports in
our program. You might reach to `impl From for f64` and `i64` respectively to
make this happen:


```
impl From<i64> for f64 {
    fn from(x: i64) -> f64 {
        x as f64
    }
}

impl From<i64> for f64 {
    fn from(x: i64) -> f64 {
        x as i64
    }
}
```

but alas another wall:

```
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src/main.rs:16:1
   |
16 | impl From<i64> for f64 {
   | ^^^^^---------^^^^^---
   | |    |             |
   | |    |             `f64` is not defined in the current crate
   | |    `i64` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead

error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src/main.rs:22:1
   |
22 | impl From<f64> for i64 {
   | ^^^^^---------^^^^^---
   | |    |             |
   | |    |             `i64` is not defined in the current crate
   | |    `f64` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead
```

Let's follow the advice the compiler has given us:

```
trait Cast<A> {
    fn cast(self) -> A;
}

impl Cast<i64> for f64 {
    fn cast(self) -> i64 {
        self as i64
    }
}

impl Cast<f64> for i64 {
    fn cast(self) -> f64 {
        self as f64
    }
}
```

and we'll update our `cast` function:

```
fn cast<A, B>(xs: NumberArray<A>) -> NumberArray<B>
where
    A: NumberLike,
    B: NumberLike,
    A::Native: Cast<B::Native>,
{
  NumberArray(xs.0.into_iter().map(|x| x.cast()).collect())
}
```

We could have easily swapped the argument order of `Cast::cast` just like
`From` and `Into`'s symmetry. The choice felt arbitrary here and I've picked to
have things feel like they say "A::Native supports casts into B::Native's" and
simply replace the `into` call we had earlier with `cast`. If we wanted to push
this further, we could also describe `NumberLikeArray` with a `Cast::cast`
implementation:

```
fn cast<A, B>(xs: NumberLikeArray<A>) -> NumberLikeArray<B>
where
  A: NumberLike,
  B: NumberLike,
  A::Native: Cast<B::Native>,
{
  NumberLikeArray(xs.0.into_iter().map(|x| x.cast()).collect())
}

impl<A, B> Cast<NumberLikeArray<B>> for NumberLikeArray<A>
where
  A: NumberLike,
  B: NumberLike,
  A::Native: Cast<B::Native>,
{
    fn cast(self) -> NumberLikeArray<B> {
        cast(self)
    }
}
```

This shows a way of writing method implementations I sometimes like to do. When
you define `Cast::cast` you get two functions for free: `Cast::cast(array)` and
`array.cast()`. I also like to have the option of doing a qualified import via
a module, s.t. someone could do `crate::convert::cast(array)` if they so
desired. With the above approach, you get all three. Here's the [full
listing](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=514a19b873cf4226f3ac7315550657ba)
to explore on your own.

The good news is, you don't have to do this 'from scratch' every time you start
a project, and you don't need to spin up your own crate because the `num` crate
already has a `NumCast` and `ToPrimitive` pair of buddy traits that do this.
The one major difference is that `NumCast::from` produces an `Option` that
we'll need to handle. We could flatmap this, causing items to go missing during
the cast, or we could make it error on first cast, like this:

```
fn cast<A, B>(xs: NumberLikeArray<A>) -> Option<NumberLikeArray<B>>
where
  A: NumberLike,
  B: NumberLike,
  A::Native: NumCast,
  B::Native: ToPrimitive + NumCast,
{
  Some(NumberLikeArray(xs.0.into_iter().map(|x| NumCast::from(x)).collect::<Option<Vec<B::Native>>>()?))
}

fn main() {
    let xs: NumberLikeArray<Int64> = NumberLikeArray(vec![12, 13, 14]);
    let ys: NumberLikeArray<Float64> = cast(xs).expect("could not cast array");
    assert_eq!(ys.0, vec![12.0, 13.0, 14.0]);
}
```

Using bounds to specify requirements on generics is a way of gaining back
information about the types we abstract over. Instead of working over all A's
and B's, we're specifically working over A's and B's that implement certain
charecteristics, and we can leverage those characteristics to transform values,
perform effects, or simply declare that some type has been marked or tagged, as
is the case with `Eq` and others.
