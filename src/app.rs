use crate::input::Input;
use crate::time::Time;
use glutin::dpi::PhysicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{CursorIcon, UserAttentionType, WindowBuilder};
use glutin::ContextBuilder;

use std::borrow::Cow;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState, SubpassContents};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{ApplicationInfo, Instance, PhysicalDevice, Version};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use vulkano_win::VkSurfaceBuild;

// Types of attention to request user
pub enum AttentionType {
    Critical,
    Informational,
}

// Style of mouse cursor
pub enum MouseIcon {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EastResize,
    NorthResize,
    NorthEastResize,
    NorthWestResize,
    SouthResize,
    SouthEastResize,
    SouthWestResize,
    WestResize,
    EastWestResize,
    NorthSouthResize,
    NorthEastSouthWestResize,
    NorthWestSouthEastResize,
    ColumnResize,
    RowResize,
}

// Default values for window initialization
const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_TITLE: &str = "Sidekick App";
const DEFAULT_RESIZABILITY: bool = false;
const DEFAULT_VISIBILITY: bool = true;
const DEFAULT_MINIMIZATION: bool = false;
const DEFAULT_MAXIMIZATION: bool = false;
const DEFAULT_DECORATION: bool = true;
const DEFAULT_ALWAYS_ON_TOP: bool = false;
const DEFAULT_CURSOR_CONFINEMENT: bool = false;
const DEFAULT_CURSOR_VISIBILITY: bool = true;
const DEFAULT_FOCUS: bool = true;

// Main game App, everything is wrapped in here
pub struct App {
    width: u32,
    height: u32,
    title: String,
    is_resizable: bool,
    is_visible: bool,
    is_minimized: bool,
    is_maximized: bool,
    is_decorated: bool,
    is_always_on_top: bool,
    is_mouse_confined: bool,
    is_mouse_visible: bool,
    mouse_icon: MouseIcon,
    is_focused: bool,
    surface: Option<std::sync::Arc<vulkano::swapchain::Surface<glutin::window::Window>>>,
    control_flow: Option<*mut ControlFlow>,
    pub input: Input,
    pub time: Time,
}

