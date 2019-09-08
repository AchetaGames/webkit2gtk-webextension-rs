// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLFontElement(Object<webkit2_webextension_sys::WebKitDOMHTMLFontElement, webkit2_webextension_sys::WebKitDOMHTMLFontElementClass, DOMHTMLFontElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_font_element_get_type(),
    }
}

pub const NONE_DOMHTML_FONT_ELEMENT: Option<&DOMHTMLFontElement> = None;

pub trait DOMHTMLFontElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_face(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_size(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_face(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_size(&self, value: &str);

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_face_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFontElement>> DOMHTMLFontElementExt for O {
    fn get_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_font_element_get_color(self.as_ref().to_glib_none().0))
        }
    }

    fn get_face(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_font_element_get_face(self.as_ref().to_glib_none().0))
        }
    }

    fn get_size(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_font_element_get_size(self.as_ref().to_glib_none().0))
        }
    }

    fn set_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_font_element_set_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_face(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_font_element_set_face(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_size(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_font_element_set_size(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLFontElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLFontElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFontElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::color\0".as_ptr() as *const _,
                Some(transmute(notify_color_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_face_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_face_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLFontElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLFontElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFontElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::face\0".as_ptr() as *const _,
                Some(transmute(notify_face_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLFontElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLFontElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLFontElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                Some(transmute(notify_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLFontElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLFontElement")
    }
}
