use crate::AppData;
use std::os::unix::prelude::AsRawFd;
#[allow(unused)]
use wayland_client::{
    protocol::{
        wl_buffer::{self, WlBuffer},
        wl_compositor::{self, WlCompositor},
        wl_keyboard,
        wl_registry::{self, WlRegistry},
        wl_seat::{self, WlSeat},
        wl_shm::{self, WlShm},
        wl_shm_pool::{self, WlShmPool},
        wl_surface::{self, WlSurface},
    },
    Connection, Dispatch, Proxy, QueueHandle,
};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};

use wayland_protocols_wlr::virtual_pointer::v1::client::{
    zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1,
    zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1,
};

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        // When receiving events from the wl_registry, we are only interested in the
        // `global` event, which signals a new available global.
        // When receiving this event, we just print its characteristics in this example.
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == WlShm::interface().name {
                registry.bind::<WlShm, _, _>(name, version, qh, ());
            } else if interface == WlSeat::interface().name {
                registry.bind::<WlSeat, _, _>(name, version, qh, ());
            } else if interface == ZwpVirtualKeyboardManagerV1::interface().name {
                let virtual_keyboard_manager =
                    registry.bind::<ZwpVirtualKeyboardManagerV1, _, _>(name, version, qh, ());
                state.virtual_keyboard_manager = Some(virtual_keyboard_manager);
            } else if interface == ZwlrVirtualPointerManagerV1::interface().name {
                let virtual_pointer_manager =
                    registry.bind::<ZwlrVirtualPointerManagerV1, _, _>(name, version, qh, ());
                state.virtual_pointer_manager = Some(virtual_pointer_manager);
            }
        }
    }
}

impl Dispatch<WlShm, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &WlShm,
        _event: <WlShm as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSeat, ()> for AppData {
    fn event(
        state: &mut Self,
        seat: &WlSeat,
        _event: <WlSeat as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let Some(virtual_keyboard_manager) = state.virtual_keyboard_manager.as_ref() {
            let virtual_keyboard = virtual_keyboard_manager.create_virtual_keyboard(seat, qh, ());
            let (file, size) = state.get_keymap_as_file();
            virtual_keyboard.keymap(
                wl_keyboard::KeymapFormat::XkbV1.into(),
                file.as_raw_fd(),
                size,
            );
            state.virtual_keyboard = Some(virtual_keyboard);
        }
        if let Some(virtual_pointer_manager) = state.virtual_pointer_manager.as_ref() {
            let virtual_pointer =
                virtual_pointer_manager.create_virtual_pointer(Some(&seat), qh, ());
            state.virtual_pointer = Some(virtual_pointer);
        }
    }
}

impl Dispatch<ZwlrVirtualPointerV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrVirtualPointerV1,
        _event: <ZwlrVirtualPointerV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwpVirtualKeyboardManagerV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ZwpVirtualKeyboardManagerV1,
        _event: <ZwpVirtualKeyboardManagerV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrVirtualPointerManagerV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrVirtualPointerManagerV1,
        _event: <ZwlrVirtualPointerManagerV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwpVirtualKeyboardV1, ()> for AppData {
    fn event(
        _state: &mut Self,
        _proxy: &ZwpVirtualKeyboardV1,
        _event: <ZwpVirtualKeyboardV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}
