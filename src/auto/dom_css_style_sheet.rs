// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DOMCSSRule;
use crate::DOMCSSRuleList;
use crate::DOMObject;
use crate::DOMStyleSheet;
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
    pub struct DOMCSSStyleSheet(Object<ffi::WebKitDOMCSSStyleSheet, ffi::WebKitDOMCSSStyleSheetClass>) @extends DOMStyleSheet, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_style_sheet_get_type(),
    }
}

pub const NONE_DOMCSS_STYLE_SHEET: Option<&DOMCSSStyleSheet> = None;

pub trait DOMCSSStyleSheetExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_add_rule")]
    fn add_rule(&self, selector: &str, style: &str, index: libc::c_ulong) -> Result<libc::c_long, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_delete_rule")]
    fn delete_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_get_css_rules")]
    fn get_css_rules(&self) -> Option<DOMCSSRuleList>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_get_owner_rule")]
    fn get_owner_rule(&self) -> Option<DOMCSSRule>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_get_rules")]
    fn get_rules(&self) -> Option<DOMCSSRuleList>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_insert_rule")]
    fn insert_rule(&self, rule: &str, index: libc::c_ulong) -> Result<libc::c_ulong, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_remove_rule")]
    fn remove_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error>;

    fn connect_property_css_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_owner_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMCSSStyleSheet>> DOMCSSStyleSheetExt for O {
    fn add_rule(&self, selector: &str, style: &str, index: libc::c_ulong) -> Result<libc::c_long, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_add_rule(self.as_ref().to_glib_none().0, selector.to_glib_none().0, style.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_delete_rule(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_css_rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_css_rules(self.as_ref().to_glib_none().0))
        }
    }

    fn get_owner_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_owner_rule(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_rules(self.as_ref().to_glib_none().0))
        }
    }

    fn insert_rule(&self, rule: &str, index: libc::c_ulong) -> Result<libc::c_ulong, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_insert_rule(self.as_ref().to_glib_none().0, rule.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_remove_rule(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_property_css_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_css_rules_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMCSSStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMCSSStyleSheet>
        {
            let f: &F = &*(f as *const F);
            f(&DOMCSSStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::css-rules\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_css_rules_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_owner_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_rule_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMCSSStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMCSSStyleSheet>
        {
            let f: &F = &*(f as *const F);
            f(&DOMCSSStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::owner-rule\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_owner_rule_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rules_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMCSSStyleSheet, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMCSSStyleSheet>
        {
            let f: &F = &*(f as *const F);
            f(&DOMCSSStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rules\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_rules_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMCSSStyleSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMCSSStyleSheet")
    }
}
