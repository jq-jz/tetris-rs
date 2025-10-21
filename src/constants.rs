pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub const CELL_SIZE: f32 = 30.0;

pub fn grid_to_world(x: i32, y: i32) -> (f32, f32) {
    let offset_x = -(GRID_WIDTH as f32) * CELL_SIZE / 2.0;
    let offset_y = GRID_HEIGHT as f32 * CELL_SIZE / 2.0;
    let world_x = offset_x + x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
    let world_y = offset_y - y as f32 * CELL_SIZE - CELL_SIZE / 2.0;
    (world_x, world_y)
}
