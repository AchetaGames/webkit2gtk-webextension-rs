// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
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

glib::wrapper! {
    pub struct DOMHTMLPreElement(Object<ffi::WebKitDOMHTMLPreElement, ffi::WebKitDOMHTMLPreElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_pre_element_get_type(),
    }
}

pub const NONE_DOMHTML_PRE_ELEMENT: Option<&DOMHTMLPreElement> = None;

pub trait DOMHTMLPreElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_pre_element_get_width")]
    fn width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_pre_element_get_wrap")]
    fn wraps(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_pre_element_set_width")]
    fn set_width(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_html_pre_element_set_wrap")]
    fn set_wrap(&self, value: bool);

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLPreElement>> DOMHTMLPreElementExt for O {
    fn width(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_html_pre_element_get_width(self.as_ref().to_glib_none().0) }
    }

    fn wraps(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_pre_element_get_wrap(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_width(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_pre_element_set_width(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_wrap(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_pre_element_set_wrap(
                self.as_ref().to_glib_none().0,
                value.to_glib(),
            );
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMHTMLPreElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMHTMLPreElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLPreElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wrap_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMHTMLPreElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DOMHTMLPreElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLPreElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wrap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wrap_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLPreElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLPreElement")
    }
}
