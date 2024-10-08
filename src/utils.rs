pub fn remap_rand_u32(val: u32, min: u32, max: u32) -> u32 {
    ((val as f32 / u32::MAX as f32) * (max - min) as f32) as u32 + min
}

pub fn remap_rand_f32(val: u32, min: f32, max: f32) -> f32 {
    ((val as f32 / u32::MAX as f32) * (max - min) as f32) as f32 + min
}
