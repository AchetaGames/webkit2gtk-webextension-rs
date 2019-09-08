// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMObject;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMCSSValue(Object<webkit2_webextension_sys::WebKitDOMCSSValue, webkit2_webextension_sys::WebKitDOMCSSValueClass, DOMCSSValueClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_css_value_get_type(),
    }
}

pub const NONE_DOMCSS_VALUE: Option<&DOMCSSValue> = None;

pub trait DOMCSSValueExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_css_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_css_value_type(&self) -> libc::c_ushort;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_css_text(&self, value: &str) -> Result<(), glib::Error>;

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_css_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMCSSValue>> DOMCSSValueExt for O {
    fn get_css_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_css_value_get_css_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_css_value_type(&self) -> libc::c_ushort {
        unsafe {
            webkit2_webextension_sys::webkit_dom_css_value_get_css_value_type(self.as_ref().to_glib_none().0)
        }
    }

    fn set_css_text(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_css_value_set_css_text(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_css_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMCSSValue, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMCSSValue>
        {
            let f: &F = &*(f as *const F);
            f(&DOMCSSValue::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::css-text\0".as_ptr() as *const _,
                Some(transmute(notify_css_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_css_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_css_value_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMCSSValue, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMCSSValue>
        {
            let f: &F = &*(f as *const F);
            f(&DOMCSSValue::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::css-value-type\0".as_ptr() as *const _,
                Some(transmute(notify_css_value_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMCSSValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMCSSValue")
    }
}
