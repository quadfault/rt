# rt

A ray tracer, being slowly and carefully developed over time.

This is based on the ray tracer in C++ from the book Ray Tracing in One
Weekend by Peter Shirley.

## Compiling and running

To compile this project, you will need to use nightly Rust (the current beta
might also work.) Run:

```
cargo build --release
```

(Don't try a debug build; it's way too slow.) Then try:

```
time $(target/release/rt > test.ppm)
```

where `test.ppm` is the output image. (This will take a while.)
