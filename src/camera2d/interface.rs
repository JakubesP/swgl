use nalgebra_glm as glm;
use super::super::global_tools::vector2::Vector2;

pub trait CameraType {
    fn matrix(&self) -> glm::Mat4;
    fn dedicated_model(&self) -> glm::Mat4;
    fn map_pixel_coords_to_game_coords(&self, coords: &Vector2<f32>) -> Vector2<f32>; 
}