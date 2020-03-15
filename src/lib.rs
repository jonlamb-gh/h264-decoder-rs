#![no_std]

// TODO - build.rs
// bindgen --use-core --ctypes-prefix=crate::ctypes bindgen/wrapper.h -o src/bindings.rs -- -Ih264_decoder/src/inc

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bindings;
pub use bindings::*;

pub mod ctypes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
