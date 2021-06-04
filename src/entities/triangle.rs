use crate::entities::game_object::GameObject;
use crate::rendering::vertex::Vertex;
use crate::types::color::Color;

use std::borrow::Cow;
use std::sync::Arc;

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

/// A triangle that can be rendered on screen
pub struct Triangle {
    pub game_object: GameObject,
    pub color: Color,

    is_init: bool,
}

impl Triangle {
    // Default constructor to initialize triangle
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),
            color: Color::white(),

            is_init: false,
        };
    }

    /// Initialize triangle
    pub fn init(&mut self) {
        self.is_init = true;
    }

    /// Render triangle on screen
    pub fn draw(&mut self) {
        if !self.is_init {
            self.init();
        }
    }
}
