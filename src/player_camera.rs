use glam::{Vec3, Mat4};

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,   // Rotation around Y axis (left/right)
    pub pitch: f32, // Rotation around X axis (up/down)
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Vec3, aspect_ratio: f32) -> Self {
        Self {
            position,
            yaw: 0.0,
            pitch: 0.0,
            fov: 70.0_f32.to_radians(),
            aspect_ratio,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Get the forward direction vector based on yaw and pitch
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize()
    }

    /// Get the right direction vector
    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    /// Get the up direction vector
    pub fn up(&self) -> Vec3 {
        self.right().cross(self.forward()).normalize()
    }

    /// Get the view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.position,
            self.position + self.forward(),
            Vec3::Y,
        )
    }

    /// Get the projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    /// Rotate the camera based on mouse movement
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        // Clamp pitch to prevent camera flipping
        self.pitch = self.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
    }

    /// Update aspect ratio (useful for window resizing)
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
}
