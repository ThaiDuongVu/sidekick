use crate::entities::game_view::GameView;
use crate::input::Input;
use crate::rendering::vertex::Vertex;
use crate::time::Time;

use std::borrow::Cow;
use std::sync::Arc;

use glutin::dpi::PhysicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{CursorIcon, UserAttentionType, Window, WindowBuilder};

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

/// Types of attention to request user
pub enum AttentionType {
    Critical,
    Informational,
}

/// Style of mouse cursor
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

/// Main game App, everything is wrapped in here
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
    pub game_view: GameView,
}

#[allow(deprecated)]
impl App {
    /// Default constructor to initialize App
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
            game_view: GameView::new(),
        };
    }

    /// Set App screen width
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }
    /// Return current App screen width
    pub fn width(&self) -> u32 {
        return self.width;
    }

    /// Set App screen height
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }
    /// Return current App screen height
    pub fn height(&self) -> u32 {
        return self.height;
    }

    /// Set App screen width and height
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_inner_size(PhysicalSize::new(self.width, self.height));
    }

    /// Set App screen title
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_title(&self.title);
    }
    /// Return App screen title
    pub fn title(&self) -> &str {
        return &self.title;
    }

    /// Set whether App is resizable
    pub fn set_resizable(&mut self, is_resizable: bool) {
        self.is_resizable = is_resizable;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_resizable(self.is_resizable);
    }
    /// Return whether App is resizable
    pub fn is_resizable(&self) -> bool {
        return self.is_resizable;
    }

    /// Set whether App is visible
    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_visible(self.is_visible);
    }
    /// Return whether App is visible
    pub fn is_visible(&self) -> bool {
        return self.is_visible;
    }

    /// Set whether App is minimized
    pub fn set_minimized(&mut self, is_minimized: bool) {
        self.is_minimized = is_minimized;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_minimized(self.is_minimized);
    }
    /// Return whether App is minimized
    pub fn is_minimized(&self) -> bool {
        return self.is_minimized;
    }

    /// Set whether App is maximized
    pub fn set_maximized(&mut self, is_maximized: bool) {
        self.is_minimized = is_maximized;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_minimized(self.is_maximized);
    }
    /// Return whether App is maximized
    pub fn is_maximized(&self) -> bool {
        return self.is_maximized;
    }

    /// Set whether App is decorated
    pub fn set_decorated(&mut self, is_decorated: bool) {
        self.is_decorated = is_decorated;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_decorations(self.is_decorated);
    }
    /// Return whether App is decorated
    pub fn is_decorated(&self) -> bool {
        return self.is_decorated;
    }

    /// Set whether App is always on top
    pub fn set_always_on_top(&mut self, is_always_on_top: bool) {
        self.is_always_on_top = is_always_on_top;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_always_on_top(self.is_always_on_top);
    }
    /// Return whether App is always on top
    pub fn is_always_on_top(&self) -> bool {
        return self.is_always_on_top;
    }

    /// Set whether mouse cursor is confined within window bound
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
    /// Return whether mouse cursor is confined within window bound
    pub fn is_mouse_confined(&self) -> bool {
        return self.is_mouse_confined;
    }

    /// Set whether mouse cursor is visible
    pub fn set_mouse_visible(&mut self, is_mouse_visible: bool) {
        self.is_mouse_visible = is_mouse_visible;
        self.surface
            .as_ref()
            .unwrap()
            .window()
            .set_cursor_visible(self.is_mouse_visible);
    }
    /// Return whether mouse cursor is visible
    pub fn is_mouse_visible(&self) -> bool {
        return self.is_mouse_visible;
    }

    /// Set mouse icon
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
    /// Return current mouse icon
    pub fn mouse_icon(self) -> MouseIcon {
        return self.mouse_icon;
    }

    /// Return whether App is focused
    pub fn is_focused(&self) -> bool {
        return self.is_focused;
    }

    /// App request for user attention
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

    /// Quit App
    pub fn quit(&mut self) {
        if let Some(control_flow) = self.control_flow {
            unsafe { *control_flow = ControlFlow::Exit }
        }
    }

    /// Run App
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
        // Create an instance
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
        // Contains both window and surface information
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
        // Queue of GPUs to execute draw commands
        let queue_family = physical
            .queue_families()
            .find(|&q| {
                // We take the first queue that supports drawing to our window.
                q.supports_graphics()
                    && self
                        .surface
                        .as_ref()
                        .unwrap()
                        .is_supported(q)
                        .unwrap_or(false)
            })
            .unwrap();

        // Create and initialize device
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_extensions,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();
        // Retrieve first queue from queues iterator
        let queue = queues.next().unwrap();
        // Create a swapchain to allocates color buffers
        // Get the images that will be rendered as well
        let (mut swapchain, images) = {
            // Querying the capabilities of the surface
            let capabilities = self
                .surface
                .as_ref()
                .unwrap()
                .capabilities(physical)
                .unwrap();

            // The alpha mode indicates how the alpha value of the final image will behave
            let alpha = capabilities
                .supported_composite_alpha
                .iter()
                .next()
                .unwrap();
            // Choosing the internal format that the images will have.
            let format = capabilities.supported_formats[0].0;

            // The dimensions of the window, only used to initially setup the swapchain.
            let dimensions: [u32; 2] = self.surface.as_ref().unwrap().window().inner_size().into();

            // Create swapchain
            Swapchain::new(
                device.clone(),
                self.surface.as_ref().unwrap().clone(),
                capabilities.min_image_count,
                format,
                dimensions,
                1,
                ImageUsage::color_attachment(),
                &queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            )
            .unwrap()
        };
        // Vertex buffer to store triangle points
        let vertex_buffer = {
            vulkano::impl_vertex!(Vertex, positions);
            CpuAccessibleBuffer::from_iter(
                device.clone(),
                BufferUsage::all(),
                false,
                [
                    Vertex {
                        positions: [-0.5, -0.25],
                    },
                    Vertex {
                        positions: [0.0, 0.5],
                    },
                    Vertex {
                        positions: [0.25, -0.1],
                    },
                ]
                .iter()
                .cloned(),
            )
            .unwrap()
        };
        // Vertex shader to render triangle
        // TODO: Extract this into its own file
        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                src: "
                    #version 450
                    layout(location = 0) in vec2 positions;
                    void main() {
                        gl_Position = vec4(positions, 0.0, 1.0);
                    }
                "
            }
        }
        // Fragment shader to render triangle
        // TODO: Extract this into its own file
        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                src: "
                    #version 450
                    layout(location = 0) out vec4 f_color;
                    void main() {
                        f_color = vec4(1.0, 0.0, 0.0, 1.0);
                    }
                "
            }
        }

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        // Create a render pass, which  describes where the output of the graphics pipeline will go
        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(
                device.clone(),
                attachments: {
                    color: {
                        // Clear the content of this attachment at the start of the drawing
                        load: Clear,
                        // Store the output of the draw in the actual image
                        store: Store,
                        // Format indicates the type of the format of the image. Here we use the same format as the swapchain.
                        format: swapchain.format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    // No depth-stencil attachment is indicated with empty brackets
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );
        // Create a render pipeline, similar to an OpenGL program, but more specific
        // Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<dyn PipelineLayoutAbstract + Send + Sync>, Arc<RenderPass<RenderPassDesc>>,>,>
        let pipeline = Arc::new(
            GraphicsPipeline::start()
                // Indicate the layout of the vertices
                .vertex_input_single_buffer()
                // Specify entry point for vertex shader
                .vertex_shader(vs.main_entry_point(), ())
                // The content of the vertex buffer describes a list of triangles.
                .triangle_list()
                // Use a resizable viewport set to draw over the entire window
                .viewports_dynamic_scissors_irrelevant(1)
                // Specify entry point for fragment shader
                .fragment_shader(fs.main_entry_point(), ())
                // Indicate which subpass of which render pass this pipeline is going to be used in. The pipeline will only be usable from this particular subpass.
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );
        // Dynamic viewports allow us to recreate just the viewport when the window is resized
        // Otherwise we would have to recreate the whole pipeline.
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };
        // Create frame buffers to draw multiple images
        let mut frame_buffers =
            window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
        // Recreate a new swapchain in case the current one gets invalid
        let mut recreate_swapchain = true;
        // The submission of the previous frame
        let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

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
                    WindowEvent::Resized(_) => recreate_swapchain = true,
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

                    // Polls various fences in order to determine what the GPU has already processed, and frees the resources that are no longer needed
                    previous_frame_end.as_mut().unwrap().cleanup_finished();
                    // Whenever the window resizes we need to recreate everything dependent on the window size
                    if recreate_swapchain {
                        // Get the new dimensions of the window
                        let dimensions: [u32; 2] =
                            self.surface.as_ref().unwrap().window().inner_size().into();
                        let (new_swapchain, new_images) =
                            match swapchain.recreate_with_dimensions(dimensions) {
                                Ok(r) => r,
                                Err(SwapchainCreationError::UnsupportedDimensions) => return,
                                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                            };

                        swapchain = new_swapchain;
                        // Because framebuffers contains an Arc on the old swapchain, we need to recreate framebuffers as well
                        frame_buffers = window_size_dependent_setup(
                            &new_images,
                            render_pass.clone(),
                            &mut dynamic_state,
                        );
                        recreate_swapchain = false;

                        // Acquire an image from the swapchain. If no image is available, then the function will block
                        let (image_num, suboptimal, acquire_future) =
                            match swapchain::acquire_next_image(swapchain.clone(), None) {
                                Ok(r) => r,
                                Err(AcquireError::OutOfDate) => {
                                    recreate_swapchain = true;
                                    return;
                                }
                                Err(e) => panic!("Failed to acquire next image: {:?}", e),
                            };
                        if suboptimal {
                            recreate_swapchain = true;
                        }

                        // Specify the color to clear the framebuffer with i.e. blue
                        let clear_values = vec![[
                            self.game_view.color.r,
                            self.game_view.color.g,
                            self.game_view.color.b,
                            self.game_view.color.a,
                        ]
                        .into()];
                        // Build a command buffer
                        let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
                            device.clone(),
                            queue.family(),
                        )
                        .unwrap();

                        builder
                            // Enter the render pass
                            .begin_render_pass(
                                frame_buffers[image_num].clone(),
                                SubpassContents::Inline,
                                clear_values,
                            )
                            .unwrap()
                            .draw(
                                pipeline.clone(),
                                &dynamic_state,
                                vertex_buffer.clone(),
                                (),
                                (),
                            )
                            .unwrap()
                            // Exit the render pass
                            .end_render_pass()
                            .unwrap();

                        // Finish building the command buffer
                        let command_buffer = builder.build().unwrap();

                        let future = previous_frame_end
                            .take()
                            .unwrap()
                            .join(acquire_future)
                            .then_execute(queue.clone(), command_buffer)
                            .unwrap()
                            // Submits a present command at the end of the queue
                            .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                            .then_signal_fence_and_flush();

                        match future {
                            Ok(future) => {
                                previous_frame_end = Some(future.boxed());
                            }
                            Err(FlushError::OutOfDate) => {
                                recreate_swapchain = true;
                                previous_frame_end = Some(sync::now(device.clone()).boxed());
                            }
                            Err(e) => {
                                println!("Failed to flush future: {:?}", e);
                                previous_frame_end = Some(sync::now(device.clone()).boxed());
                            }
                        }
                    }
                }
                _ => (),
            }
        });
    }
}

/// Called when the window is resized
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}
