// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMFileList(Object<ffi::WebKitDOMFileList>);

    match fn {
        get_type => || ffi::webkit_dom_file_list_get_type(),
    }
}

impl DOMFileList {
    //pub fn get_length(&self) -> /*Unimplemented*/Fundamental: ULong {
    //    unsafe { TODO: call ffi::webkit_dom_file_list_get_length() }
    //}

    //pub fn item(&self, index: /*Unimplemented*/Fundamental: ULong) -> Option<DOMFile> {
    //    unsafe { TODO: call ffi::webkit_dom_file_list_item() }
    //}
}
