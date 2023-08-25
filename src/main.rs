use wayland_client::{protocol::wl_pointer, Connection};

mod dispatch;
mod state;

use state::AppData;
use wayland_client::protocol::wl_keyboard::KeyState;
// The main function of our program
fn main() {
    // Create a Wayland connection by connecting to the server through the
    // environment-provided configuration.
    let conn = Connection::connect_to_env().unwrap();

    // Retrieve the WlDisplay Wayland object from the connection. This object is
    // the starting point of any Wayland program, from which all other objects will
    // be created.
    let display = conn.display();

    // Create an event queue for our event processing
    // TODO: mut it
    let mut event_queue = conn.new_event_queue();
    // An get its handle to associated new objects to it
    let qh = event_queue.handle();

    // Create a wl_registry object by sending the wl_display.get_registry request
    // This method takes two arguments: a handle to the queue the newly created
    // wl_registry will be assigned to, and the user-data that should be associated
    // with this registry (here it is () as we don't need user-data).
    let _registry = display.get_registry(&qh, ());

    // At this point everything is ready, and we just need to wait to receive the events
    // from the wl_registry, our callback will print the advertized globals.
    let mut data = AppData::init();
    while data.virtual_keyboard.is_none() || data.virtual_pointer.is_none() {
        event_queue.blocking_dispatch(&mut data).unwrap();
    }
    let mut pressed = false;
    loop {
        pressed = !pressed;
        data.virtual_keyboard.as_ref().unwrap().key(
            100,
            10,
            if pressed {
                KeyState::Released.into()
            } else {
                KeyState::Pressed.into()
            },
        );

        data.virtual_pointer.as_ref().unwrap().axis_discrete(
            100,
            wl_pointer::Axis::VerticalScroll,
            10.0,
            10,
        );

        data.virtual_pointer.as_ref().unwrap().button(
            100,
            2,
            if pressed {
                wl_pointer::ButtonState::Released.into()
            } else {
                wl_pointer::ButtonState::Pressed.into()
            },
        );
        std::thread::sleep(std::time::Duration::from_nanos(100));
    }
}
