// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use DOMStyleSheet;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLStyleElement(Object<webkit2_webextension_sys::WebKitDOMHTMLStyleElement, webkit2_webextension_sys::WebKitDOMHTMLStyleElementClass, DOMHTMLStyleElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_style_element_get_type(),
    }
}

pub const NONE_DOMHTML_STYLE_ELEMENT: Option<&DOMHTMLStyleElement> = None;

pub trait DOMHTMLStyleElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_disabled(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_media(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_sheet(&self) -> Option<DOMStyleSheet>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_type_attr(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_disabled(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_media(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_type_attr(&self, value: &str);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLStyleElement>> DOMHTMLStyleElementExt for O {
    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_html_style_element_get_disabled(self.as_ref().to_glib_none().0))
        }
    }

    fn get_media(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_style_element_get_media(self.as_ref().to_glib_none().0))
        }
    }

    fn get_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_style_element_get_sheet(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_style_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_style_element_set_disabled(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_media(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_style_element_set_media(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_style_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `type` getter")
        }
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"type\0".as_ptr() as *const _, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_disabled_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLStyleElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLStyleElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::disabled\0".as_ptr() as *const _,
                Some(transmute(notify_disabled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLStyleElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLStyleElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::media\0".as_ptr() as *const _,
                Some(transmute(notify_media_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sheet_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLStyleElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLStyleElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::sheet\0".as_ptr() as *const _,
                Some(transmute(notify_sheet_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLStyleElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLStyleElement>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLStyleElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLStyleElement")
    }
}
