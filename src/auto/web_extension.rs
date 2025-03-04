// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::WebPage;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct WebExtension(Object<ffi::WebKitWebExtension, ffi::WebKitWebExtensionClass>);

    match fn {
        get_type => || ffi::webkit_web_extension_get_type(),
    }
}

pub const NONE_WEB_EXTENSION: Option<&WebExtension> = None;

pub trait WebExtensionExt: 'static {
    #[doc(alias = "webkit_web_extension_get_page")]
    fn get_page(&self, page_id: u64) -> Option<WebPage>;

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //#[doc(alias = "webkit_web_extension_send_message_to_context")]
    //fn send_message_to_context<P: FnOnce(Result</*Ignored*/UserMessage, glib::Error>) + Send + 'static>(&self, message: /*Ignored*/&UserMessage, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P);

    //
    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn send_message_to_context_future(&self, message: /*Ignored*/&UserMessage) -> Pin<Box_<dyn std::future::Future<Output = Result</*Ignored*/UserMessage, glib::Error>> + 'static>>;

    fn connect_page_created<F: Fn(&Self, &WebPage) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn connect_user_message_received<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebExtension>> WebExtensionExt for O {
    fn get_page(&self, page_id: u64) -> Option<WebPage> {
        unsafe {
            from_glib_none(ffi::webkit_web_extension_get_page(self.as_ref().to_glib_none().0, page_id))
        }
    }

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn send_message_to_context<P: FnOnce(Result</*Ignored*/UserMessage, glib::Error>) + Send + 'static>(&self, message: /*Ignored*/&UserMessage, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call ffi:webkit_web_extension_send_message_to_context() }
    //}

    //
    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn send_message_to_context_future(&self, message: /*Ignored*/&UserMessage) -> Pin<Box_<dyn std::future::Future<Output = Result</*Ignored*/UserMessage, glib::Error>> + 'static>> {

        //let message = message.clone();
        //Box_::pin(gio::GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    obj.send_message_to_context(
        //        &message,
        //        Some(&cancellable),
        //        move |res| {
        //            send.resolve(res);
        //        },
        //    );

        //    cancellable
        //}))
    //}

    fn connect_page_created<F: Fn(&Self, &WebPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_created_trampoline<P, F: Fn(&P, &WebPage) + 'static>(this: *mut ffi::WebKitWebExtension, web_page: *mut ffi::WebKitWebPage, f: glib::ffi::gpointer)
            where P: IsA<WebExtension>
        {
            let f: &F = &*(f as *const F);
            f(&WebExtension::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(web_page))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-created\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(page_created_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn connect_user_message_received<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored message: WebKit2WebExtension.UserMessage
    //}
}

impl fmt::Display for WebExtension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebExtension")
    }
}
