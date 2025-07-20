#![feature(c_variadic)]

use core::{ffi::*, ptr::null_mut};

extern "C" {
    fn asprintf(s: *mut *mut c_char, format: *const c_char, ...) -> c_int;
    fn free(p: *mut c_void);
}

unsafe extern "C" fn rust_fmt(str: *const c_char, mut args: ...) -> Box<(c_int, String)> {
    let mut s = String::new();
    let bytes_written = printf_compat::format(
        str,
        args.clone().as_va_list(),
        printf_compat::output::fmt_write(&mut s),
    );
    assert!(bytes_written >= 0);
    let mut s2 = std::io::Cursor::new(vec![]);
    assert_eq!(
        bytes_written,
        printf_compat::format(
            str,
            args.as_va_list(),
            printf_compat::output::io_write(&mut s2),
        )
    );
    assert_eq!(s.as_bytes(), s2.get_ref());
    Box::new((bytes_written, s))
}

macro_rules! c_fmt {
    ($format:literal $(, $p:expr)*) => {{
        let mut ptr = null_mut();
        let bytes_written = asprintf(&mut ptr, $format.as_ptr() $(, $p)*);
        assert!(bytes_written >= 0);
        let s: String = CStr::from_ptr(ptr).to_string_lossy().into();
        free(ptr.cast());
        (bytes_written, s)
    }};
}

macro_rules! assert_eq_fmt {
    ($format:literal $(, $p:expr)*) => {
        assert_eq!(
            c_fmt!($format $(, $p)*),
            *rust_fmt($format.as_ptr().cast(), $($p),*)
        );
    };
    ($format:literal $(, $p:expr)* => $expected:literal) => {
        let (bytes_written, s) = c_fmt!($format $(, $p)*);
        assert_eq!(s, $expected);
        assert_eq!((bytes_written, s), *rust_fmt($format.as_ptr().cast(), $($p),*));
        assert_eq!(usize::try_from(bytes_written).unwrap(), $expected.len());
    };
}

#[test]
fn test_plain() {
    unsafe {
        assert_eq_fmt!(c"abc");
        assert_eq_fmt!(c"");
        assert_eq_fmt!(c"%%");
        assert_eq_fmt!(c"%% def");
        assert_eq_fmt!(c"abc %%");
        assert_eq_fmt!(c"abc %% def");
        assert_eq_fmt!(c"abc %%%% def");
        assert_eq_fmt!(c"%%%%%%");
    }
}

#[test]
fn test_str() {
    unsafe {
        assert_eq_fmt!(c"hello %s", c"world");
        assert_eq_fmt!(c"hello %%%s", c"world");
        assert_eq_fmt!(c"%10s", c"world");
        assert_eq_fmt!(c"%.4s", c"world");
        assert_eq_fmt!(c"%10.4s", c"world");
        assert_eq_fmt!(c"%-10.4s", c"world");
        assert_eq_fmt!(c"%-10s", c"world");
        assert_eq_fmt!(c"%s", null_mut::<c_char>());
    }
}

#[test]
fn test_int() {
    unsafe {
        assert_eq_fmt!(c"% 0*i", 17, 23125);
        assert_eq_fmt!(c"% 010i", 23125);
        assert_eq_fmt!(c"% 10i", 23125);
        assert_eq_fmt!(c"% 5i", 23125);
        assert_eq_fmt!(c"% 4i", 23125);
        assert_eq_fmt!(c"%- 010i", 23125);
        assert_eq_fmt!(c"%- 10i", 23125);
        assert_eq_fmt!(c"%- 5i", 23125);
        assert_eq_fmt!(c"%- 4i", 23125);
        assert_eq_fmt!(c"%+ 010i", 23125);
        assert_eq_fmt!(c"%+ 10i", 23125);
        assert_eq_fmt!(c"%+ 5i", 23125);
        assert_eq_fmt!(c"%+ 4i", 23125);
        assert_eq_fmt!(c"%-010i", 23125);
        assert_eq_fmt!(c"%-10i", 23125);
        assert_eq_fmt!(c"%-5i", 23125);
        assert_eq_fmt!(c"%-4i", 23125);
    }
}

