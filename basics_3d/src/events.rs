use minifb::Key;
use crate::player::Player;

pub const MOVE_SPEED: f32 = 10.0;
pub const ROTATION_SPEED: f32 = std::f32::consts::PI / 10.0;

pub fn process_events(window: &minifb::Window, player: &mut Player) {
    if window.is_key_down(Key::W) {
        player.x += player.angle.cos() * MOVE_SPEED;
        player.y += player.angle.sin() * MOVE_SPEED;
    }
    if window.is_key_down(Key::S) {
        player.x -= player.angle.cos() * MOVE_SPEED;
        player.y -= player.angle.sin() * MOVE_SPEED;
    }
    if window.is_key_down(Key::A) {
        player.angle -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.angle += ROTATION_SPEED;
    }
}
