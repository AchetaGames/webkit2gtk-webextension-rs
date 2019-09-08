// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMCharacterData;
use DOMEventTarget;
use DOMNode;
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
    pub struct DOMText(Object<webkit2_webextension_sys::WebKitDOMText, webkit2_webextension_sys::WebKitDOMTextClass, DOMTextClass>) @extends DOMCharacterData, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_text_get_type(),
    }
}

pub const NONE_DOM_TEXT: Option<&DOMText> = None;

pub trait DOMTextExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_whole_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_14", deprecated)]
    fn replace_whole_text(&self, content: &str) -> Result<DOMText, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn split_text(&self, offset: libc::c_ulong) -> Result<DOMText, glib::Error>;

    fn connect_property_whole_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMText>> DOMTextExt for O {
    fn get_whole_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_text_get_whole_text(self.as_ref().to_glib_none().0))
        }
    }

    fn replace_whole_text(&self, content: &str) -> Result<DOMText, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_text_replace_whole_text(self.as_ref().to_glib_none().0, content.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn split_text(&self, offset: libc::c_ulong) -> Result<DOMText, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_text_split_text(self.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_whole_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_whole_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMText, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMText>
        {
            let f: &F = &*(f as *const F);
            f(&DOMText::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::whole-text\0".as_ptr() as *const _,
                Some(transmute(notify_whole_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMText")
    }
}
