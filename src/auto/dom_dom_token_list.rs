// This file was generated by gir (d50d839) from gir-files (469db10)
// DO NOT EDIT

use DOMObject;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use Error;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMDOMTokenList(Object<ffi::WebKitDOMDOMTokenList, ffi::WebKitDOMDOMTokenListClass>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_dom_token_list_get_type(),
    }
}

pub trait DOMDOMTokenListExt {
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn add(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn contains(&self, token: &str) -> bool;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_length(&self) -> libc::c_ulong;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_value(&self) -> Option<String>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn item(&self, index: libc::c_ulong) -> Option<String>;

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn remove(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn replace(&self, token: &str, newToken: &str) -> Result<(), Error>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_value(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn toggle(&self, token: &str, force: bool) -> Result<(), Error>;

    fn get_property_length(&self) -> libc::c_ulong;

    fn get_property_value(&self) -> Option<String>;

    fn set_property_value(&self, value: Option<&str>);

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMDOMTokenList> + IsA<glib::object::Object>> DOMDOMTokenListExt for O {
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn add(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::webkit_dom_dom_token_list_add() }
    //}

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn contains(&self, token: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_token_list_contains(self.to_glib_none().0, token.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_token_list_get_length(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_dom_token_list_get_value(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn item(&self, index: libc::c_ulong) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_dom_token_list_item(self.to_glib_none().0, index))
        }
    }

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn remove(&self, error: &mut Error, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::webkit_dom_dom_token_list_remove() }
    //}

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn replace(&self, token: &str, newToken: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_token_list_replace(self.to_glib_none().0, token.to_glib_none().0, newToken.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_dom_token_list_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn toggle(&self, token: &str, force: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_token_list_toggle(self.to_glib_none().0, token.to_glib_none().0, force.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_length(&self) -> libc::c_ulong {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <libc::c_ulong as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "length".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_value(&self) -> Option<String> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <String as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "value".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_value(&self, value: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "value".to_glib_none().0, Value::from(value).to_glib_none().0);
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::length",
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::WebKitDOMDOMTokenList, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDOMTokenList> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDOMTokenList::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::WebKitDOMDOMTokenList, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDOMTokenList> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDOMTokenList::from_glib_borrow(this).downcast_unchecked())
}