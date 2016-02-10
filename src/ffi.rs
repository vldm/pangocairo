use cairo;
use pango;

#[link(name = "pangocairo-1.0")]
extern {
    pub fn pango_cairo_create_layout(cr: *mut cairo::ffi::cairo_t) -> *mut pango::ffi::PangoLayout;
    pub fn pango_cairo_update_layout(cr: *mut cairo::ffi::cairo_t, layout: *mut pango::ffi::PangoLayout);
    pub fn pango_cairo_show_layout(cr: *mut cairo::ffi::cairo_t, layout: *mut pango::ffi::PangoLayout);
}
