use wayland_client::{protocol::wl_pointer, Connection, EventQueue};

mod dispatch;
mod state;

pub use state::AppData;
use wayland_client::protocol::wl_keyboard::KeyState;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyPointerError {
    #[error("Connection create Error")]
    ConnectionError(String),
    #[error("Error during queue")]
    QueueError,
}

impl AppData {
    pub fn init(queue: &mut EventQueue<Self>) -> Result<Self, KeyPointerError> {
        let mut data = AppData::new();
        while data.virtual_keyboard.is_none() || data.virtual_pointer.is_none() {
            queue
                .blocking_dispatch(&mut data)
                .map_err(|_| KeyPointerError::QueueError)?;
        }
        Ok(data)
    }

    pub fn notify_pointer_motion(&self, dx: f64, dy: f64) {
        self.virtual_pointer.as_ref().unwrap().motion(10, dx, dy);
    }

    pub fn notify_pointer_motion_absolute(&self, x: f64, y: f64, x_extent: u32, y_extent: u32) {
        self.virtual_pointer
            .as_ref()
            .unwrap()
            .motion_absolute(10, x as u32, y as u32, x_extent, y_extent);
    }

    pub fn notify_pointer_button(&self, button: i32, state: u32) {
        self.virtual_pointer.as_ref().unwrap().button(
            100,
            button as u32,
            if state == 0 {
                wl_pointer::ButtonState::Pressed
            } else {
                wl_pointer::ButtonState::Released
            },
        );
    }

    pub fn notify_pointer_axis(&self, dx: f64, dy: f64) {
        self.virtual_pointer
            .as_ref()
            .unwrap()
            .axis(100, wl_pointer::Axis::HorizontalScroll, dx);
        self.virtual_pointer
            .as_ref()
            .unwrap()
            .axis(100, wl_pointer::Axis::VerticalScroll, dy);
    }

    pub fn notify_pointer_axis_discrete(&self, axis: u32, steps: i32) {
        self.virtual_pointer.as_ref().unwrap().axis_discrete(
            100,
            if axis == 0 {
                wl_pointer::Axis::VerticalScroll
            } else {
                wl_pointer::Axis::HorizontalScroll
            },
            10.0,
            steps,
        );
    }

    pub fn notify_keyboard_keycode(&self, keycode: i32, state: u32) {
        self.virtual_keyboard
            .as_ref()
            .unwrap()
            .key(100, keycode as u32, state);
    }

    pub fn notify_keyboard_keysym(&self, keysym: i32, state: u32) {
        self.virtual_keyboard
            .as_ref()
            .unwrap()
            .key(100, keysym as u32, state);
    }
}

fn main() {
    // Create a Wayland connection by connecting to the server through the
    // environment-provided configuration.
    let conn = Connection::connect_to_env().unwrap();

    // Retrieve the WlDisplay Wayland object from the connection. This object is
    // the starting point of any Wayland program, from which all other objects will
    // be created.
    let display = conn.display();

    // Create an event queue for our event processing
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
    let data = AppData::init(&mut event_queue).unwrap();

    let mut pressed = false;

    loop {
        pressed = !pressed;
        data.notify_keyboard_keycode(
            10,
            if pressed {
                KeyState::Released.into()
            } else {
                KeyState::Pressed.into()
            },
        );

        std::thread::sleep(std::time::Duration::from_nanos(100));
    }
}
