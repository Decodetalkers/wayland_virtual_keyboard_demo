use std::{ffi::CString, fs::File, io::Write, path::PathBuf};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};
use xkbcommon::xkb;

// This struct represents the state of our app. This simple app does not
// need any state, by this type still supports the `Dispatch` implementations.
pub struct AppData {
    pub virtual_keyboard_manager: Option<ZwpVirtualKeyboardManagerV1>,
    pub virtual_keyboard: Option<ZwpVirtualKeyboardV1>,
    pub xkb_state: xkb::State,
}

impl AppData {
    pub fn init() -> Self {
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

        let keymap = xkb::Keymap::new_from_names(
            &context,
            "",
            "",
            "no",
            "",
            None,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        )
        .expect("xkbcommon keymap panicked!");
        Self {
            virtual_keyboard_manager: None,
            virtual_keyboard: None,
            xkb_state: xkb::State::new(&keymap),
        }
    }
    pub fn get_keymap_as_file(&mut self) -> (File, u32) {
        let keymap = self
            .xkb_state
            .get_keymap()
            .get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);
        let keymap = CString::new(keymap).expect("Keymap should not contain interior nul bytes");
        let keymap = keymap.as_bytes_with_nul();
        let dir = std::env::var_os("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let mut file = tempfile::tempfile_in(dir).expect("File could not be created!");
        file.write_all(keymap).unwrap();
        file.flush().unwrap();
        (file, keymap.len() as u32)
    }
}
