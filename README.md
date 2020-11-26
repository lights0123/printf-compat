# printf-compat

[![Crates.io](https://img.shields.io/crates/v/printf-compat.svg)](https://crates.io/crates/printf-compat)
[![Docs.rs](https://docs.rs/printf-compat/badge.svg)](https://docs.rs/printf-compat)

`printf` reimplemented in Rust

This is a complete reimplementation of `printf` in Rust, using the unstable
(i.e. **requires a Nightly compiler**) `c_variadic` feature.

- [Many C][sigrok-log] [libraries][libusb-log] provide a way to provide a
  custom log callback. With this crate, you can provide a pure Rust option,
  and do whatever you want with it. Log it to the console,
- If you're writing a Rust-first program for a microcontroller and need to
  interface with a C library, you might not *have* a libc and have to
  reimplement it yourself. If it uses `printf`, use this crate to easily add
  your own output. [`core::fmt`] too big? No problem! Write your own
  formatting code, or use a minimal formatting library like [`ufmt`] or
  [`defmt`]. Don't need *every* single option given by `printf` format
  strings? No problem! Just don't implement it.
- Likewise, if you're using `wasm32-unknown-unknown` instead of emscripten
  (as wasm-bindgen is only compatible with the former), you have no libc. If
  you want to interface with a C library, you'll have to do it all yourself.
  With this crate, that turns into 5 lines instead of hundreds for `printf`.

## Benefits

### ⚒ Modular

printf-compat lets you pick how you want to output a message. Use
pre-written adapters for [`fmt::Write`][output::fmt_write] (like a
[`String`]) or [`io::Write`][output::io_write] (like
[`io::stdout()`][std::io::stdout]), or implement your own.

### 🔬 Small

This crate is `no_std` compatible (`printf-compat = { version = "0.1",
default-features = false }` in your Cargo.toml). The main machinery doesn't
require the use of [`core::fmt`], and it can't panic.

### 🔒 Safe (as can be)

Of course, `printf` is *completely* unsafe, as it requires the use of
`va_list`. However, outside of that, all of the actual string parsing is
written in completely safe Rust. No buffer overflow attacks!

The `n` format specifier, which writes to a user-provided pointer, is
considered a serious security vulnerability if a user-provided string is
ever passed to `printf`. It *is* supported by this crate; however, it
doesn't do anything by default, and you'll have to explicitly do the writing
yourself.

### 🧹 Tested

A wide [test suite] is used to ensure that many different possibilities are
identical to glibc's `printf`. [Differences are documented][output::fmt_write#differences].

## Getting Started

Start by adding the unstable feature:

```rust
#![feature(c_variadic)]
```

Now, add your function signature:

```rust
use cty::{c_char, c_int};

#[no_mangle]
unsafe extern "C" fn c_library_print(str: *const c_char, mut args: ...) -> c_int {
    todo!()
}
```

If you have access to [`std`], i.e. not an embedded platform, you can use
[`std::os::raw`] instead of [`cty`]. Also, think about what you're doing:

- If you're implenting `printf` *because you don't have one*, you'll want to
  call it `printf` and add `#[no_mangle]`.
- Likewise, if you're creating a custom log function for a C library and it
  expects to call a globally-defined function, keep `#[no_mangle]` and
  rename the function to what it expects.
- On the other hand, if your C library expects you to call a function to
  register a callback ([example 1][sigrok-log], [example 2][libusb-log]),
  remove `#[no_mangle]`.

Now, add your logic:

```rust
#
use printf_compat::{format, output};
let mut s = String::new();
let bytes_written = format(str, args.as_va_list(), output::fmt_write(&mut s));
println!("{}", s);
bytes_written
```

Of course, replace [`output::fmt_write`] with whatever you like—some are
provided for you in [`output`]. If you'd like to write your own, follow
their function signature: you need to provide a function to [`format()`]
that takes an [`Argument`] and returns the number of bytes written (although
you don't *need* to if your C library doesn't use it) or -1 if there was an
error.

[sigrok-log]: https://sigrok.org/api/libsigrok/unstable/a00074.html#ga4240b8fe79be72ef758f40f9acbd4316
[libusb-log]: http://libusb.sourceforge.net/api-1.0/group__libusb__lib.html#ga2efb66b8f16ffb0851f3907794c06e20
[test suite]: https://github.com/lights0123/printf-compat/blob/master/src/tests.rs
[`ufmt`]: https://docs.rs/ufmt/
[`defmt`]: https://defmt.ferrous-systems.com/

License: MIT OR Apache-2.0
