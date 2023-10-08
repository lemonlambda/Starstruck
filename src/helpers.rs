pub fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    variant << 29 | major << 22 | minor << 12 | patch
}
pub fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    major << 22 | minor << 12 | patch
}
