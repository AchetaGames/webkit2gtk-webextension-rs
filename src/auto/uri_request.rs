// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct URIRequest(Object<ffi::WebKitURIRequest, ffi::WebKitURIRequestClass>);

    match fn {
        get_type => || ffi::webkit_uri_request_get_type(),
    }
}

impl URIRequest {
    #[doc(alias = "webkit_uri_request_new")]
    pub fn new(uri: &str) -> URIRequest {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_uri_request_new(uri.to_glib_none().0))
        }
    }
}

pub const NONE_URI_REQUEST: Option<&URIRequest> = None;

pub trait URIRequestExt: 'static {
    //#[doc(alias = "webkit_uri_request_get_http_headers")]
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    #[doc(alias = "webkit_uri_request_get_http_method")]
    fn get_http_method(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_uri_request_get_uri")]
    fn get_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_uri_request_set_uri")]
    fn set_uri(&self, uri: &str);

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<URIRequest>> URIRequestExt for O {
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders> {
    //    unsafe { TODO: call ffi:webkit_uri_request_get_http_headers() }
    //}

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    fn get_http_method(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_uri_request_get_http_method(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_uri_request_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::webkit_uri_request_set_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitURIRequest, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<URIRequest>
        {
            let f: &F = &*(f as *const F);
            f(&URIRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_uri_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for URIRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("URIRequest")
    }
}
