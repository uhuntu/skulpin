use skulpin_renderer::ash;
pub use glfw;

pub use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;
use ash::vk::Handle;

use skulpin_renderer::PhysicalSize;
use skulpin_renderer::LogicalSize;
use skulpin_renderer::Window;

use std::ptr;

#[cfg(target_os = "windows")]
const DEFAULT_DPI: f32 = 96.0;

pub struct GlfwWindow<'a> {
    window: &'a glfw::Window,
}

impl<'a> GlfwWindow<'a> {
    pub fn new(window: &'a glfw::Window) -> Self {
        GlfwWindow { window }
    }

    #[cfg(target_os = "windows")]
    fn compute_scale_factor(&self) -> Option<f64> {
        Some((DEFAULT_DPI / 1.0).into())
    }
}

impl<'a> Window for GlfwWindow<'a> {
    fn physical_size(&self) -> PhysicalSize {
        let physical_size = self.window.get_size();
        PhysicalSize::new(physical_size.0 as u32, physical_size.1 as u32)
    }

    #[cfg(target_os = "windows")]
    fn logical_size(&self) -> LogicalSize {
        let physical_size = self.physical_size();
        physical_size.to_logical(self.scale_factor())
    }

    #[cfg(not(target_os = "windows"))]
    fn logical_size(&self) -> LogicalSize {
        let logical_size = self.window.size();
        LogicalSize::new(logical_size.0, logical_size.1)
    }

    #[cfg(target_os = "windows")]
    fn scale_factor(&self) -> f64 {
        self.compute_scale_factor().unwrap_or(1.0)
    }

    #[cfg(not(target_os = "windows"))]
    fn scale_factor(&self) -> f64 {
        let physical_size = self.window.vulkan_drawable_size();
        let logical_size = self.window.size();
        logical_size.0 as f64 / physical_size.0 as f64
    }

    fn create_vulkan_surface(
        &self,
        _entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> Result<vk::SurfaceKHR, vk::Result> {
        let mut surface_pointer: u64 = 0;

        self.window.create_window_surface(
            instance.handle().as_raw() as usize,
            ptr::null(),
            &mut surface_pointer,
        );

        Ok(vk::SurfaceKHR::from_raw(surface_pointer as u64))
    }

    fn extension_names(&self) -> Vec<*const i8> {
        let vec_string = self.window.glfw.get_required_instance_extensions().unwrap();

        let vec_string2: Vec<String> = vec_string
            .into_iter()
            .map(|mut e| {
                e.push('\0');
                e
            })
            .collect();

        let vec_str: Vec<&str> = vec_string2.iter().map(|s| s as &str).collect();

        println!("vec_str = {:?}", vec_str);

        let str_vec = ["VK_KHR_surface\0", "VK_KHR_win32_surface\0"].to_vec();

        println!("str_vec = {:?}", str_vec);

        let matching = vec_str
            .iter()
            .zip(&str_vec)
            .filter(|&(a, b)| a == b)
            .count();

        println!("{}", matching);

        vec_str
            .into_iter()
            .map(|extension| extension.as_ptr() as *const i8)
            .collect()
    }
}
