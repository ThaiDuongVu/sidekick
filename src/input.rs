// Input manager
pub struct Input {
    pub current_key_down: u32,
    pub key_down_buffer: u32,
    pub key_up_buffer: u32,
}

// Keyboard keys to check for input
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

impl Input {
    // Default constructor to initialize Input
    pub fn new() -> Self {
        Self {
            current_key_down: 0,
            key_down_buffer: 0,
            key_up_buffer: 0,
        }
    }

    // Return whether a key is being held down
    pub fn is_key_down(&mut self, key: Key) -> bool {
        return self.current_key_down == (key as u32);
    }
    // Return whether a key is not being held down
    pub fn is_key_up(&mut self, key: Key) -> bool {
        return self.current_key_down != (key as u32);
    }

    // Return true for the first frame a key is pressed
    pub fn on_key_down(&mut self, key: Key) -> bool {
        if self.key_down_buffer == key as u32 {
            self.key_down_buffer = 0;
            return true;
        }
        return false;
    }
    // Return true for the first frame a key is released
    pub fn on_key_up(&mut self, key: Key) -> bool {
        if self.key_up_buffer == key as u32 {
            self.key_up_buffer = 0;
            return true;
        }
        return false;
    }
}
