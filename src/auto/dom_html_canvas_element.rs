// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
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
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLCanvasElement(Object<webkit2_webextension_sys::WebKitDOMHTMLCanvasElement, webkit2_webextension_sys::WebKitDOMHTMLCanvasElementClass, DOMHTMLCanvasElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_canvas_element_get_type(),
    }
}

pub const NONE_DOMHTML_CANVAS_ELEMENT: Option<&DOMHTMLCanvasElement> = None;

pub trait DOMHTMLCanvasElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_height(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_height(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_width(&self, value: libc::c_long);

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLCanvasElement>> DOMHTMLCanvasElementExt for O {
    fn get_height(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_canvas_element_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_canvas_element_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn set_height(&self, value: libc::c_long) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_canvas_element_set_height(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_width(&self, value: libc::c_long) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_canvas_element_set_width(self.as_ref().to_glib_none().0, value);
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLCanvasElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLCanvasElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLCanvasElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::height\0".as_ptr() as *const _,
                Some(transmute(notify_height_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLCanvasElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLCanvasElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLCanvasElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                Some(transmute(notify_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLCanvasElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLCanvasElement")
    }
}
