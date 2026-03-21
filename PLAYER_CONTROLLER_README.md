# Voxel Game Fly Mode Player Controller

A complete fly mode player controller implementation for your voxel game, featuring smooth movement, camera rotation, and frame-rate independent physics.

## Features

- **Fly Mode Movement**: Move freely in all 6 directions (forward, backward, left, right, up, down)
- **Sprint Mode**: Hold sprint key for faster movement
- **Smooth Camera Rotation**: Mouse-based first-person camera control with pitch clamping
- **Frame-Rate Independent**: Uses delta time for consistent movement across different frame rates
- **Smooth Acceleration/Deceleration**: Realistic movement with acceleration and friction
- **Diagonal Movement Normalization**: Prevents faster movement when moving diagonally

## Files

- [`player_camera.rs`](voxel_engine/src/player_camera.rs) - Camera implementation with view/projection matrices
- [`player_controller.rs`](voxel_engine/src/player_controller.rs) - Player controller with movement logic

## Usage

### Basic Setup

```rust
use voxel_engine::{Player, KeyState};
use glam::Vec3;

// Create a player at spawn position with 16:9 aspect ratio
let mut player = Player::new(Vec3::new(0.0, 50.0, 0.0), 16.0 / 9.0);
```

### Game Loop Integration

```rust
// In your game loop:
let delta_time = /* time since last frame in seconds */;

// 1. Gather input
let key_state = KeyState {
    forward: is_key_pressed(Key::W),
    backward: is_key_pressed(Key::S),
    left: is_key_pressed(Key::A),
    right: is_key_pressed(Key::D),
    up: is_key_pressed(Key::Space),
    down: is_key_pressed(Key::LShift),
    sprint: is_key_pressed(Key::LControl),
};

// 2. Update player
player.update(key_state, delta_time);

// 3. Handle mouse movement
let (mouse_dx, mouse_dy) = get_mouse_delta();
player.handle_mouse_movement(mouse_dx, mouse_dy);

// 4. Get view-projection matrix for rendering
let view_proj = player.view_projection_matrix();
```

### KeyState Structure

```rust
pub struct KeyState {
    pub forward: bool,   // W key
    pub backward: bool,  // S key
    pub left: bool,      // A key
    pub right: bool,     // D key
    pub up: bool,        // Space key
    pub down: bool,      // Shift key
    pub sprint: bool,    // Control key
}
```

## Configuration

You can customize the player's behavior by modifying these fields:

```rust
player.speed = 15.0;              // Base movement speed (default: 10.0)
player.sprint_multiplier = 3.0;   // Sprint speed multiplier (default: 2.0)
player.mouse_sensitivity = 0.003; // Mouse sensitivity (default: 0.002)
player.acceleration = 60.0;       // Acceleration rate (default: 50.0)
player.friction = 12.0;           // Friction/deceleration (default: 10.0)
```

## Camera Properties

The camera supports:
- **FOV**: Field of view (default: 70°)
- **Aspect Ratio**: Automatically adjustable for window resizing
- **Near/Far Planes**: Configurable clipping planes (default: 0.1 - 1000.0)
- **Pitch Clamping**: Prevents camera from flipping (±89°)

### Adjusting Camera

```rust
// Change FOV
player.camera.fov = 90.0_f32.to_radians();

// Update aspect ratio on window resize
player.set_aspect_ratio(new_width / new_height);

// Adjust clipping planes
player.camera.near = 0.01;
player.camera.far = 2000.0;
```

## Methods

### Player Methods

- [`Player::new(position, aspect_ratio)`](voxel_engine/src/player_controller.rs:29) - Create a new player
- [`player.update(key_state, delta_time)`](voxel_engine/src/player_controller.rs:43) - Update player state
- [`player.handle_mouse_movement(dx, dy)`](voxel_engine/src/player_controller.rs:96) - Handle mouse input
- [`player.set_position(position)`](voxel_engine/src/player_controller.rs:103) - Teleport player
- [`player.set_aspect_ratio(ratio)`](voxel_engine/src/player_controller.rs:110) - Update aspect ratio
- [`player.view_projection_matrix()`](voxel_engine/src/player_controller.rs:115) - Get VP matrix for rendering

### Camera Methods

- [`camera.forward()`](voxel_engine/src/player_camera.rs:25) - Get forward direction vector
- [`camera.right()`](voxel_engine/src/player_camera.rs:34) - Get right direction vector
- [`camera.up()`](voxel_engine/src/player_camera.rs:39) - Get up direction vector
- [`camera.view_matrix()`](voxel_engine/src/player_camera.rs:44) - Get view matrix
- [`camera.projection_matrix()`](voxel_engine/src/player_camera.rs:53) - Get projection matrix
- [`camera.rotate(yaw, pitch)`](voxel_engine/src/player_camera.rs:58) - Rotate camera

## Example

Run the example to see the controller in action:

```bash
cargo run --example player_example
```

## Integration with Rendering

To use the player controller with your rendering system:

```rust
// Get the view-projection matrix
let vp_matrix = player.view_projection_matrix();

// Pass to your shader uniform
shader.set_uniform("u_viewProjection", vp_matrix);

// Or get individual matrices
let view = player.camera.view_matrix();
let projection = player.camera.projection_matrix();
```

## Physics Notes

- **Delta Time**: Always pass delta time in seconds (e.g., `1.0 / 60.0` for 60 FPS)
- **Acceleration**: Higher values = snappier movement
- **Friction**: Higher values = quicker stops
- **Velocity**: Automatically managed by the controller
- **Normalization**: Diagonal movement is automatically normalized to prevent speed exploits

## Tips

1. **Lock Mouse Cursor**: For best experience, lock and hide the mouse cursor during gameplay
2. **Delta Time**: Use a proper timer to calculate accurate delta time
3. **Input Buffering**: Consider buffering input to handle multiple keys pressed simultaneously
4. **Collision**: Add collision detection by checking `player.position` before applying movement
5. **Smooth Camera**: The built-in acceleration provides smooth camera movement

## Future Enhancements

Potential additions you might want to implement:
- Ground collision and gravity mode
- Walking/running animation states
- Head bobbing effect
- Crouching/prone positions
- Velocity-based FOV changes (speed lines effect)
- Inertia for more realistic physics
