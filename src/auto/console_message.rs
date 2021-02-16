// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConsoleMessage(Boxed<ffi::WebKitConsoleMessage>);

    match fn {
        copy => |ptr| ffi::webkit_console_message_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_console_message_free(ptr),
        get_type => || ffi::webkit_console_message_get_type(),
    }
}

impl ConsoleMessage {
    //#[cfg(any(feature = "v2_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    //#[doc(alias = "webkit_console_message_get_level")]
    //pub fn get_level(&mut self) -> /*Ignored*/ConsoleMessageLevel {
    //    unsafe { TODO: call ffi:webkit_console_message_get_level() }
    //}

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    #[doc(alias = "webkit_console_message_get_line")]
    pub fn get_line(&mut self) -> u32 {
        unsafe {
            ffi::webkit_console_message_get_line(self.to_glib_none_mut().0)
        }
    }

    //#[cfg(any(feature = "v2_12", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    //#[doc(alias = "webkit_console_message_get_source")]
    //pub fn get_source(&mut self) -> /*Ignored*/ConsoleMessageSource {
    //    unsafe { TODO: call ffi:webkit_console_message_get_source() }
    //}

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    #[doc(alias = "webkit_console_message_get_source_id")]
    pub fn get_source_id(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_console_message_get_source_id(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
    #[doc(alias = "webkit_console_message_get_text")]
    pub fn get_text(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_console_message_get_text(self.to_glib_none_mut().0))
        }
    }
}