#[test]
fn test_octal() {
    unsafe {
        assert_eq_fmt!(c"% 010o", 23125);
        assert_eq_fmt!(c"% 10o", 23125);
        assert_eq_fmt!(c"% 5o", 23125);
        assert_eq_fmt!(c"% 4o", 23125);
        assert_eq_fmt!(c"%- 010o", 23125);
        assert_eq_fmt!(c"%- 10o", 23125);
        assert_eq_fmt!(c"%- 5o", 23125);
        assert_eq_fmt!(c"%- 4o", 23125);
        assert_eq_fmt!(c"%+ 010o", 23125);
        assert_eq_fmt!(c"%+ 10o", 23125);
        assert_eq_fmt!(c"%+ 5o", 23125);
        assert_eq_fmt!(c"%+ 4o", 23125);
        assert_eq_fmt!(c"%-010o", 23125);
        assert_eq_fmt!(c"%-10o", 23125);
        assert_eq_fmt!(c"%-5o", 23125);
        assert_eq_fmt!(c"%-4o", 23125);
    }
}

#[test]
fn test_hex() {
    unsafe {
        assert_eq_fmt!(c"% 010x", 23125);
        assert_eq_fmt!(c"% 10x", 23125);
        assert_eq_fmt!(c"% 5x", 23125);
        assert_eq_fmt!(c"% 4x", 23125);
        assert_eq_fmt!(c"%- 010x", 23125);
        assert_eq_fmt!(c"%- 10x", 23125);
        assert_eq_fmt!(c"%- 5x", 23125);
        assert_eq_fmt!(c"%- 4x", 23125);
        assert_eq_fmt!(c"%+ 010x", 23125);
        assert_eq_fmt!(c"%+ 10x", 23125);
        assert_eq_fmt!(c"%+ 5x", 23125);
        assert_eq_fmt!(c"%+ 4x", 23125);
        assert_eq_fmt!(c"%-010x", 23125);
        assert_eq_fmt!(c"%-10x", 23125);
        assert_eq_fmt!(c"%-5x", 23125);
        assert_eq_fmt!(c"%-4x", 23125);

        assert_eq_fmt!(c"%# 010x", 23125);
        assert_eq_fmt!(c"%# 10x", 23125);
        assert_eq_fmt!(c"%# 5x", 23125);
        assert_eq_fmt!(c"%# 4x", 23125);
        assert_eq_fmt!(c"%#- 010x", 23125);
        assert_eq_fmt!(c"%#- 10x", 23125);
        assert_eq_fmt!(c"%#- 5x", 23125);
        assert_eq_fmt!(c"%#- 4x", 23125);
        assert_eq_fmt!(c"%#+ 010x", 23125);
        assert_eq_fmt!(c"%#+ 10x", 23125);
        assert_eq_fmt!(c"%#+ 5x", 23125);
        assert_eq_fmt!(c"%#+ 4x", 23125);
        assert_eq_fmt!(c"%#-010x", 23125);
        assert_eq_fmt!(c"%#-10x", 23125);
        assert_eq_fmt!(c"%#-5x", 23125);
        assert_eq_fmt!(c"%#-4x", 23125);

        assert_eq_fmt!(c"% 010X", 23125);
        assert_eq_fmt!(c"% 10X", 23125);
        assert_eq_fmt!(c"% 5X", 23125);
        assert_eq_fmt!(c"% 4X", 23125);
        assert_eq_fmt!(c"%- 010X", 23125);
        assert_eq_fmt!(c"%- 10X", 23125);
        assert_eq_fmt!(c"%- 5X", 23125);
        assert_eq_fmt!(c"%- 4X", 23125);
        assert_eq_fmt!(c"%+ 010X", 23125);
        assert_eq_fmt!(c"%+ 10X", 23125);
        assert_eq_fmt!(c"%+ 5X", 23125);
        assert_eq_fmt!(c"%+ 4X", 23125);
        assert_eq_fmt!(c"%-010X", 23125);
        assert_eq_fmt!(c"%-10X", 23125);
        assert_eq_fmt!(c"%-5X", 23125);
        assert_eq_fmt!(c"%-4X", 23125);
    }
}

#[test]
fn test_float() {
    unsafe {
        assert_eq_fmt!(c"%f", 1234f64);
        assert_eq_fmt!(c"%.5f", 1234f64);
        assert_eq_fmt!(c"%.*f", 1234f64, 3);
    }
}

#[test]
fn test_char() {
    unsafe {
        assert_eq_fmt!(c"%c", b'a' as c_int);
        assert_eq_fmt!(c"%10c", b'a' as c_int);
        assert_eq_fmt!(c"%-10c", b'a' as c_int);
    }
}
