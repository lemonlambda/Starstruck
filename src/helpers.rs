pub fn rgba(r: u32, g: u32, b: u32, a: u32) -> (f32, f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
}
