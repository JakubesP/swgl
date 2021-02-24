
pub fn convert_2d_to_1d(x: u64, y: u64, width: u64) -> u64 {
    x + y * width 
}

pub fn convert_1d_to_2d(index: u64, width: u64) -> (u64, u64) {
    (index % width, index / width)
}