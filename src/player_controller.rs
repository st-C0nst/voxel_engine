use crate::player_camera::Camera;
use glam::Vec3;

/// Represents the state of keyboard inputs for player movement
#[derive(Default, Clone, Copy)]
pub struct KeyState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub sprint: bool,
}

/// Player controller for fly mode in voxel game
pub struct Player {
    pub camera: Camera,
    pub position: Vec3,
    pub velocity: Vec3,
    pub speed: f32,
    pub sprint_multiplier: f32,
    pub mouse_sensitivity: f32,
    pub acceleration: f32,
    pub friction: f32,
}

impl Player {
    /// Create a new player at the given position
    pub fn new(position: Vec3, aspect_ratio: f32) -> Self {
        Self {
            camera: Camera::new(position, aspect_ratio),
            position,
            velocity: Vec3::ZERO,
            speed: 10.0,              // Base movement speed
            sprint_multiplier: 2.0,   // Sprint speed multiplier
            mouse_sensitivity: 0.002, // Mouse sensitivity for camera rotation
            acceleration: 50.0,       // How quickly player accelerates
            friction: 10.0,           // How quickly player decelerates
        }
    }

    /// Update player state based on input and delta time
    /// delta_time should be in seconds (e.g., 0.016 for 60 FPS)
    pub fn update(&mut self, key_state: KeyState, delta_time: f32) {
        // Calculate movement direction based on input
        let mut movement_dir = Vec3::ZERO;

        if key_state.forward {
            movement_dir += self.camera.forward();
        }
        if key_state.backward {
            movement_dir -= self.camera.forward();
        }
        if key_state.right {
            movement_dir += self.camera.right();
        }
        if key_state.left {
            movement_dir -= self.camera.right();
        }
        if key_state.up {
            movement_dir += Vec3::Y;
        }
        if key_state.down {
            movement_dir -= Vec3::Y;
        }

        // Normalize movement direction to prevent faster diagonal movement
        if movement_dir.length_squared() > 0.0 {
            movement_dir = movement_dir.normalize();
        }

        // Calculate target velocity
        let current_speed = if key_state.sprint {
            self.speed * self.sprint_multiplier
        } else {
            self.speed
        };

        let target_velocity = movement_dir * current_speed;

        // Apply acceleration/friction for smooth movement
        if movement_dir.length_squared() > 0.0 {
            // Accelerate towards target velocity
            self.velocity = self.velocity.lerp(target_velocity, self.acceleration * delta_time);
        } else {
            // Apply friction when no input
            self.velocity = self.velocity.lerp(Vec3::ZERO, self.friction * delta_time);
        }

        // Update position based on velocity
        self.position += self.velocity * delta_time;
        self.camera.position = self.position;
    }

    /// Handle mouse movement for camera rotation
    /// delta_x and delta_y are the mouse movement in pixels
    pub fn handle_mouse_movement(&mut self, delta_x: f32, delta_y: f32) {
        let yaw_delta = delta_x * self.mouse_sensitivity;
        let pitch_delta = -delta_y * self.mouse_sensitivity; // Inverted Y axis
        
        self.camera.rotate(yaw_delta, pitch_delta);
    }

    /// Teleport player to a specific position
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.camera.position = position;
        self.velocity = Vec3::ZERO;
    }

    /// Set the camera's aspect ratio (call when window is resized)
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.camera.set_aspect_ratio(aspect_ratio);
    }

    /// Get the view-projection matrix for rendering
    pub fn view_projection_matrix(&self) -> glam::Mat4 {
        self.camera.projection_matrix() * self.camera.view_matrix()
    }
}
