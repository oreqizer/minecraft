use winit::event::{DeviceEvent, WindowEvent};

use crate::game::input::Input;

pub fn from_window(we: WindowEvent) -> Option<Input> {
    match we {
        WindowEvent::KeyboardInput { input, .. } => Some(Input::Key {
            code: input.scancode,
            virtual_keycode: input.virtual_keycode,
            state: input.state,
        }),
        WindowEvent::MouseInput { button, state, .. } => Some(Input::MouseButton { button, state }),
        WindowEvent::ReceivedCharacter(c) => Some(Input::Char(c)),
        _ => None,
    }
}

pub fn from_device(de: DeviceEvent) -> Option<Input> {
    match de {
        DeviceEvent::MouseMotion { delta: (x, y) } => Some(Input::MouseDelta(x as i32, y as i32)),
        _ => None,
    }
}
