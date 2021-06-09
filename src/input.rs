use crate::types::vector2::Vector2;
use glutin::dpi::PhysicalPosition;
use glutin::event::{ElementState, KeyboardInput};

/// Input manager
pub struct Input {
    current_keys_down: Vec<u32>,
    key_down_buffer: u32,
    key_up_buffer: u32,
    current_mouse_buttons_down: Vec<u32>,
    mouse_button_down_buffer: u32,
    mouse_button_up_buffer: u32,
    mouse_position: Vector2,
    is_mouse_entered: bool,
}

/// Keyboard keys to check for input
pub enum Key {
    Esc = 1,
    Num1 = 2,
    Num2 = 3,
    Num3 = 4,
    Num4 = 5,
    Num5 = 6,
    Num6 = 7,
    Num7 = 8,
    Num8 = 9,
    Num9 = 10,
    Num0 = 11,
    Minus = 12,
    Plus = 13,
    Backspace = 14,
    Tab = 15,
    Q = 16,
    W = 17,
    E = 18,
    R = 19,
    T = 20,
    Y = 21,
    U = 22,
    I = 23,
    O = 24,
    P = 25,
    LeftBracket = 26,
    RightBracket = 27,
    Enter = 28,
    LeftCtrl = 29,
    A = 30,
    S = 31,
    D = 32,
    F = 33,
    G = 34,
    H = 35,
    J = 36,
    K = 37,
    L = 38,
    Semicolon = 39,
    Apostrophe = 40,
    GraveAccent = 41,
    LeftShift = 42,
    BackSlash = 43,
    Z = 44,
    X = 45,
    C = 46,
    V = 47,
    B = 48,
    N = 49,
    M = 50,
    Comma = 51,
    Period = 52,
    ForwardSlash = 53,
    RightShift = 54,
    LeftAlt = 56,
    Space = 57,
    CapsLock = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    F11 = 87,
    F12 = 88,
    RightCtrl = 57373,
    RightAlt = 57400,
    Up = 57416,
    Left = 57419,
    Right = 57421,
    Down = 57424,
    Insert = 57426,
    Delete = 57427,
    Meta = 57435,
}

/// Different mouse buttons
pub enum MouseButton {
    Left = 1,
    Right = 2,
    Middle = 3,
}

impl Input {
    /// Default constructor to initialize Input
    pub fn new() -> Self {
        return Self {
            current_keys_down: Vec::new(),
            key_down_buffer: 0,
            key_up_buffer: 0,
            current_mouse_buttons_down: Vec::new(),
            mouse_button_down_buffer: 0,
            mouse_button_up_buffer: 0,
            mouse_position: Vector2::new(),
            is_mouse_entered: false,
        };
    }

    /// Return whether a key is being held down
    pub fn is_key_down(&mut self, key: Key) -> bool {
        return self.current_keys_down.contains(&(key as u32));
    }
    /// Return whether a key is NOT being held down
    pub fn is_key_up(&mut self, key: Key) -> bool {
        return !self.current_keys_down.contains(&(key as u32));
    }

    /// Return true for the first frame a key is pressed
    pub fn on_key_down(&mut self, key: Key) -> bool {
        if self.key_down_buffer == key as u32 {
            self.key_down_buffer = 0;
            return true;
        }
        return false;
    }
    /// Return true for the first frame a key is released
    pub fn on_key_up(&mut self, key: Key) -> bool {
        if self.key_up_buffer == key as u32 {
            self.key_up_buffer = 0;
            return true;
        }
        return false;
    }

    /// Return value of horizontal input axis
    pub fn get_axis_horizontal(&mut self) -> f32 {
        let wasd: f32 =
            -(self.is_key_down(Key::A) as i32 as f32) + (self.is_key_down(Key::D) as i32 as f32);
        let arrows: f32 = -(self.is_key_down(Key::Left) as i32 as f32)
            + (self.is_key_down(Key::Right) as i32 as f32);
        return (wasd + arrows).clamp(-1.0, 1.0);
    }
    /// Return value of vertical input axis
    pub fn get_axis_vertical(&mut self) -> f32 {
        let wasd: f32 =
            (self.is_key_down(Key::S) as i32 as f32) - (self.is_key_down(Key::W) as i32 as f32);
        let arrows: f32 =
            (self.is_key_down(Key::Down) as i32 as f32) - (self.is_key_down(Key::Up) as i32 as f32);
        return (wasd + arrows).clamp(-1.0, 1.0);
    }

