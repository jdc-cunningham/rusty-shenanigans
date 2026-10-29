Not much here just talking about changing or defining types

Casting

The keyword `as` is casting

`let character = integer as char;`

`from` and `into` for conversion

`let big_num: i32 = i32::from(small_num);`

What is `::`

namespace path separator

`.` used to access fields or methods

literals

```
println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
```

I also saw `&` is a reference

modules

`size_of_val` function defined in `mem` module, `mem` module defined in `std` crate

type inference

```
let elem = 5u8;
let mut vec = Vec::new();
vec.push(elem); // inferred from 5u8 above
```

aliasing

```
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;
```

All are the same but different names "aliases"
