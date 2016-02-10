use std::ops::Range;
use std::mem;

use pango;
use Color;

use libc::c_uint;


pub struct PangoAttribute(*mut pango::ffi::PangoAttribute);

impl PangoAttribute {
    pub fn from_fg_color(color: Color) -> PangoAttribute {
        PangoAttribute (
            unsafe {
                pango::ffi::pango_attr_foreground_new(color.0, color.1, color.2)
            })
    }
    
    pub fn from_bg_color(color: Color) -> PangoAttribute {
        PangoAttribute (
            unsafe {
                pango::ffi::pango_attr_background_new(color.0, color.1, color.2)
            })
    }
    
    pub fn new_underline() -> PangoAttribute {
        PangoAttribute (
            unsafe {
                pango::ffi::pango_attr_underline_new(
                                pango::ffi::PangoUnderline::Single)
            })
    }
    
    pub fn new_bold() -> PangoAttribute {
        PangoAttribute (
            unsafe {
                pango::ffi::pango_attr_weight_new(
                                pango::ffi::PangoWeight::Bold)
            })
    }
    
    pub fn new_italic() -> PangoAttribute {
        PangoAttribute (
            unsafe {
                pango::ffi::pango_attr_style_new(
                                pango::ffi::PangoStyle::Italic)
            })
    }
    
    pub fn set_range(&mut self, range: &Range<usize>) -> &mut PangoAttribute {
        unsafe {
            (*self.0).start_index = range.start as c_uint;
            (*self.0).end_index = range.end as c_uint;
        }
        self
    }
    
    pub fn range(mut self, range: &Range<usize>) -> PangoAttribute {
        self.set_range(range);
        self
    }
    
    pub unsafe fn destroy(self) -> *mut pango::ffi::PangoAttribute {
        let x = self.0;
        mem::forget(self);
        x
    }
}

impl Drop for PangoAttribute {
    fn drop(&mut self) {
        unsafe {
            pango::ffi::pango_attribute_destroy(self.0);
        }
    }
}
