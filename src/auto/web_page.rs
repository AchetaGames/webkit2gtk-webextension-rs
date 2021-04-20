// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
use crate::ConsoleMessage;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use crate::ContextMenu;
use crate::DOMDocument;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use crate::Frame;
use crate::URIRequest;
use crate::URIResponse;
#[cfg(any(feature = "v2_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
use crate::WebEditor;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use crate::WebHitTestResult;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct WebPage(Object<ffi::WebKitWebPage, ffi::WebKitWebPageClass>);

    match fn {
        type_ => || ffi::webkit_web_page_get_type(),
    }
}

pub const NONE_WEB_PAGE: Option<&WebPage> = None;

pub trait WebPageExt: 'static {
    #[doc(alias = "webkit_web_page_get_dom_document")]
    fn dom_document(&self) -> Option<DOMDocument>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[doc(alias = "webkit_web_page_get_editor")]
    fn editor(&self) -> Option<WebEditor>;

    #[doc(alias = "webkit_web_page_get_id")]
    fn id(&self) -> u64;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_web_page_get_main_frame")]
    fn main_frame(&self) -> Option<Frame>;

    #[doc(alias = "webkit_web_page_get_uri")]
    fn uri(&self) -> Option<glib::GString>;

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //#[doc(alias = "webkit_web_page_send_message_to_view")]
    //fn send_message_to_view<P: FnOnce(Result</*Ignored*/UserMessage, glib::Error>) + Send + 'static>(&self, message: /*Ignored*/&UserMessage, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P);

    //
    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn send_message_to_view_future(&self, message: /*Ignored*/&UserMessage) -> Pin<Box_<dyn std::future::Future<Output = Result</*Ignored*/UserMessage, glib::Error>> + 'static>>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    fn connect_console_message_sent<F: Fn(&Self, &ConsoleMessage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_context_menu<F: Fn(&Self, &ContextMenu, &WebHitTestResult) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_document_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg_attr(feature = "v2_26", deprecated)]
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    //fn connect_form_controls_associated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    //fn connect_form_controls_associated_for_frame<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_send_request<F: Fn(&Self, &URIRequest, Option<&URIResponse>) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn connect_user_message_received<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    //fn connect_will_submit_form<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebPage>> WebPageExt for O {
    fn dom_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_dom_document(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    fn editor(&self) -> Option<WebEditor> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_editor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn id(&self) -> u64 {
        unsafe { ffi::webkit_web_page_get_id(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn main_frame(&self) -> Option<Frame> {
        unsafe {
            from_glib_none(ffi::webkit_web_page_get_main_frame(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_web_page_get_uri(self.as_ref().to_glib_none().0)) }
    }

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn send_message_to_view<P: FnOnce(Result</*Ignored*/UserMessage, glib::Error>) + Send + 'static>(&self, message: /*Ignored*/&UserMessage, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call ffi:webkit_web_page_send_message_to_view() }
    //}

    //
    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn send_message_to_view_future(&self, message: /*Ignored*/&UserMessage) -> Pin<Box_<dyn std::future::Future<Output = Result</*Ignored*/UserMessage, glib::Error>> + 'static>> {

    //let message = message.clone();
    //Box_::pin(gio::GioFuture::new(self, move |obj, send| {
    //    let cancellable = gio::Cancellable::new();
    //    obj.send_message_to_view(
    //        &message,
    //        Some(&cancellable),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );

    //    cancellable
    //}))
    //}

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    fn connect_console_message_sent<F: Fn(&Self, &ConsoleMessage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn console_message_sent_trampoline<
            P,
            F: Fn(&P, &ConsoleMessage) + 'static,
        >(
            this: *mut ffi::WebKitWebPage,
            console_message: *mut ffi::WebKitConsoleMessage,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<WebPage>,
        {
            let f: &F = &*(f as *const F);
            f(
                &WebPage::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(console_message),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"console-message-sent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    console_message_sent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_context_menu<F: Fn(&Self, &ContextMenu, &WebHitTestResult) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn context_menu_trampoline<
            P,
            F: Fn(&P, &ContextMenu, &WebHitTestResult) -> bool + 'static,
        >(
            this: *mut ffi::WebKitWebPage,
            context_menu: *mut ffi::WebKitContextMenu,
            hit_test_result: *mut ffi::WebKitWebHitTestResult,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<WebPage>,
        {
            let f: &F = &*(f as *const F);
            f(
                &WebPage::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(context_menu),
                &from_glib_borrow(hit_test_result),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"context-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    context_menu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_document_loaded<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn document_loaded_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitWebPage,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<WebPage>,
        {
            let f: &F = &*(f as *const F);
            f(&WebPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"document-loaded\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    document_loaded_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    //fn connect_form_controls_associated<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype elements: *.PtrArray TypeId { ns_id: 1, id: 12 }
    //}

    //#[cfg(any(feature = "v2_26", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_26")))]
    //fn connect_form_controls_associated_for_frame<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype elements: *.PtrArray TypeId { ns_id: 1, id: 12 }
    //}

    fn connect_send_request<F: Fn(&Self, &URIRequest, Option<&URIResponse>) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_request_trampoline<
            P,
            F: Fn(&P, &URIRequest, Option<&URIResponse>) -> bool + 'static,
        >(
            this: *mut ffi::WebKitWebPage,
            request: *mut ffi::WebKitURIRequest,
            redirected_response: *mut ffi::WebKitURIResponse,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean
        where
            P: IsA<WebPage>,
        {
            let f: &F = &*(f as *const F);
            f(
                &WebPage::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(request),
                Option::<URIResponse>::from_glib_borrow(redirected_response)
                    .as_ref()
                    .as_ref(),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-request\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    send_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[cfg(any(feature = "v2_28", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    //fn connect_user_message_received<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored message: WebKit2WebExtension.UserMessage
    //}

    //#[cfg(any(feature = "v2_20", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    //fn connect_will_submit_form<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored step: WebKit2WebExtension.FormSubmissionStep
    //    Empty ctype text_field_names: *.PtrArray TypeId { ns_id: 0, id: 28 }
    //    Empty ctype text_field_values: *.PtrArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitWebPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<WebPage>,
        {
            let f: &F = &*(f as *const F);
            f(&WebPage::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_uri_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WebPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebPage")
    }
}
