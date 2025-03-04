// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMNode;
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
    pub struct DOMAttr(Object<ffi::WebKitDOMAttr, ffi::WebKitDOMAttrClass>) @extends DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_attr_get_type(),
    }
}

pub const NONE_DOM_ATTR: Option<&DOMAttr> = None;

pub trait DOMAttrExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_attr_get_name")]
    fn get_name(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_attr_get_owner_element")]
    fn get_owner_element(&self) -> Option<DOMElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_attr_get_specified")]
    fn get_specified(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_attr_get_value")]
    fn get_value(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_attr_set_value")]
    fn set_value(&self, value: &str) -> Result<(), glib::Error>;

    fn connect_property_local_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_namespace_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_owner_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_prefix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_specified_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMAttr>> DOMAttrExt for O {
    fn get_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_owner_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_attr_get_owner_element(self.as_ref().to_glib_none().0))
        }
    }

    fn get_specified(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_attr_get_specified(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_value(self.as_ref().to_glib_none().0))
        }
    }

    fn set_value(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_attr_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_local_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMAttr>
        {
            let f: &F = &*(f as *const F);
            f(&DOMAttr::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::local-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_local_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMAttr>
        {
            let f: &F = &*(f as *const F);
            f(&DOMAttr::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_namespace_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_namespace_uri_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMAttr>
        {
            let f: &F = &*(f as *const F);
            f(&DOMAttr::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::namespace-uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_namespace_uri_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_owner_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_element_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMAttr>
        {
            let f: &F = &*(f as *const F);
            f(&DOMAttr::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::owner-element\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_owner_element_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_prefix_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_prefix_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMAttr>
        {
            let f: &F = &*(f as *const F);
            f(&DOMAttr::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::prefix\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_prefix_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_specified_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_specified_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMAttr>
        {
            let f: &F = &*(f as *const F);
            f(&DOMAttr::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::specified\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_specified_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMAttr, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMAttr>
        {
            let f: &F = &*(f as *const F);
            f(&DOMAttr::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_value_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMAttr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMAttr")
    }
}
