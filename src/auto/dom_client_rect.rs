// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct DOMClientRect(Object<ffi::WebKitDOMClientRect, ffi::WebKitDOMClientRectClass>) @extends DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_client_rect_get_type(),
    }
}

pub const NONE_DOM_CLIENT_RECT: Option<&DOMClientRect> = None;

pub trait DOMClientRectExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_dom_client_rect_get_bottom")]
    fn get_bottom(&self) -> f32;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_dom_client_rect_get_height")]
    fn get_height(&self) -> f32;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_dom_client_rect_get_left")]
    fn get_left(&self) -> f32;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_dom_client_rect_get_right")]
    fn get_right(&self) -> f32;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_dom_client_rect_get_top")]
    fn get_top(&self) -> f32;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    #[doc(alias = "webkit_dom_client_rect_get_width")]
    fn get_width(&self) -> f32;

    fn get_property_bottom(&self) -> f32;

    fn get_property_height(&self) -> f32;

    fn get_property_left(&self) -> f32;

    fn get_property_right(&self) -> f32;

    fn get_property_top(&self) -> f32;

    fn get_property_width(&self) -> f32;

    fn connect_property_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_right_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMClientRect>> DOMClientRectExt for O {
    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn get_bottom(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_bottom(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn get_height(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_height(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn get_left(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_left(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn get_right(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_right(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn get_top(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_top(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_18")))]
    fn get_width(&self) -> f32 {
        unsafe {
            ffi::webkit_dom_client_rect_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_property_bottom(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"bottom\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `bottom` getter").unwrap()
        }
    }

    fn get_property_height(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"height\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `height` getter").unwrap()
        }
    }

    fn get_property_left(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"left\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `left` getter").unwrap()
        }
    }

    fn get_property_right(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"right\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `right` getter").unwrap()
        }
    }

    fn get_property_top(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"top\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `top` getter").unwrap()
        }
    }

    fn get_property_width(&self) -> f32 {
        unsafe {
            let mut value = glib::Value::from_type(<f32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"width\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `width` getter").unwrap()
        }
    }

    fn connect_property_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bottom_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMClientRect>
        {
            let f: &F = &*(f as *const F);
            f(&DOMClientRect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::bottom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_bottom_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMClientRect>
        {
            let f: &F = &*(f as *const F);
            f(&DOMClientRect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_height_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_left_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMClientRect>
        {
            let f: &F = &*(f as *const F);
            f(&DOMClientRect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::left\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_left_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_right_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_right_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMClientRect>
        {
            let f: &F = &*(f as *const F);
            f(&DOMClientRect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::right\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_right_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_top_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMClientRect>
        {
            let f: &F = &*(f as *const F);
            f(&DOMClientRect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::top\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_top_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMClientRect, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMClientRect>
        {
            let f: &F = &*(f as *const F);
            f(&DOMClientRect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_width_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMClientRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMClientRect")
    }
}
