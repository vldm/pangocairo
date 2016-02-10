extern crate libc;
extern crate pango;
extern crate cairo;
extern crate gobject_sys;
extern crate glib;

pub type Color = (u16, u16, u16);

mod ffi;
pub mod attribute;
pub mod attribute_list;
pub mod layout;