#[allow(deprecated)]
impl App {
    // Default constructor to initialize App
    pub fn new() -> Self {
        return Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            title: String::from(DEFAULT_TITLE),
            is_resizable: DEFAULT_RESIZABILITY,
            is_visible: DEFAULT_VISIBILITY,
            is_minimized: DEFAULT_MINIMIZATION,
            is_maximized: DEFAULT_MAXIMIZATION,
            is_decorated: DEFAULT_DECORATION,
            is_always_on_top: DEFAULT_ALWAYS_ON_TOP,
            is_mouse_confined: DEFAULT_CURSOR_CONFINEMENT,
            is_mouse_visible: DEFAULT_CURSOR_VISIBILITY,
            mouse_icon: MouseIcon::Default,
            is_focused: DEFAULT_FOCUS,
            surface: None,
            control_flow: None,
            input: Input::new(),
            time: Time::new(),
        };
    }

    // Set App screen width
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }
    // Return current App screen width
    pub fn width(&self) -> u32 {
        return self.width;
    }

    // Set App screen height
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }
    // Return current App screen height
    pub fn height(&self) -> u32 {
        return self.height;
    }

    // Set App screen width and height
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }

    // Set App screen title
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_title(&self.title);
    }
    // Return App screen title
    pub fn title(&self) -> &str {
        return &self.title;
    }

    // Set whether App is resizable
    pub fn set_resizable(&mut self, is_resizable: bool) {
        self.is_resizable = is_resizable;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_resizable(self.is_resizable);
    }
    // Return whether App is resizable
    pub fn is_resizable(&self) -> bool {
        return self.is_resizable;
    }

    // Set whether App is visible
    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_visible(self.is_visible);
    }
    // Return whether App is visible
    pub fn is_visible(&self) -> bool {
        return self.is_visible;
    }

    // Set whether App is minimized
    pub fn set_minimized(&mut self, is_minimized: bool) {
        self.is_minimized = is_minimized;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_minimized(self.is_minimized);
    }
    // Return whether App is minimized
    pub fn is_minimized(&self) -> bool {
        return self.is_minimized;
    }

    // Set whether App is maximized
    pub fn set_maximized(&mut self, is_maximized: bool) {
        self.is_minimized = is_maximized;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_minimized(self.is_maximized);
    }
    // Return whether App is maximized
    pub fn is_maximized(&self) -> bool {
        return self.is_maximized;
    }

    // Set whether App is decorated
    pub fn set_decorated(&mut self, is_decorated: bool) {
        self.is_decorated = is_decorated;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_decorations(self.is_decorated);
    }
    // Return whether App is decorated
    pub fn is_decorated(&self) -> bool {
        return self.is_decorated;
    }

    // Set whether App is always on top
    pub fn set_always_on_top(&mut self, is_always_on_top: bool) {
        self.is_always_on_top = is_always_on_top;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_always_on_top(self.is_always_on_top);
    }
    // Return whether App is always on top
    pub fn is_always_on_top(&self) -> bool {
        return self.is_always_on_top;
    }

    // Set whether mouse cursor is confined within window bound
    pub fn set_mouse_confined(&mut self, is_mouse_confined: bool) {
        self.is_mouse_confined = is_mouse_confined;
        match self
            .surface
            .as_ref()
            .unwrap()
            .window()
            .set_cursor_grab(self.is_mouse_confined)
        {
            Ok(_) => {}
            Err(err) => {
                println!("Error when setting mouse cursor confinement: {}", err)
            }
        }
    }
    // Return whether mouse cursor is confined within window bound
    pub fn is_mouse_confined(&self) -> bool {
        return self.is_mouse_confined;
    }

    // Set whether mouse cursor is visible
    pub fn set_mouse_visible(&mut self, is_mouse_visible: bool) {
        self.is_mouse_visible = is_mouse_visible;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_cursor_visible(self.is_mouse_visible);
    }
    // Return whether mouse cursor is visible
    pub fn is_mouse_visible(&self) -> bool {
        return self.is_mouse_visible;
    }

    // Set mouse icon
    pub fn set_mouse_icon(&mut self, mouse_icon: MouseIcon) {
        let icon: CursorIcon = match mouse_icon {
            MouseIcon::Default => CursorIcon::Default,
            MouseIcon::Crosshair => CursorIcon::Crosshair,
            MouseIcon::Hand => CursorIcon::Hand,
            MouseIcon::Arrow => CursorIcon::Arrow,
            MouseIcon::Move => CursorIcon::Move,
            MouseIcon::Text => CursorIcon::Text,
            MouseIcon::Wait => CursorIcon::Wait,
            MouseIcon::Help => CursorIcon::Help,
            MouseIcon::Progress => CursorIcon::Progress,
            MouseIcon::NotAllowed => CursorIcon::NotAllowed,
            MouseIcon::ContextMenu => CursorIcon::ContextMenu,
            MouseIcon::Cell => CursorIcon::Cell,
            MouseIcon::VerticalText => CursorIcon::VerticalText,
            MouseIcon::Alias => CursorIcon::Alias,
            MouseIcon::Copy => CursorIcon::Copy,
            MouseIcon::NoDrop => CursorIcon::NoDrop,
            MouseIcon::Grab => CursorIcon::Grab,
            MouseIcon::Grabbing => CursorIcon::Grabbing,
            MouseIcon::AllScroll => CursorIcon::AllScroll,
            MouseIcon::ZoomIn => CursorIcon::ZoomIn,
            MouseIcon::ZoomOut => CursorIcon::ZoomOut,
            MouseIcon::EastResize => CursorIcon::EResize,
            MouseIcon::NorthResize => CursorIcon::NResize,
            MouseIcon::NorthEastResize => CursorIcon::NeResize,
            MouseIcon::NorthWestResize => CursorIcon::NwResize,
            MouseIcon::SouthResize => CursorIcon::SResize,
            MouseIcon::SouthEastResize => CursorIcon::SeResize,
            MouseIcon::SouthWestResize => CursorIcon::SwResize,
            MouseIcon::WestResize => CursorIcon::WResize,
            MouseIcon::EastWestResize => CursorIcon::EwResize,
            MouseIcon::NorthSouthResize => CursorIcon::NsResize,
            MouseIcon::NorthEastSouthWestResize => CursorIcon::NeswResize,
            MouseIcon::NorthWestSouthEastResize => CursorIcon::NwseResize,
            MouseIcon::ColumnResize => CursorIcon::ColResize,
            MouseIcon::RowResize => CursorIcon::RowResize,
        };
        self.mouse_icon = mouse_icon;

        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_cursor_icon(icon);
    }
    // Return current mouse icon
    pub fn mouse_icon(self) -> MouseIcon {
        return self.mouse_icon;
    }

    // Return whether App is focused
    pub fn is_focused(&self) -> bool {
        return self.is_focused;
    }

    // App request for user attention
    pub fn request_attention(&mut self, attention_type: AttentionType) {
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .request_user_attention(match attention_type {
                AttentionType::Critical => Some(UserAttentionType::Critical),
                AttentionType::Informational => Some(UserAttentionType::Informational),
            });
    }

    // Quit App
    pub fn quit(&mut self) {
        if let Some(control_flow) = self.control_flow {
            unsafe { *control_flow = ControlFlow::Exit }
        }
    }

    // Run App
    pub fn run<I, U, R, E>(
        mut self,
        init: Option<I>,
        update: Option<U>,
        render: Option<R>,
        exit: Option<E>,
    ) where
        I: Fn(&mut App) + 'static,
        U: Fn(&mut App) + 'static,
        R: Fn(&mut App) + 'static,
        E: Fn(&mut App) + 'static,
    {
        // Create a Vulkan instance
        // Get list of required extensions
        let required_extensions = vulkano_win::required_extensions();
        // General application info
        let app_info = ApplicationInfo {
            application_name: Some(Cow::Borrowed(self.title())),
            application_version: Some(Version::from_vulkan_version(100)),
            engine_name: Some(Cow::Borrowed("Sidekick")),
            engine_version: Some(Version::from_vulkan_version(010)),
        };
        // Create instance
        let instance = Instance::new(Some(&app_info), &required_extensions, None).unwrap();
        // First physical device to draw on
        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

        // Create event loop for window context
        let event_loop = EventLoop::new();
        // Contains both window and Vulkan surface
        self.surface = Some(
            WindowBuilder::new()
                .with_title(&self.title)
                .with_inner_size(glutin::dpi::PhysicalSize::new(self.width, self.height))
                .with_resizable(self.is_resizable)
                .with_visible(self.is_visible)
                .with_decorations(self.is_decorated)
                .with_maximized(self.is_maximized)
                .build_vk_surface(&event_loop, instance.clone())
                .unwrap(),
        );

        // User-defined initialization
        if let Some(init) = init {
            init(&mut self);
        }

        // Main program loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            self.control_flow = Some(control_flow);

            // User-defined update
            if let Some(update) = &update {
                update(&mut self);
            }

            // Poll for events in main loop
            match event {
                Event::LoopDestroyed => {
                    // User-defined exit
                    if let Some(exit) = &exit {
                        exit(&mut self);
                    }
                    return;
                }
                Event::WindowEvent { event, .. } => match event {
                    // Handle keyboard input
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    } => self.input.update_keyboard_input(input),

                    // Handle mouse button input
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button,
                        modifiers: _,
                    } => self.input.update_mouse_button_input(state, button),

                    // Handle mouse position input
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        modifiers: _,
                    } => self.input.update_mouse_position_input(position),
                    // Handle mouse enter/exit input
                    WindowEvent::CursorEntered { device_id: _ } => {
                        self.input.update_mouse_entered_input(true)
                    }
                    WindowEvent::CursorLeft { device_id: _ } => {
                        self.input.update_mouse_entered_input(false)
                    }

                    WindowEvent::Focused(is_focused) => self.is_focused = is_focused,
                    WindowEvent::CloseRequested => self.quit(),
                    _ => (),
                },
                Event::RedrawEventsCleared => {
                    // Update frame time before next update iteration
                    self.time.update();
                }
                Event::RedrawRequested(_) => {
                    // User-defined render
                    if let Some(render) = &render {
                        render(&mut self);
                    }
                }
                _ => (),
            }
        });
    }
}
