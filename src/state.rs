use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};

use wayland_protocols_wlr::virtual_pointer::v1::client::{
    zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1,
    zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1,
};

// This struct represents the state of our app. This simple app does not
// need any state, by this type still supports the `Dispatch` implementations.
#[derive(Debug)]
pub struct AppData {
    pub(crate) virtual_keyboard_manager: Option<ZwpVirtualKeyboardManagerV1>,
    pub(crate) virtual_keyboard: Option<ZwpVirtualKeyboardV1>,

    pub(crate) virtual_pointer_manager: Option<ZwlrVirtualPointerManagerV1>,
    pub(crate) virtual_pointer: Option<ZwlrVirtualPointerV1>,
}

impl AppData {
    pub(crate) fn new() -> Self {
        Self {
            virtual_keyboard_manager: None,
            virtual_keyboard: None,
            virtual_pointer_manager: None,
            virtual_pointer: None,
        }
    }
}
