use super::super::super::gl_wrapper::vertex_array_object;
use super::super::super::global_tools::vector2::Vector2;

pub trait VertexType {
    fn get_vao_signature() -> Vec<vertex_array_object::VertexBufferAttribute>;
    fn get_row_width() -> u32;
    fn to_vec(&self) -> Vec<f32>;
    fn get_position(&self) -> Vector2<f32>;
    fn set_position(&mut self, position: &Vector2<f32>);
}

pub trait DedicatedShader {
    fn get_dedicated_shader() -> &'static str;
} 

pub trait TextureCoords {
    fn get_tex_coords(&self) -> Vector2<f32>;
    fn set_tex_coords(&mut self, coords: &Vector2<f32>);  
}