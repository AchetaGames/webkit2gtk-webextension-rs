// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct WebExtension(Object<ffi::WebKitWebExtension>);

    match fn {
        get_type => || ffi::webkit_web_extension_get_type(),
    }
}

impl WebExtension {
    //pub fn get_page(&self, page_id: u64) -> /*Ignored*/Option<WebPage> {
    //    unsafe { TODO: call ffi::webkit_web_extension_get_page() }
    //}

    //pub fn connect_page_created<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored web_page: WebKit2WebExtension.WebPage
    //}
}