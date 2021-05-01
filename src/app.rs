extern crate glutin;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

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
    pub fn new<'a>() -> Self {
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

    // Set App screen width and height
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    // Set App screen title
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    // Run App
    pub fn run<'a>(
        mut self,
        init: Option<fn(&mut App)>,
        update: Option<fn(&mut App)>,
        render: Option<fn(&mut App)>,
        exit: Option<fn(&mut App)>,
    ) {
        // User-defined initialization
        if let Some(init) = init {
            init(&mut self);
        }

        // Create event loop and window builder
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(glutin::dpi::PhysicalSize::new(self.width, self.height));

        // Create context for current window
        let windowed_context = ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        // Main program loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            // User-defined update
            if let Some(update) = update {
                update(&mut self);
            }
            // Poll for events in main loop
            match event {
                Event::LoopDestroyed => {
                    // User-defined exit
                    if let Some(exit) = exit {
                        exit(&mut self);
                    }
                    return;
                }
                Event::WindowEvent { event, .. } => match event {
                    // WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    // User-defined render
                    if let Some(render) = render {
                        render(&mut self);
                    }
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
