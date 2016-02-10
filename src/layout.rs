use std::ffi::CString;
use std::mem;

use pango;
use gobject_sys;
use glib::translate::ToGlibPtr;
use cairo::Context;
use ffi;

use attribute::PangoAttribute;
use attribute_list::PangoAttributeList;

use libc::{c_char, c_int};

#[allow(non_camel_case_types)]
pub struct PangoLayout_t ( *mut pango::ffi::PangoLayout);
pub struct PangoLayout<'a> {
    context: &'a mut Context,
    layout: PangoLayout_t,
    text: String,
    len: usize,
    attributes: PangoAttributeList,

}
impl<'a> PangoLayout<'a> {

    pub fn new(context: &'a mut Context, font_name: &str) -> PangoLayout<'a> {
        let font_name = CString::new(font_name.as_bytes()).unwrap().as_ptr();
        unsafe{
            let layout = ffi::pango_cairo_create_layout(
                                (&*context).to_glib_none().0);
            let font = pango::ffi::pango_font_description_from_string(
                                    font_name);
            pango::ffi::pango_layout_set_font_description(layout, font);
            pango::ffi::pango_font_description_free(font);
            PangoLayout {
                layout: PangoLayout_t(layout),
                context: context,
                text: String::new(),
                attributes: PangoAttributeList::new(),
                len: 0,
            }
        }
    }
    
    pub fn push<'c, 'b:'c> (&'b mut self, text: &str, attributes: Vec<PangoAttribute>)
                -> &'c mut PangoLayout<'a> {
        for attribute in attributes {
            self.attributes.push(attribute.range(&(self.len..self.len + text.len())));
        }
        self.len += text.len();
        self.text.push_str(text);
        self
    }
    
    pub fn update<'c, 'b:'c> (&'b mut self) -> &'c mut PangoLayout<'a> {
        let attributes = mem::replace(&mut self.attributes,
                                 PangoAttributeList::new());
        unsafe {
            pango::ffi::pango_layout_set_attributes(self.layout.0,
                         attributes.destroy());
            pango::ffi::pango_layout_set_text(self.layout.0,
                         self.text.as_ptr() as *const c_char,
                         self.text.len() as c_int);
            ffi::pango_cairo_update_layout((&*self.context).to_glib_none().0,
                         self.layout.0);

        }
        self
    }
    
    pub fn get_size(&self) -> (isize, isize) {
        let mut height: c_int = unsafe { mem::zeroed() };
        let mut width: c_int = unsafe { mem::zeroed() };
        unsafe {
            pango::ffi::pango_layout_get_size (self.layout.0, &mut width, &mut height);
        }
        (width as isize, height as isize)
    }
    
    pub fn show<'c, 'b:'c> (&'b mut self) -> &'c mut PangoLayout<'a> {
        unsafe {
            ffi::pango_cairo_show_layout((&*self.context).to_glib_none().0,
                 self.layout.0);
        }
        self
    }
}

impl Drop for PangoLayout_t {
    fn drop(&mut self) {
        unsafe {
            gobject_sys::g_object_unref(mem::transmute(self.0));
        }
    }
}
