// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLFrameSetElement(Object<webkit2_webextension_sys::WebKitDOMHTMLFrameSetElement, webkit2_webextension_sys::WebKitDOMHTMLFrameSetElementClass, DOMHTMLFrameSetElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_frame_set_element_get_type(),
    }
}

pub const NONE_DOMHTML_FRAME_SET_ELEMENT: Option<&DOMHTMLFrameSetElement> = None;

pub trait DOMHTMLFrameSetElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_cols(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_rows(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_cols(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_rows(&self, value: &str);

    fn connect_property_cols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFrameSetElement>> DOMHTMLFrameSetElementExt for O {
    fn get_cols(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_frame_set_element_get_cols(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_rows(&self) -> Option<GString> {
        unsafe {
            from_glib_full(
                webkit2_webextension_sys::webkit_dom_html_frame_set_element_get_rows(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn set_cols(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_frame_set_element_set_cols(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_rows(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_frame_set_element_set_rows(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn connect_property_cols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cols_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFrameSetElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFrameSetElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFrameSetElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cols\0".as_ptr() as *const _,
                Some(transmute(notify_cols_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rows_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_webextension_sys::WebKitDOMHTMLFrameSetElement,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DOMHTMLFrameSetElement>,
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFrameSetElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rows\0".as_ptr() as *const _,
                Some(transmute(notify_rows_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLFrameSetElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLFrameSetElement")
    }
}
