extern crate glutin;

use glutin::dpi::PhysicalSize;
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
    is_resizable: bool,
    is_visible: bool,
    is_minimized: bool,
    is_maximized: bool,
    is_decorated: bool,
    current_context: Option<glutin::WindowedContext<glutin::PossiblyCurrent>>,
}

impl App {
    // Default constructor to initialize App
    pub fn new() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),
            is_resizable: false,
            is_visible: true,
            is_minimized: false,
            is_maximized: false,
            is_decorated: true,
            current_context: None,
        }
    }

    // Set App screen width
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }
    // Return current App screen width
    pub fn width(&mut self) -> u32 {
        return self.width;
    }

    // Set App screen height
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }
    // Return current App screen height
    pub fn height(&mut self) -> u32 {
        return self.height;
    }

    // Set App screen width and height
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }

    // Set App screen title
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_title(&self.title);
    }
    // Return App screen title
    pub fn title(&mut self) -> &str {
        return &self.title;
    }

    // Set whether App is resizable
    pub fn set_resizable(&mut self, is_resizable: bool) {
        self.is_resizable = is_resizable;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_resizable(self.is_resizable);
    }
    // Return whether App is resizable
    pub fn is_resizable(&mut self) -> bool {
        return self.is_resizable;
    }

    // Set whether App is visible
    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_visible(self.is_visible);
    }
    // Return whether App is visible
    pub fn is_visible(&mut self) -> bool {
        return self.is_visible;
    }

    // Set whether App is minimized
    pub fn set_minimized(&mut self, is_minimized: bool) {
        self.is_minimized = is_minimized;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_minimized(self.is_minimized);
    }
    // Return whether App is minimized
    pub fn is_minimized(&mut self) -> bool {
        return self.is_minimized;
    }

    // Set whether App is maximized
    pub fn set_maximized(&mut self, is_maximized: bool) {
        self.is_minimized = is_maximized;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_minimized(self.is_maximized);
    }
    // Return whether App is maximized
    pub fn is_maximized(&mut self) -> bool {
        return self.is_maximized;
    }

    // Set whether App is decorated
    pub fn set_decorated(&mut self, is_decorated: bool) {
        self.is_decorated = is_decorated;
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_decorations(self.is_decorated);
    }
    // Return whether App is decorated
    pub fn is_decorated(&mut self) -> bool {
        return self.is_decorated;
    }

    // Run App
    pub fn run(
        mut self,
        init: Option<fn(&mut App)>,
        update: Option<fn(&mut App)>,
        render: Option<fn(&mut App)>,
        exit: Option<fn(&mut App)>,
    ) {
        // Create event loop for window context
        let event_loop = EventLoop::new();
        // Create and set a new window context
        self.current_context = Some(unsafe {
            ContextBuilder::new()
                .build_windowed(
                    WindowBuilder::new()
                        .with_title(&self.title)
                        .with_inner_size(glutin::dpi::PhysicalSize::new(self.width, self.height))
                        .with_resizable(self.is_resizable)
                        .with_visible(self.is_visible)
                        .with_decorations(self.is_decorated),
                    &event_loop,
                )
                .unwrap()
                .make_current()
                .unwrap()
        });

        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_minimized(self.is_minimized);
        self.current_context
            .as_ref()
            .unwrap()
            .window()
            .set_maximized(self.is_maximized);

        // User-defined initialization
        if let Some(init) = init {
            init(&mut self);
        }

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
                    // WindowEvent::Resized(physical_size) => new_context.resize(physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    // User-defined render
                    if let Some(render) = render {
                        render(&mut self);
                    }
                    self.current_context
                        .as_ref()
                        .unwrap()
                        .swap_buffers()
                        .unwrap();
                }
                _ => (),
            }
        });
    }
}
