#![allow(internal_features)]
#![feature(auto_traits, intrinsics, lang_items, no_core, transparent_unions)]
#![no_implicit_prelude]
#![no_core]
#![no_std]

pub mod impls;
pub mod items;

fn main() {
    let mut y = 23;
    let x = &mut y;
    let z = x;
    x;
    z;
    // yup, this works. and zero unsafe code anywhere
}
