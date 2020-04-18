---
title: "The Lowly Assert: Roundtrips"
author: Ryan James Spencer
date: 2019-11-02T10:37:16.048817270+00:00
tags: [the lowly assert, rust]
summary: >-
  Data "roundtrips" when it goes from one value, to another, and back to the same
  value without any data loss, gain, or corruption. If you write code, you have
  probably roundtripped JSON, YAML, TOML, or some other serialization format in
  your time. You have also probably written versions of functions that do a
  similar 'cycle' of some data. Any time you care about data being the same after
  it's gone through the ringer, you want to write a roundtrip test.
---

Data "roundtrips" when it goes from one value, to another, and back to the same
value without any data loss, gain, or corruption. If you write code, you have
probably roundtripped JSON, YAML, TOML, or some other serialization format in
your time. You have also probably written versions of functions that do a
similar 'cycle' of some data. Any time you care about data being the same after
it's gone through the ringer, you want to write a roundtrip test.

Pretend we have a system where data comes in as JSON. We slurp up that JSON into
a type using `serde` (rust's idiomatic, type-driven serialization +
deserialization library). That data might later go onto being a type unrelated
to JSON, so we might write some `From` instances. This will be our adaptive
layer so we can keep the shape of the JSON and our core types distinct. I
mention this approach briefly in my post ["Safely Shape Code with
Curtains"](https://www.justanotherdot.com/posts/safely-shape-code-with-curtains.html).
The `From` instance would normally be trivial, but we don't want the JSON layer
and the core types to look the same, do we? That would make the point of the
JSON types moot:

```
struct JsonType {
  names: Option<Vec<String>>,
  ids: Vec<i64>,
}

struct CoreType {
  names: Vec<String>,
  ids: Vec<i64>,
}

impl From<JsonType> for CoreType {
  fn from(x: JsonType) -> Self {
    Self {
      names: x.names.unwrap_or(vec![]),
      ids: x.ids,
    }
  }
}

impl From<CoreType> for JsonType {
  fn from(x: CoreType) -> Self {
    Self {
      names: Some(x.unwrap())
      ids: x.ids,
    }
  }
}
```

We could test each direction in isolation, but that would mask the actual
mistake here. Can you spot it? The roundtrip test in a property based testing
context would find the failure quite quickly. I'll do it by hand here to
demonstrate the mistake:

```
let beg = JsonType {
  names: None,
  ids: vec![1,2,3],
};
let roundtrip_fwd: CoreType = expected.into();
let end: JsonType = roundtrip_fwd.into();
assert_eq!(beg, end);
```

When the data comes back to the JSON layer, unless we tell `serde` that empty
vectors are always `None`s for this field, we've now lost information. Clients
might care a lot that their POST of some JSON for creating an entity in this
make-believe system is non-symmetric. Developers might be going between the core
and the JSON types regularly, and they may even be using the JSON types to write
to disk, too, which would mean what was passed up from the client is now not the
same as what is stored.

We can extrapolate this sort of information loss or corruption to other
conversions. If you author an automatic code formatter, say `prettier`, `gofmt`,
`mix fmt`, `rustfmt`, and so on, you'd want to make sure that any time you save
a file and the formatter runs that your code is still the same code,
semantically, as it was before saving the file. Although things might possibly
look the same by eye, it could be another program entirely when run.

### Food for thought

A quick refresher on functions.

* Functions can be seen as **mappings** from one type of value to another
* All possible values that can go into our mapping are known as the **domain** of
  a function
* The set of all possible values our mapping can produce is called the **codomain**
* The set of all values the mapping realistically produces is called the
    **range** or **image**

Ok, onto the concepts with fancy names:

* An **injective** mapping is when a mapping goes from values in the domain to
_unique_ values in the codomain.

* A **surjective** mapping is when a mapping goes from values in the domain to
_every_ value in the codomain, even if some mappings overlap.

* A **bijective** mapping is **simultaneously injective and surjective** which
means every value in the domain maps to every value in the codomain exactly
once.

Why does this matter?

Bijective mappings give you an inverse function for free. If you are a value in
the codomain and you know the mapping is bijective, then you can be sure that
there must be one, and only one, value where you came from in the domain.
One could [prove bijections](https://math.stackexchange.com/a/165440/156419)
using classical means but we don't need to for production usage. Instead, it
suffices to simply show the action going forwards and backwards.