    /// Return whether a mouse button is being held down
    pub fn is_mouse_button_down(&mut self, mouse_button: MouseButton) -> bool {
        return self
            .current_mouse_buttons_down
            .contains(&(mouse_button as u32));
    }
    /// Return whether a mouse button is NOT being held down
    pub fn is_mouse_button_up(&mut self, mouse_button: MouseButton) -> bool {
        return !self
            .current_mouse_buttons_down
            .contains(&(mouse_button as u32));
    }

    /// Return true for the first frame a mouse button is pressed
    pub fn on_mouse_button_down(&mut self, mouse_button: MouseButton) -> bool {
        if self.mouse_button_down_buffer == mouse_button as u32 {
            self.mouse_button_down_buffer = 0;
            return true;
        }
        return false;
    }
    /// Return true for the first frame a mouse button is released
    pub fn on_mouse_button_up(&mut self, mouse_button: MouseButton) -> bool {
        if self.mouse_button_up_buffer == mouse_button as u32 {
            self.mouse_button_up_buffer = 0;
            return true;
        }
        return false;
    }

    /// Return position of mouse cursor
    pub fn mouse_position(&mut self) -> Vector2 {
        return self.mouse_position;
    }

    /// Return whether mouse cursor is in game window
    pub fn is_mouse_entered(&mut self) -> bool {
        return self.is_mouse_entered;
    }
    /// Return whether mouse cursor is NOT in game window
    pub fn is_mouse_exitted(&mut self) -> bool {
        return !self.is_mouse_entered;
    }

    /// Update keyboard input. Only meant to be called internally within sidekick
    pub fn update_keyboard_input(&mut self, input: KeyboardInput) {
        if input.state == ElementState::Pressed {
            if !self.current_keys_down.contains(&input.scancode) {
                self.current_keys_down.push(input.scancode);
            }
            self.key_down_buffer = input.scancode;
        } else {
            self.current_keys_down
                .retain(|code| *code != input.scancode);
            self.key_up_buffer = input.scancode;
        };
    }

    /// Update mouse button input. Only meant to be called internally within sidekick
    pub fn update_mouse_button_input(
        &mut self,
        state: ElementState,
        button: glutin::event::MouseButton,
    ) {
        if state == ElementState::Pressed {
            match button {
                glutin::event::MouseButton::Left => {
                    self.current_mouse_buttons_down
                        .push(MouseButton::Left as u32);
                    self.mouse_button_down_buffer = MouseButton::Left as u32;
                }
                glutin::event::MouseButton::Right => {
                    self.current_mouse_buttons_down
                        .push(MouseButton::Right as u32);
                    self.mouse_button_down_buffer = MouseButton::Right as u32;
                }
                glutin::event::MouseButton::Middle => {
                    self.current_mouse_buttons_down
                        .push(MouseButton::Middle as u32);
                    self.mouse_button_down_buffer = MouseButton::Middle as u32;
                }
                _ => {}
            }
        } else {
            match button {
                glutin::event::MouseButton::Left => {
                    self.current_mouse_buttons_down
                        .retain(|button| *button != MouseButton::Left as u32);
                    self.mouse_button_up_buffer = MouseButton::Left as u32;
                }
                glutin::event::MouseButton::Right => {
                    self.current_mouse_buttons_down
                        .retain(|button| *button != MouseButton::Right as u32);
                    self.mouse_button_up_buffer = MouseButton::Right as u32;
                }
                glutin::event::MouseButton::Middle => {
                    self.current_mouse_buttons_down
                        .retain(|button| *button != MouseButton::Middle as u32);
                    self.mouse_button_up_buffer = MouseButton::Middle as u32;
                }
                _ => {}
            }
        };
    }

    /// Update mouse position input. Only meant to be called internally within sidekick
    pub fn update_mouse_position_input(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = Vector2 {
            x: position.x as f32,
            y: position.y as f32,
        };
    }

    /// Update mouse entered/exit input. Only meant to be called internally within sidekick
    pub fn update_mouse_entered_input(&mut self, is_mouse_entered: bool) {
        self.is_mouse_entered = is_mouse_entered;
    }
}
