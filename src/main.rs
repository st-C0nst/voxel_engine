use voxel_engine::{Player, KeyState};
use glam::Vec3;

fn main() {
    // Example usage of the player controller
    let mut player = Player::new(Vec3::new(0.0, 50.0, 0.0), 16.0 / 9.0);
    
    println!("Player initialized at position: {:?}", player.position);
    println!("Camera forward direction: {:?}", player.camera.forward());
    
    // Example: Simulate one frame of movement
    let mut key_state = KeyState::default();
    key_state.forward = true;
    key_state.up = true;
    
    let delta_time = 1.0 / 60.0; // 60 FPS
    player.update(key_state, delta_time);
    
    println!("Player position after update: {:?}", player.position);
    println!("Player velocity: {:?}", player.velocity);
}
