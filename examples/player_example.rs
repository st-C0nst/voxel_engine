use voxel_engine::player_controller::{Player, KeyState};
use glam::Vec3;

fn main() {
    // Create a player at spawn position
    let mut player = Player::new(Vec3::new(0.0, 50.0, 0.0), 16.0 / 9.0);
    
    println!("=== Voxel Game Fly Mode Player Controller Example ===\n");
    println!("Initial player position: {:?}", player.position);
    println!("Initial camera forward: {:?}\n", player.camera.forward());
    
    // Simulate 60 FPS
    let delta_time = 1.0 / 60.0;
    
    // Example 1: Move forward
    println!("--- Moving forward for 1 second ---");
    let mut key_state = KeyState {
        forward: true,
        ..Default::default()
    };
    
    for _ in 0..60 {
        player.update(key_state, delta_time);
    }
    println!("Position after moving forward: {:?}", player.position);
    println!("Velocity: {:?}\n", player.velocity);
    
    // Example 2: Move up and right with sprint
    println!("--- Moving up and right with sprint for 0.5 seconds ---");
    key_state = KeyState {
        up: true,
        right: true,
        sprint: true,
        ..Default::default()
    };
    
    for _ in 0..30 {
        player.update(key_state, delta_time);
    }
    println!("Position after moving up/right: {:?}", player.position);
    println!("Velocity: {:?}\n", player.velocity);
    
    // Example 3: Stop moving (friction applies)
    println!("--- Stopping (no input) for 0.5 seconds ---");
    key_state = KeyState::default();
    
    for _ in 0..30 {
        player.update(key_state, delta_time);
    }
    println!("Position after stopping: {:?}", player.position);
    println!("Velocity after friction: {:?}\n", player.velocity);
    
    // Example 4: Mouse look
    println!("--- Rotating camera with mouse ---");
    player.handle_mouse_movement(100.0, -50.0); // Simulate mouse drag
    println!("Camera forward after rotation: {:?}", player.camera.forward());
    println!("Camera yaw: {:.2}°", player.camera.yaw.to_degrees());
    println!("Camera pitch: {:.2}°\n", player.camera.pitch.to_degrees());
    
    // Example 5: Teleport
    println!("--- Teleporting to new position ---");
    player.set_position(Vec3::new(100.0, 75.0, 100.0));
    println!("New position: {:?}", player.position);
    println!("Velocity reset: {:?}\n", player.velocity);
    
    println!("=== Example Complete ===");
}
