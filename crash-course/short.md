10/11/2023

11:09 PM

let's try this one, 10 mins

https://www.youtube.com/watch?v=br3GIIQeefY

11:16 PM

my mind man... it's everywhere right now, feeling good feeling great

11:21 PM

omg man... I keep getting distracted lmao

alright let's go...

do I have rust installed already? pretty sure I do

`let x: i32 = 42;`

signed 32 bit integer... how do you know if it's unsigned... oh `u` nice

man I studied this shit so long ago... 2's compliment and what not ahhhhhhh it's gone I'm a CRUD simpleton

oh it's split in half with negative half, unsigned is the whole thing but starting from 0

python promotes the integer on runtime as needed

lol just use `i32` for everything

i32 = long in c, int in java

rust uses i32 as default

uninitialized variables are not accessible

`_` throw away huh

`let _ = get_thing();` throw away result

tuples

fixed length collections of values of different types

explicit type annotation

`let pair: (char, i32) = ('a', 17);`

f you Jacob focus!

man this is so weird

`let (_, right) = slice.split_at(middle);` so this is like an array (no) of two things but the first one is discarded wtf

omg that's right, mutable lmao I'm committing bro, I can't just be a front end scrub forever

semicolons have to be there

vectors... `vec![...]` hmm

wth is `fold` lmao... ahh reducer equivalent hmmm

nice functions

```
fn method_name() -> i32 {
  4
}
```

arrow is return type

interesting this block scoping with brackets hmm

no global scope huh

tail of the block...

```
  let x = {
    ... crap
    tail
  }
```

I guess that's like a return with no return keybword

ahh naruhodo that's why there's no semicolon above otherwise it's `return 4;`

damn applies to if/else too

match like switch case nice

dot-style method/value calling

oh boy here we go `::` this mf

`std::cmp::min(3, 8)` crate::module::function

also `use std::cmp::min`

11:44 PM

making good progress, it's funny that's only 4.5 minutes of video lmao

hmm this is interesting  `"amos".len()` equal to `str::len("amos")`

`struct`

always `structin` dat ass, you be so sick and f'n tired... `structin` dat ass

ooh lightweight new type mf'n YZAK!

ehh let me research this

example I see is

```
struct Number {
  odd: bool,
  value: i32,
}
```

I guess that's just made up

```
struct Human {
  legs: true,
  arms: true,
  wings: false,
  name: String
}
```

idk

oh yeah, names and types of the pieces of data, cool

12:00 AM

alright please Jacob finish it

match `_` is catch all/default

ooh it's like a prototype in JavaScript

this thing

```
impl Number {
  fn is_positive...
}
```

cool

here we go immutability, okay use he `mut` in front of the variable

oh man... generic typing

hmm generic structs same "inner thing" types

`let mut v1 = Vec::new()` heap-allocated array

interesting the type is determined by what you push into it

`v1.push(bool)`

hmm seen this before but yeah `let v1 = vec![1, 2, 3];`

`!` is macro? eg. `name![]`

oh sh... `println!` now I see, I was blind but now I see

12:09 AM

`panic!` (at the disco) aha someone else had this idea nice

omg wtf is this...

`Some` 

`None`

variants of enum

holy f

https://stackoverflow.com/a/24772148/2710227

can contain something or nothing and yeah above did talk about `Option`

alright I guess TL;DR is keep this in mind if you think something will fail

`unwrap` gets the value of the complex type

https://stackoverflow.com/a/71231484/2710227

Oh damn okay `None`` and `Some`` are in `Option`

Ahh nice `Result` similar but `Ok` vs. `Err`

functions that can fail typically return a result okay

damn I don't understand this

```
let s1 = str::from_utf8(...)
```

`valid_up_to`

man this stuff is hard to read, can my smooth brain handle it, it's like TypeScript I still have that cognitive overhead reading it

"converts a slice of bytes into a string slice..."

it's part of a string okay

"if you want to panic in case of failure, you can unwrap"

`.expect` for custom error message

unwrapping the value inside a result if it's okay or returning an error

`let s = str::from_utf8(melon)?;` it's the question mark

12:25 AM

holy sh I'm still not done... this is so dense

watch me turn $75K into $0 baby let's goooooooo

whoa `1..` will go from 1 to infinity damn

range iterator `(0..).contains(&100)`

`(..=20)` 20 or less than 20 huh

contains check that's cool

oh nice, vector loop

```
  for i in vec![52, 49, 21] {
    ...
  }
```

and the string interpolation `println!("huh {}", i)`

this is a slice

`&[52, 49, 21]`

"unicode scalar value" damn boy... direction no size what is that

"single number or value" hmm

whoa this is nice

```
  .filter(|c| c.is_lowercase())
  .flat_map(|c| c.to_uppercase())
```

reduces the source array then makes a new array damn.... cool bracket location threw me off alright nice

`cargo build`

traits shared behavior huh

I seent that last time I reviewed OOP

alright this was good
