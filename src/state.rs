use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};

// This struct represents the state of our app. This simple app does not
// need any state, by this type still supports the `Dispatch` implementations.
#[derive(Default)]
pub struct AppData {
    pub virtual_keyboard_manager: Option<ZwpVirtualKeyboardManagerV1>,
    pub virtual_keyboard: Option<ZwpVirtualKeyboardV1>,
}
