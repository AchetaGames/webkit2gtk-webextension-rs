// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct DOMCSSValue(Object<ffi::WebKitDOMCSSValue, ffi::WebKitDOMCSSValueClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_css_value_get_type(),
    }
}

pub const NONE_DOMCSS_VALUE: Option<&DOMCSSValue> = None;

pub trait DOMCSSValueExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_value_get_css_text")]
    fn css_text(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_value_get_css_value_type")]
    fn css_value_type(&self) -> libc::c_ushort;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_value_set_css_text")]
    fn set_css_text(&self, value: &str) -> Result<(), glib::Error>;

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_css_value_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DOMCSSValue>> DOMCSSValueExt for O {
    fn css_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_value_get_css_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn css_value_type(&self) -> libc::c_ushort {
        unsafe { ffi::webkit_dom_css_value_get_css_value_type(self.as_ref().to_glib_none().0) }
    }

    fn set_css_text(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_value_set_css_text(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_property_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_css_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMCSSValue,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMCSSValue>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMCSSValue::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::css-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_css_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_css_value_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_css_value_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMCSSValue,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMCSSValue>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMCSSValue::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::css-value-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_css_value_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMCSSValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMCSSValue")
    }
}
