pub struct Font {
    pub path: String,
    pub width: u32,
    pub height: u32,
}

impl Font {
    // Default constructor to initialize Font
    pub fn new() -> Self {
        return Self {
            path: String::from("./resources/default_font.ttf"),
            width: 0,
            height: 48,
        };
    }
}
