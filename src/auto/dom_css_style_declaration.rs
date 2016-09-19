// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use DOMCSSRule;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMCSSStyleDeclaration(Object<ffi::WebKitDOMCSSStyleDeclaration>);

    match fn {
        get_type => || ffi::webkit_dom_css_style_declaration_get_type(),
    }
}

impl DOMCSSStyleDeclaration {
    pub fn get_css_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_css_text(self.to_glib_none().0))
        }
    }

    //pub fn get_length(&self) -> /*Unimplemented*/Fundamental: ULong {
    //    unsafe { TODO: call ffi::webkit_dom_css_style_declaration_get_length() }
    //}

    pub fn get_parent_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_parent_rule(self.to_glib_none().0))
        }
    }

    pub fn get_property_priority(&self, propertyName: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_priority(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    pub fn get_property_shorthand(&self, propertyName: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_shorthand(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    pub fn get_property_value(&self, propertyName: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_value(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    pub fn is_property_implicit(&self, propertyName: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_css_style_declaration_is_property_implicit(self.to_glib_none().0, propertyName.to_glib_none().0))
        }
    }

    //pub fn item(&self, index: /*Unimplemented*/Fundamental: ULong) -> Option<String> {
    //    unsafe { TODO: call ffi::webkit_dom_css_style_declaration_item() }
    //}

    //pub fn remove_property(&self, propertyName: &str, error: /*Ignored*/Option<Error>) -> Option<String> {
    //    unsafe { TODO: call ffi::webkit_dom_css_style_declaration_remove_property() }
    //}

    //pub fn set_css_text(&self, value: &str, error: /*Ignored*/Option<Error>) {
    //    unsafe { TODO: call ffi::webkit_dom_css_style_declaration_set_css_text() }
    //}

    //pub fn set_property(&self, propertyName: &str, value: &str, priority: &str, error: /*Ignored*/Option<Error>) {
    //    unsafe { TODO: call ffi::webkit_dom_css_style_declaration_set_property() }
    //}
}
