use std::iter::FromIterator;
use std::mem;

use pango;
use attribute::PangoAttribute;

pub struct PangoAttributeList(*mut pango::ffi::PangoAttrList);

impl PangoAttributeList {
    pub fn new() -> PangoAttributeList {
        PangoAttributeList(
            unsafe {
                pango::ffi::pango_attr_list_new() 
        })
    }
    
    pub fn push(&mut self, attribute: PangoAttribute) {
        unsafe {
            pango::ffi::pango_attr_list_insert(self.0, attribute.destroy());
        }
    }
    pub unsafe fn destroy(self) -> *mut pango::ffi::PangoAttrList {
        let x = self.0;
        mem::forget(self);
        x
    }
}

impl Drop for PangoAttributeList {
    fn drop(&mut self) {
        unsafe {  pango::ffi::pango_attr_list_unref(self.0); }
    }
}

impl FromIterator<PangoAttribute> for PangoAttributeList {
    fn from_iter<T: IntoIterator<Item=PangoAttribute>>(iterator: T) -> PangoAttributeList {
        let mut attributes = PangoAttributeList::new();
        for attribute in iterator {
            attributes.push(attribute);
        }
        attributes
    }
}
