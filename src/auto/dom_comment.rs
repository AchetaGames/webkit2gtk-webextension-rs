// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use DOMCharacterData;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMComment(Object<ffi::WebKitDOMComment>): DOMCharacterData;

    match fn {
        get_type => || ffi::webkit_dom_comment_get_type(),
    }
}

impl DOMComment {}
