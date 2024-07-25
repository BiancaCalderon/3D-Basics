use crate::player::Player;

pub fn cast_ray(
    maze: &Vec<Vec<char>>,
    player: &Player,
    buffer: &mut Vec<u32>,
    window_width: usize,
    window_height: usize,
    cell_size: usize,
) {
    let mut x = player.x;
    let mut y = player.y;
    let mut distance = 0.0;

    loop {
        let cell_x = (x / cell_size as f32).floor() as usize;
        let cell_y = (y / cell_size as f32).floor() as usize;

        if cell_x >= maze[0].len() || cell_y >= maze.len() || maze[cell_y][cell_x] == '+' || maze[cell_y][cell_x] == '|' {
            break;
        }

        let pixel_x = x as usize;
        let pixel_y = y as usize;

        if pixel_x < window_width && pixel_y < window_height {
            buffer[pixel_y * window_width + pixel_x] = 0xFF00FF; // Color del rayo
        }

        x += player.angle.cos();
        y += player.angle.sin();
        distance += 1.0;
    }
}