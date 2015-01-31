#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(missing_copy_implementations)]
#![deny(raw_pointer_derive)]

#![feature(libc)]

extern crate libc;

use libc::{
    c_char,
    c_int
};

#[allow(dead_code)]
enum Sass_Output_Style {
    SASS_STYLE_NESTED,
    SASS_STYLE_EXPANDED,
    SASS_STYLE_COMPACT,
    SASS_STYLE_COMPRESSED
}

extern "C" {
 pub fn sass2scss (sass: * const c_char , options: c_int) -> * const c_char;


 pub fn sass2scss_version() -> * const c_char;

}
