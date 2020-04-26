use crate::wm::dpi::{LogicalSize, PhysicalSize};

/// Resource for managing the window size
///
/// Any requests to resize the window and/or to get the size and the hidpi factor value are
/// recommended to go through this resource instead of interacting with the [`Window`] directly.
/// This resource is kept in sync by [`WindowSizeControl`] system.
///
/// [`Window`]: ../winit/struct.Window.html
/// [`WindowSizeControl`]: ./fn.create_window_size_control_system.html
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSize {
    size: PhysicalSize,
    aspect_ratio: f32,
    dpi_factor: f64,
    requested_size: Option<PhysicalSize>,
}

impl WindowSize {
    /// Create a new instance from the given size and hidpi factor
    pub(crate) fn new(size: impl Into<PhysicalSize>, dpi_factor: f64) -> Self {
        let size = size.into();
        let aspect_ratio = (size.width / size.height) as f32;

        Self {
            size,
            aspect_ratio,
            dpi_factor,
            requested_size: None,
        }
    }

    /// Get the physical size of the window from the last update
    pub fn physical_size(&self) -> PhysicalSize {
        self.size
    }

    /// Get the logical size of the window from the last update
    pub fn logical_size(&self) -> LogicalSize {
        self.size.to_logical(self.dpi_factor)
    }

    /// Get the aspect ratio of the window from the last update
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// Get the hidpi factor of the window from the last update
    pub fn dpi_factor(&self) -> f64 {
        self.dpi_factor
    }

    /// If a new size of the window is currently requested
    pub fn is_new_size_requested(&self) -> bool {
        self.requested_size.is_some()
    }

    /// Get the currently requested physical size of the window
    pub fn current_requested_physical_size(&self) -> Option<PhysicalSize> {
        self.requested_size
    }

    /// Get the currently requested logical size of the window
    pub fn current_requested_logical_size(&self) -> Option<LogicalSize> {
        self.requested_size.map(|p| p.to_logical(self.dpi_factor))
    }

    /// Request a new window size in physical pixels
    pub fn request_new_physical_size(&mut self, requested_size: impl Into<PhysicalSize>) {
        self.requested_size.replace(requested_size.into());
    }

    /// Request a new window size in logical pixels
    pub fn request_new_logical_size(&mut self, requested_size: impl Into<LogicalSize>) {
        self.requested_size
            .replace(requested_size.into().to_physical(self.dpi_factor));
    }

    /// Request a new window size in physical pixels
    pub fn request_new_physical_size_values(&mut self, width: f64, height: f64) {
        self.request_new_physical_size((width, height));
    }

    /// Request a new window size in logical pixels
    pub fn request_new_logical_size_values(&mut self, width: f64, height: f64) {
        self.request_new_logical_size((width, height));
    }

    /// Clear out the currently requested new window size to cancel the request
    pub fn cancel_new_size_request(&mut self) {
        self.requested_size.take();
    }

    /// Set the size value from logical size
    pub(crate) fn set_logical_size(&mut self, size: LogicalSize) {
        self.size = size.to_physical(self.dpi_factor);
        self.aspect_ratio = (self.size.width / self.size.height) as f32;
    }

    /// Set the hidpi factor
    pub(crate) fn set_dpi_factor(&mut self, dpi_factor: f64) {
        self.dpi_factor = dpi_factor;
    }

    /// Get the value of the requested window size while also clearing out the value
    pub(crate) fn take_requested_size(&mut self) -> Option<PhysicalSize> {
        self.requested_size.take()
    }
}
