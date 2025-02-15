// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ContextMenuItem;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct ContextMenu(Object<ffi::WebKitContextMenu, ffi::WebKitContextMenuClass>);

    match fn {
        get_type => || ffi::webkit_context_menu_get_type(),
    }
}

impl ContextMenu {
    #[doc(alias = "webkit_context_menu_new")]
    pub fn new() -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_context_menu_new())
        }
    }

    #[doc(alias = "webkit_context_menu_new_with_items")]
    pub fn with_items(items: &[ContextMenuItem]) -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_context_menu_new_with_items(items.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_context_menu_append")]
    pub fn append(&self, item: &ContextMenuItem) {
        unsafe {
            ffi::webkit_context_menu_append(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_context_menu_first")]
    pub fn first(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_first(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_context_menu_get_item_at_position")]
    pub fn get_item_at_position(&self, position: u32) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_get_item_at_position(self.to_glib_none().0, position))
        }
    }

    #[doc(alias = "webkit_context_menu_get_items")]
    pub fn get_items(&self) -> Vec<ContextMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_context_menu_get_items(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_context_menu_get_n_items")]
    pub fn get_n_items(&self) -> u32 {
        unsafe {
            ffi::webkit_context_menu_get_n_items(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_context_menu_get_user_data")]
    pub fn get_user_data(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_get_user_data(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_context_menu_insert")]
    pub fn insert(&self, item: &ContextMenuItem, position: i32) {
        unsafe {
            ffi::webkit_context_menu_insert(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    #[doc(alias = "webkit_context_menu_last")]
    pub fn last(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_last(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_context_menu_move_item")]
    pub fn move_item(&self, item: &ContextMenuItem, position: i32) {
        unsafe {
            ffi::webkit_context_menu_move_item(self.to_glib_none().0, item.to_glib_none().0, position);
        }
    }

    #[doc(alias = "webkit_context_menu_prepend")]
    pub fn prepend(&self, item: &ContextMenuItem) {
        unsafe {
            ffi::webkit_context_menu_prepend(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_context_menu_remove")]
    pub fn remove(&self, item: &ContextMenuItem) {
        unsafe {
            ffi::webkit_context_menu_remove(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_context_menu_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::webkit_context_menu_remove_all(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_context_menu_set_user_data")]
    pub fn set_user_data(&self, user_data: &glib::Variant) {
        unsafe {
            ffi::webkit_context_menu_set_user_data(self.to_glib_none().0, user_data.to_glib_none().0);
        }
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ContextMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ContextMenu")
    }
}
