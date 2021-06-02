//! # Input
//!
//! Module describing user input in the game.

use winit::event::{ElementState, MouseButton, VirtualKeyCode};

#[derive(Debug)]
pub enum Input {
    Char(char),
    Key {
        code: u32,
        virtual_keycode: Option<VirtualKeyCode>,
        state: ElementState,
    },
    MouseButton {
        button: MouseButton,
        state: ElementState,
    },
    MouseDelta(i32, i32),
}

pub struct Controls {
    // keys
    pub forward: bool, // W
    pub back: bool,    // S
    pub left: bool,    // A
    pub right: bool,   // D
    pub up: bool,      // Space
    pub down: bool,    // Shift
    // mouse
    pub mb_left: bool,
    pub mb_right: bool,
    pub mouse_position: (i32, i32),
    pub mouse_delta: (i32, i32),
}

impl Controls {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            // keys
            forward: false,
            back: false,
            left: false,
            right: false,
            up: false,
            down: false,
            // mouse
            mb_left: false,
            mb_right: false,
            mouse_position: (0, 0),
            mouse_delta: (0, 0),
        }
    }
}
