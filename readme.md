Generate a random rgba()-formatted color.

#### About

Inspired by "[random-rgb-color](https://github.com/mrmrs/random-rgb-color/tree/main)" by [mrmrs](https://mrmrs.cc/), which was in turn inspired by [random-hex-color](http://github.com/johno/random-hex-color) by [John Otander](https://www.johno.com/).

This is a simple project for learning idiomatic Rust, publishing Rust projects, and how to approximate other programs using Rust.

#### Intended Use

It can be used to generate a string such as `rgba(179, 134, 103, 0.33)`, or a rust struct that looks like this:

```rust
RandomColor { 
    red: 189, 
    green: 138, 
    blue: 54, 
    alpha: 0.79 
}
```

#### Next steps

- [ ] Write actual docs
- [ ] Update the functions to accomodate for default values if min/max values aren't provided
- [ ] Compress the two functions for producing Strings and RandomColor structs into a single expression, possibly using enums?
- [ ] Improve the commenting, and learn how doc comments work
- [ ] Make it more idiomatic, squeeze out performance, etc.