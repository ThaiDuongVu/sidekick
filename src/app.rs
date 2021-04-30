extern crate glfw;
use glfw::Context;

// Default values for window initialization
const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_TITLE: &str = "Sidekick App";

pub struct App {
    width: u32,
    height: u32,
    title: String,
}

impl App {
    // Default constructor to initialize App
    pub fn new() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),
        }
    }

    // Set App screen width
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Set App screen height
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    // Set App screen title
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    // Run App
    pub fn run(&mut self, init: Option<fn(&mut App)>) {
        // User-defined initialization
        if let Some(init) = init {
            init(self);
        }
        // Initialize glfw and create a window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut glfw_window, _events) = glfw
            .create_window(
                self.width,
                self.height,
                &self.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        glfw_window.set_key_polling(true);
        glfw_window.make_current();
        // Main program loop
        while !glfw_window.should_close() {
            glfw.poll_events();
        }
    }
}
