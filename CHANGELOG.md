# Changelog

## 0.3.0 (December 7, 2025)

* Fix compilation errors on recent nightlies due to changes in the std
  [`VaList` implementation](https://github.com/rust-lang/rust/pull/141980).
  <https://github.com/lights0123/printf-compat/pull/45>
* Switch to the 2024 edition.
  <https://github.com/lights0123/printf-compat/pull/46>

## 0.2.1 (July 20, 2025)

* Remove dependency on `alloc`.
  <https://github.com/lights0123/printf-compat/pull/25>
* Output `(null)` when a null pointer is formatted with `%s`.
  <https://github.com/lights0123/printf-compat/pull/31>
* Update to edition 2021.
  <https://github.com/lights0123/printf-compat/pull/33>

## 0.2.0 (July 14, 2025)

* Remove `cty` and `cstr_core` dependencies.
  <https://github.com/lights0123/printf-compat/pull/4>
* Improve integer conversions to match C's behavior.
  <https://github.com/lights0123/printf-compat/pull/11>
* Update `itertools` dependency to 0.14.0.
  <https://github.com/lights0123/printf-compat/pull/17>
* Update `bitflags` dependency to 2.9.1.
  <https://github.com/lights0123/printf-compat/pull/18>
