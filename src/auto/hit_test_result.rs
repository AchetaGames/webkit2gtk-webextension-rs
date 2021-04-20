// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct HitTestResult(Object<ffi::WebKitHitTestResult, ffi::WebKitHitTestResultClass>);

    match fn {
        type_ => || ffi::webkit_hit_test_result_get_type(),
    }
}

pub const NONE_HIT_TEST_RESULT: Option<&HitTestResult> = None;

pub trait HitTestResultExt: 'static {
    #[doc(alias = "webkit_hit_test_result_context_is_editable")]
    fn context_is_editable(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_image")]
    fn context_is_image(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_link")]
    fn context_is_link(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_media")]
    fn context_is_media(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_context_is_scrollbar")]
    fn context_is_scrollbar(&self) -> bool;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_hit_test_result_context_is_selection")]
    fn context_is_selection(&self) -> bool;

    #[doc(alias = "webkit_hit_test_result_get_context")]
    fn context(&self) -> u32;

    #[doc(alias = "webkit_hit_test_result_get_image_uri")]
    fn image_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_link_label")]
    fn link_label(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_link_title")]
    fn link_title(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_link_uri")]
    fn link_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_hit_test_result_get_media_uri")]
    fn media_uri(&self) -> Option<glib::GString>;
}

impl<O: IsA<HitTestResult>> HitTestResultExt for O {
    fn context_is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_editable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_link(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_link(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_media(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context_is_scrollbar(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_scrollbar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn context_is_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_selection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn context(&self) -> u32 {
        unsafe { ffi::webkit_hit_test_result_get_context(self.as_ref().to_glib_none().0) }
    }

    fn image_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_image_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn link_label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn link_title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn link_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_link_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn media_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_hit_test_result_get_media_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for HitTestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HitTestResult")
    }
}
