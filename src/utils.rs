pub fn calculate_tile_x(index_x: u8, width: u8, tile_size: f32) -> f32 {
    (index_x as f32 * tile_size) - ((tile_size / 2.) * (width - 1) as f32)
}

pub fn calculate_tile_y(index_y: u8, height: u8, tile_size: f32) -> f32 {
    (index_y as f32 * tile_size) - ((tile_size / 2.) * (height - 1) as f32)
}
