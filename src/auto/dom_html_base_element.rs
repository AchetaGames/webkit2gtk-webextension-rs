// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLBaseElement(Object<ffi::WebKitDOMHTMLBaseElement>): DOMElement;

    match fn {
        get_type => || ffi::webkit_dom_html_base_element_get_type(),
    }
}

impl DOMHTMLBaseElement {
    pub fn get_href(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_element_get_href(self.to_glib_none().0))
        }
    }

    pub fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_element_get_target(self.to_glib_none().0))
        }
    }

    pub fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_element_set_href(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}