// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DOMNode;
use crate::DOMObject;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use crate::DOMRange;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
#[cfg(any(feature = "v2_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
use std::ptr;

glib::wrapper! {
    pub struct DOMDOMSelection(Object<ffi::WebKitDOMDOMSelection, ffi::WebKitDOMDOMSelectionClass>) @extends DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_dom_selection_get_type(),
    }
}

pub const NONE_DOMDOM_SELECTION: Option<&DOMDOMSelection> = None;

pub trait DOMDOMSelectionExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_add_range")]
    fn add_range<P: IsA<DOMRange>>(&self, range: &P);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_collapse")]
    fn collapse<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_collapse_to_end")]
    fn collapse_to_end(&self) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_collapse_to_start")]
    fn collapse_to_start(&self) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_contains_node")]
    fn contains_node<P: IsA<DOMNode>>(&self, node: &P, allowPartial: bool) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_delete_from_document")]
    fn delete_from_document(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_empty")]
    fn empty(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_extend")]
    fn extend<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_anchor_node")]
    fn get_anchor_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_anchor_offset")]
    fn get_anchor_offset(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_base_node")]
    fn get_base_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_base_offset")]
    fn get_base_offset(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_extent_node")]
    fn get_extent_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_extent_offset")]
    fn get_extent_offset(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_focus_node")]
    fn get_focus_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_focus_offset")]
    fn get_focus_offset(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_is_collapsed")]
    fn get_is_collapsed(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_range_at")]
    fn get_range_at(&self, index: libc::c_ulong) -> Result<DOMRange, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_range_count")]
    fn get_range_count(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_get_selection_type")]
    fn get_selection_type(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_modify")]
    fn modify(&self, alter: &str, direction: &str, granularity: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_remove_all_ranges")]
    fn remove_all_ranges(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_select_all_children")]
    fn select_all_children<P: IsA<DOMNode>>(&self, node: &P);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_set_base_and_extent")]
    fn set_base_and_extent<P: IsA<DOMNode>, Q: IsA<DOMNode>>(&self, baseNode: &P, baseOffset: libc::c_ulong, extentNode: &Q, extentOffset: libc::c_ulong);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    #[doc(alias = "webkit_dom_dom_selection_set_position")]
    fn set_position<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong);

    fn get_property_anchor_node(&self) -> Option<DOMNode>;

    fn get_property_anchor_offset(&self) -> libc::c_ulong;

    fn get_property_base_node(&self) -> Option<DOMNode>;

    fn get_property_base_offset(&self) -> libc::c_ulong;

    fn get_property_extent_node(&self) -> Option<DOMNode>;

    fn get_property_extent_offset(&self) -> libc::c_ulong;

    fn get_property_focus_node(&self) -> Option<DOMNode>;

    fn get_property_focus_offset(&self) -> libc::c_ulong;

    fn get_property_is_collapsed(&self) -> bool;

    fn get_property_range_count(&self) -> libc::c_ulong;

    fn get_property_type(&self) -> Option<glib::GString>;

    fn connect_property_anchor_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_anchor_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_base_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_base_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_extent_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_extent_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_focus_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_focus_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_range_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMDOMSelection>> DOMDOMSelectionExt for O {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn add_range<P: IsA<DOMRange>>(&self, range: &P) {
        unsafe {
            ffi::webkit_dom_dom_selection_add_range(self.as_ref().to_glib_none().0, range.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn collapse<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_collapse(self.as_ref().to_glib_none().0, node.as_ref().to_glib_none().0, offset);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn collapse_to_end(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_collapse_to_end(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn collapse_to_start(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_collapse_to_start(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn contains_node<P: IsA<DOMNode>>(&self, node: &P, allowPartial: bool) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_selection_contains_node(self.as_ref().to_glib_none().0, node.as_ref().to_glib_none().0, allowPartial.to_glib()))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn delete_from_document(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_delete_from_document(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn empty(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_empty(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn extend<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_extend(self.as_ref().to_glib_none().0, node.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_anchor_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_anchor_node(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_anchor_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_anchor_offset(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_base_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_base_node(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_base_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_base_offset(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_extent_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_extent_node(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_extent_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_extent_offset(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_focus_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_focus_node(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_focus_offset(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_focus_offset(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_is_collapsed(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_selection_get_is_collapsed(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_range_at(&self, index: libc::c_ulong) -> Result<DOMRange, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_selection_get_range_at(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_range_count(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_dom_selection_get_range_count(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn get_selection_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_dom_selection_get_selection_type(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn modify(&self, alter: &str, direction: &str, granularity: &str) {
        unsafe {
            ffi::webkit_dom_dom_selection_modify(self.as_ref().to_glib_none().0, alter.to_glib_none().0, direction.to_glib_none().0, granularity.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn remove_all_ranges(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_remove_all_ranges(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn select_all_children<P: IsA<DOMNode>>(&self, node: &P) {
        unsafe {
            ffi::webkit_dom_dom_selection_select_all_children(self.as_ref().to_glib_none().0, node.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_base_and_extent<P: IsA<DOMNode>, Q: IsA<DOMNode>>(&self, baseNode: &P, baseOffset: libc::c_ulong, extentNode: &Q, extentOffset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_set_base_and_extent(self.as_ref().to_glib_none().0, baseNode.as_ref().to_glib_none().0, baseOffset, extentNode.as_ref().to_glib_none().0, extentOffset);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_16")))]
    fn set_position<P: IsA<DOMNode>>(&self, node: &P, offset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_set_position(self.as_ref().to_glib_none().0, node.as_ref().to_glib_none().0, offset);
        }
    }

    fn get_property_anchor_node(&self) -> Option<DOMNode> {
        unsafe {
            let mut value = glib::Value::from_type(<DOMNode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"anchor-node\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `anchor-node` getter")
        }
    }

    fn get_property_anchor_offset(&self) -> libc::c_ulong {
        unsafe {
            let mut value = glib::Value::from_type(<libc::c_ulong as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"anchor-offset\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `anchor-offset` getter").unwrap()
        }
    }

    fn get_property_base_node(&self) -> Option<DOMNode> {
        unsafe {
            let mut value = glib::Value::from_type(<DOMNode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"base-node\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `base-node` getter")
        }
    }

    fn get_property_base_offset(&self) -> libc::c_ulong {
        unsafe {
            let mut value = glib::Value::from_type(<libc::c_ulong as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"base-offset\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `base-offset` getter").unwrap()
        }
    }

    fn get_property_extent_node(&self) -> Option<DOMNode> {
        unsafe {
            let mut value = glib::Value::from_type(<DOMNode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"extent-node\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `extent-node` getter")
        }
    }

    fn get_property_extent_offset(&self) -> libc::c_ulong {
        unsafe {
            let mut value = glib::Value::from_type(<libc::c_ulong as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"extent-offset\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `extent-offset` getter").unwrap()
        }
    }

    fn get_property_focus_node(&self) -> Option<DOMNode> {
        unsafe {
            let mut value = glib::Value::from_type(<DOMNode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"focus-node\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `focus-node` getter")
        }
    }

    fn get_property_focus_offset(&self) -> libc::c_ulong {
        unsafe {
            let mut value = glib::Value::from_type(<libc::c_ulong as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"focus-offset\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `focus-offset` getter").unwrap()
        }
    }

    fn get_property_is_collapsed(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"is-collapsed\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `is-collapsed` getter").unwrap()
        }
    }

    fn get_property_range_count(&self) -> libc::c_ulong {
        unsafe {
            let mut value = glib::Value::from_type(<libc::c_ulong as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"range-count\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `range-count` getter").unwrap()
        }
    }

    fn get_property_type(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `type` getter")
        }
    }

    fn connect_property_anchor_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchor_node_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::anchor-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_anchor_node_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_anchor_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchor_offset_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::anchor-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_anchor_offset_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_base_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_base_node_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::base-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_base_node_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_base_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_base_offset_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::base-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_base_offset_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_extent_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_extent_node_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::extent-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_extent_node_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_extent_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_extent_offset_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::extent-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_extent_offset_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_focus_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_node_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::focus-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_focus_node_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_focus_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_offset_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::focus-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_focus_offset_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_is_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_collapsed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-collapsed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_is_collapsed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_range_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_range_count_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::range-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_range_count_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::WebKitDOMDOMSelection, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer)
            where P: IsA<DOMDOMSelection>
        {
            let f: &F = &*(f as *const F);
            f(&DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMDOMSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMDOMSelection")
    }
}
