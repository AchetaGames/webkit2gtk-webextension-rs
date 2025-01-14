// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DOMCharacterData;
use crate::DOMEventTarget;
use crate::DOMNode;
use crate::DOMObject;
use crate::DOMText;
use std::fmt;

glib::wrapper! {
    pub struct DOMCDATASection(Object<ffi::WebKitDOMCDATASection, ffi::WebKitDOMCDATASectionClass>) @extends DOMText, DOMCharacterData, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_cdata_section_get_type(),
    }
}

impl DOMCDATASection {}

pub const NONE_DOMCDATA_SECTION: Option<&DOMCDATASection> = None;

impl fmt::Display for DOMCDATASection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMCDATASection")
    }
}
